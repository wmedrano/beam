// Licensed to the Apache Software Foundation (ASF) under one or more
// contributor license agreements. See the NOTICE file distributed with
// this work for additional information regarding copyright ownership.
// The ASF licenses this file to You under the Apache License, Version 2.0
// (the "License"); you may not use this file except in compliance with
// the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::graph::{ASSERT_CHECK, ASSERT_TAG, CREATE, DOFN, SUM};
use crate::metrics::{Catalog, Metrics};
use crate::registry::{Processor, Registry};
use crate::{Result, error};
use beam_coders::{Coder, Context, Error as DecodeError, Limits, Value, WindowedValue};
use beam_model as model;
use protobuf::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use tempfile::NamedTempFile;
use tokio::sync::{Notify, Semaphore, mpsc, oneshot};
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tonic::Request;

type Packet = (model::Elements, Option<oneshot::Sender<()>>);
type DataSender = mpsc::Sender<Packet>;
type Port = (String, String);
struct Spool {
    file: NamedTempFile,
    bytes: u64,
    closed: bool,
}
#[derive(Default)]
struct Hub {
    inputs: Mutex<HashMap<Port, Spool>>,
    failure: Mutex<Option<String>>,
    notify: Notify,
    senders: tokio::sync::Mutex<HashMap<String, DataSender>>,
}

fn request<T>(value: T, worker: &str) -> Result<Request<T>> {
    let mut request = Request::new(value);
    request.metadata_mut().insert("worker_id", worker.parse()?);
    Ok(request)
}

fn endpoint(endpoint: model::ApiServiceDescriptorView<'_>) -> Result<String> {
    if !endpoint.authentication().urn().is_empty() {
        return Err(error("unsupported endpoint authentication scheme"));
    }
    Ok(endpoint.url().to_str()?.to_owned())
}

impl Hub {
    async fn connect(self: &Arc<Self>, url: &str, worker: &str) -> Result<DataSender> {
        let mut senders = self.senders.lock().await;
        if let Some(sender) = senders.get(url) {
            return Ok(sender.clone());
        }
        let mut client =
            model::beam_fn_data_client::BeamFnDataClient::new(crate::job::connect(url).await?)
                .with_max_message_bytes(64 * 1024 * 1024);
        let (tx, rx) = mpsc::channel::<Packet>(8);
        let stream = ReceiverStream::new(rx).map(|(message, ack)| {
            if let Some(ack) = ack {
                let _ = ack.send(());
            }
            message
        });
        let mut responses = client.data(request(stream, worker)?).await?.into_inner();
        let hub = self.clone();
        tokio::spawn(async move {
            let result: Result<()> = async {
                while let Some(message) = responses.message().await? {
                    if !message.timers().is_empty() {
                        return Err(error("Rust timer data is unsupported"));
                    }
                    let mut inputs = hub.inputs.lock().unwrap();
                    for data in message.data().iter() {
                        if inputs.values().map(|spool| spool.bytes).sum::<u64>()
                            + data.data().len() as u64
                            > 1024 * 1024 * 1024
                        {
                            return Err(error("worker input spools exceed the 1 GiB disk budget"));
                        }
                        let key = (
                            data.instruction_id().to_string(),
                            data.transform_id().to_string(),
                        );
                        if inputs.len() >= 128 && !inputs.contains_key(&key) {
                            return Err(error("too many input streams"));
                        }
                        let spool = match inputs.entry(key) {
                            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
                            std::collections::hash_map::Entry::Vacant(entry) => {
                                entry.insert(Spool {
                                    file: NamedTempFile::new()?,
                                    bytes: 0,
                                    closed: false,
                                })
                            }
                        };
                        if spool.closed {
                            return Err(error("data after stream end marker"));
                        }
                        spool.bytes += data.data().len() as u64;
                        if spool.bytes > 1024 * 1024 * 1024 {
                            return Err(error("bundle input spool exceeds 1 GiB limit"));
                        }
                        spool.file.write_all(data.data())?;
                        spool.closed = data.is_last();
                    }
                    drop(inputs);
                    hub.notify.notify_waiters();
                }
                Err(error("Fn Data stream disconnected"))
            }
            .await;
            if let Err(error) = result {
                *hub.failure.lock().unwrap() = Some(error.to_string());
                hub.notify.notify_waiters();
            }
        });
        senders.insert(url.to_owned(), tx.clone());
        Ok(tx)
    }

    async fn input(&self, bundle: &str, port: &str) -> Result<File> {
        loop {
            let notified = self.notify.notified();
            if let Some(failure) = self.failure.lock().unwrap().as_ref() {
                return Err(error(failure));
            }
            {
                let inputs = self.inputs.lock().unwrap();
                if let Some(spool) = inputs.get(&(bundle.to_owned(), port.to_owned()))
                    && spool.closed
                {
                    let mut file = spool.file.reopen()?;
                    file.seek(SeekFrom::Start(0))?;
                    return Ok(file);
                }
            }
            notified.await;
        }
    }
    fn release(&self, bundle: &str) {
        self.inputs
            .lock()
            .unwrap()
            .retain(|(instruction, _), _| instruction != bundle);
    }
}

fn validate_descriptor(descriptor: &model::ProcessBundleDescriptor) -> Result<()> {
    if descriptor.transforms().len() > 1024
        || descriptor.coders().len() > 4096
        || descriptor.pcollections().len() > 4096
    {
        return Err(error("bundle descriptor component limit exceeded"));
    }
    let mut producers = BTreeMap::new();
    for (id, transform) in descriptor.transforms().iter() {
        if !transform.subtransforms().is_empty() {
            return Err(error(
                "composite transforms are not executable bundle operators",
            ));
        }
        for (_, output) in transform.outputs().iter() {
            if descriptor.pcollections().get(output).is_none() {
                return Err(error("transform output references a missing collection"));
            }
            if producers
                .insert(output.to_string(), id.to_string())
                .is_some()
            {
                return Err(error("multiple producers for one collection"));
            }
        }
    }
    let mut pending = BTreeMap::new();
    for (id, transform) in descriptor.transforms().iter() {
        let mut dependencies = std::collections::BTreeSet::new();
        for (_, input) in transform.inputs().iter() {
            let producer = producers
                .get(&input.to_string())
                .ok_or_else(|| error("missing bundle input producer"))?;
            dependencies.insert(producer.clone());
        }
        pending.insert(id.to_string(), dependencies);
    }
    while !pending.is_empty() {
        let ready: Vec<_> = pending
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(id, _)| id.clone())
            .collect();
        if ready.is_empty() {
            return Err(error("cycle in bundle descriptor"));
        }
        for id in ready {
            pending.remove(&id);
            for dependencies in pending.values_mut() {
                dependencies.remove(&id);
            }
        }
    }
    for (_, collection) in descriptor.pcollections().iter() {
        coder(descriptor, collection.coder_id().to_str()?, 64)?;
        let windows = descriptor
            .windowing_strategies()
            .get(collection.windowing_strategy_id())
            .ok_or_else(|| error("missing windowing strategy"))?;
        if windows.window_fn().urn() != "beam:window_fn:global_windows:v1"
            || windows.allowed_lateness() != 0
            || !windows.trigger().has_default()
        {
            return Err(error(
                "Rust workers currently support only global windows, default triggering and zero lateness",
            ));
        }
    }
    Ok(())
}

fn port_value(coder: &Coder) -> Result<&Coder> {
    match coder {
        Coder::Windowed(value, window) if **window == Coder::GlobalWindow => Ok(value),
        Coder::LengthPrefix(inner) => port_value(inner),
        _ => Err(error("runner port requires a global windowed-value coder")),
    }
}

pub(crate) async fn run(registry: Arc<Registry>, args: BTreeMap<String, String>) -> Result<()> {
    let worker = args
        .get("id")
        .ok_or_else(|| error("worker requires --id"))?
        .clone();
    let provision_url = args
        .get("provision_endpoint")
        .ok_or_else(|| error("worker requires --provision_endpoint"))?;
    let mut provision = model::provision_service_client::ProvisionServiceClient::new(
        crate::job::connect(provision_url).await?,
    );
    let info = provision
        .get_provision_info(request(model::GetProvisionInfoRequest::new(), &worker)?)
        .await?
        .into_inner();
    let control_url = endpoint(info.info().control_endpoint())?;
    let control_channel = crate::job::connect(&control_url).await?;
    let mut control =
        model::beam_fn_control_client::BeamFnControlClient::new(control_channel.clone());
    let (responses, rx) = mpsc::channel(32);
    let mut instructions = control
        .control(request(ReceiverStream::new(rx), &worker)?)
        .await?
        .into_inner();
    let hub = Arc::new(Hub::default());
    let cache = Arc::new(tokio::sync::Mutex::new(HashMap::<
        String,
        model::ProcessBundleDescriptor,
    >::new()));
    let compute = Arc::new(Semaphore::new(4));
    let short_ids = info.info().runner_capabilities().iter().any(|capability| {
        capability == model::urns::STANDARD_RUNNER_PROTOCOLS_ENUM_MONITORING_INFO_SHORT_IDS
    });
    let catalog = Arc::new(Catalog::default());
    let active = Arc::new(Mutex::new(HashMap::<String, Arc<Metrics>>::new()));
    let mut tasks = tokio::task::JoinSet::new();
    // Opening logging is part of boot. Its bounded channel stays alive until
    // control ends; stderr remains a useful fallback on transport failure.
    let _logging = logging(endpoint(info.info().logging_endpoint())?, &worker).await?;
    eprintln!("Rust worker {worker} connected");
    while let Some(instruction) = instructions.message().await? {
        while let Some(completed) = tasks.try_join_next() {
            completed.map_err(|failure| error(format!("bundle task failed: {failure}")))?;
        }
        let instruction_id = instruction.instruction_id().to_string();
        if instruction.has_process_bundle() {
            if tasks.len() >= 128 {
                return Err(error("too many concurrent bundle instructions"));
            }
            let id = instruction
                .process_bundle()
                .process_bundle_descriptor_id()
                .to_string();
            let metrics = Arc::new(Metrics::default());
            if active
                .lock()
                .unwrap()
                .insert(instruction_id.clone(), metrics.clone())
                .is_some()
            {
                return Err(error("duplicate active bundle instruction ID"));
            }
            let active = active.clone();
            let catalog = catalog.clone();
            let (hub, registry, cache, compute, responses, worker, channel) = (
                hub.clone(),
                registry.clone(),
                cache.clone(),
                compute.clone(),
                responses.clone(),
                worker.clone(),
                control_channel.clone(),
            );
            tasks.spawn(async move {
                let result = async {
                    let cached = cache.lock().await.get(&id).cloned();
                    let descriptor = if let Some(descriptor) = cached {
                        descriptor
                    } else {
                        let mut client =
                            model::beam_fn_control_client::BeamFnControlClient::new(channel);
                        let mut req = model::GetProcessBundleDescriptorRequest::new();
                        req.set_process_bundle_descriptor_id(&id);
                        let descriptor = client
                            .get_process_bundle_descriptor(request(req, &worker)?)
                            .await?
                            .into_inner();
                        let mut descriptors = cache.lock().await;
                        if descriptors.len() >= 128 && !descriptors.contains_key(&id) {
                            return Err(error("bundle descriptor cache limit exceeded"));
                        }
                        descriptors.insert(id, descriptor.clone());
                        descriptor
                    };
                    bundle(
                        registry,
                        descriptor,
                        &instruction_id,
                        &worker,
                        hub.clone(),
                        compute,
                        metrics.clone(),
                    )
                    .await
                }
                .await;
                hub.release(&instruction_id);
                active.lock().unwrap().remove(&instruction_id);
                let mut response = model::InstructionResponse::new();
                response.set_instruction_id(&instruction_id);
                match result {
                    Ok(()) => {
                        if short_ids {
                            match catalog.record(metrics.infos()) {
                                Ok(data) => {
                                    for (id, bytes) in data {
                                        response
                                            .process_bundle_mut()
                                            .monitoring_data_mut()
                                            .insert(id.as_str(), bytes);
                                    }
                                }
                                Err(error) => response.set_error(error.to_string()),
                            }
                        } else {
                            for info in metrics.infos() {
                                response
                                    .process_bundle_mut()
                                    .monitoring_infos_mut()
                                    .push(info);
                            }
                        }
                        response.process_bundle_mut();
                    }
                    Err(error) => {
                        eprintln!("worker={worker} bundle={instruction_id}: {error}");
                        response.set_error(error.to_string());
                    }
                }
                let _ = responses.send(response).await;
            });
        } else {
            let mut response = model::InstructionResponse::new();
            response.set_instruction_id(&instruction_id);
            if instruction.has_register() {
                for descriptor in instruction.register().process_bundle_descriptor().iter() {
                    cache
                        .lock()
                        .await
                        .insert(descriptor.id().to_string(), descriptor.to_owned());
                }
                response.register_mut();
            } else if instruction.has_process_bundle_progress() {
                response.process_bundle_progress_mut();
                if let Some(metrics) = active.lock().unwrap().get(
                    instruction
                        .process_bundle_progress()
                        .instruction_id()
                        .to_str()?,
                ) {
                    if short_ids {
                        for (id, bytes) in catalog.record(metrics.infos())? {
                            response
                                .process_bundle_progress_mut()
                                .monitoring_data_mut()
                                .insert(id.as_str(), bytes);
                        }
                    } else {
                        for info in metrics.infos() {
                            response
                                .process_bundle_progress_mut()
                                .monitoring_infos_mut()
                                .push(info);
                        }
                    }
                }
            } else if instruction.has_monitoring_infos() {
                response.monitoring_infos_mut();
                for id in instruction.monitoring_infos().monitoring_info_id().iter() {
                    if let Some(info) = catalog.get(id.to_str()?) {
                        response
                            .monitoring_infos_mut()
                            .monitoring_info_mut()
                            .insert(id, info);
                    } else {
                        response.set_error(format!("unknown monitoring ID {id}"));
                        break;
                    }
                }
            } else if instruction.has_process_bundle_split() {
                response.process_bundle_split_mut();
            } else {
                response.set_error("unsupported Rust harness instruction");
            }
            responses
                .send(response)
                .await
                .map_err(|_| error("control response stream disconnected"))?;
        }
        while tasks.try_join_next().is_some() {}
    }
    tasks.abort_all();
    Err(error(
        "Fn Control stream disconnected; active bundles are not replayed",
    ))
}

async fn logging(url: String, worker: &str) -> Result<mpsc::Sender<model::log_entry::List>> {
    let mut client =
        model::beam_fn_logging_client::BeamFnLoggingClient::new(crate::job::connect(&url).await?);
    let (tx, rx) = mpsc::channel(32);
    let stream = request(ReceiverStream::new(rx), worker)?;
    tokio::spawn(async move {
        match client.logging(stream).await {
            Ok(response) => {
                let mut responses = response.into_inner();
                while let Ok(Some(_)) = responses.message().await {}
            }
            Err(error) => eprintln!("Fn Logging transport failed: {error}"),
        }
    });
    let mut entry = model::LogEntry::new();
    entry.set_severity(model::log_entry::severity::Enum::Info);
    entry.set_message("Native Rust harness connected");
    let mut entries = model::log_entry::List::new();
    entries.log_entries_mut().push(entry);
    tx.send(entries).await?;
    Ok(tx)
}

pub(crate) fn coder(
    descriptor: &model::ProcessBundleDescriptor,
    id: &str,
    depth: usize,
) -> Result<Coder> {
    if depth == 0 {
        return Err(error("cyclic or excessively nested coder descriptor"));
    }
    let proto = descriptor
        .coders()
        .get(id)
        .ok_or_else(|| error(format!("missing coder {id}")))?;
    let components: Result<Vec<_>> = proto
        .component_coder_ids()
        .iter()
        .map(|id| coder(descriptor, id.to_str()?, depth - 1))
        .collect();
    let components = components?;
    let urn = proto.spec().urn();
    let name = urn
        .to_str()?
        .strip_prefix("beam:coder:")
        .and_then(|s| s.strip_suffix(":v1"))
        .ok_or_else(|| error("nonstandard coder URN"))?;
    let count = match name {
        "kv" | "windowed_value" => 2,
        "iterable" | "nullable" | "length_prefix" => 1,
        _ => 0,
    };
    if components.len() != count || !proto.spec().payload().is_empty() {
        return Err(error("invalid coder components/payload"));
    }
    Ok(match name {
        "bytes" => Coder::Bytes,
        "string_utf8" => Coder::Utf8,
        "varint" => Coder::VarInt,
        "bool" => Coder::Bool,
        "double" => Coder::Double,
        "global_window" => Coder::GlobalWindow,
        "interval_window" => Coder::IntervalWindow,
        "kv" => Coder::Kv(
            Box::new(components[0].clone()),
            Box::new(components[1].clone()),
        ),
        "windowed_value" => Coder::Windowed(
            Box::new(components[0].clone()),
            Box::new(components[1].clone()),
        ),
        "iterable" => Coder::Iterable(Box::new(components[0].clone())),
        "nullable" => Coder::Nullable(Box::new(components[0].clone())),
        "length_prefix" => Coder::LengthPrefix(Box::new(components[0].clone())),
        _ => return Err(error(format!("unsupported coder {urn}"))),
    })
}

fn collection_coder(
    descriptor: &model::ProcessBundleDescriptor,
    collection: &str,
) -> Result<Coder> {
    let value = descriptor
        .pcollections()
        .get(collection)
        .ok_or_else(|| error("missing PCollection"))?;
    coder(descriptor, value.coder_id().to_str()?, 64)
}

enum Operator {
    Flatten {
        out: String,
    },
    User {
        out: String,
        processor: Option<Box<dyn Processor>>,
        payload: model::RustFunctionPayload,
        input: Coder,
        output: Coder,
    },
    Builtin {
        out: String,
        id: String,
        config: Vec<u8>,
        input: Coder,
        output: Coder,
    },
    Sink {
        coder: Coder,
        sender: DataSender,
    },
}
struct Stage {
    bundle: String,
    consumers: HashMap<String, Vec<String>>,
    operators: BTreeMap<String, Operator>,
    metrics: Arc<Metrics>,
}
impl Stage {
    fn route(&mut self, collection: &str, value: &WindowedValue, depth: usize) -> Result<()> {
        if depth == 0 {
            return Err(error("cyclic or too deeply fused stage"));
        }
        let metrics = self.metrics.clone();
        metrics.element(collection);
        let consumers = self.consumers.get(collection).cloned().unwrap_or_default();
        for id in consumers {
            let mut operator = self
                .operators
                .remove(&id)
                .ok_or_else(|| error("stage contains a cycle"))?;
            let result = match &mut operator {
                Operator::Flatten { out } => self.route(out, value, depth - 1),
                Operator::User { out, processor, .. } => processor
                    .as_mut()
                    .ok_or_else(|| error("processor was not started"))?
                    .process(
                        &value.value,
                        &mut |output| {
                            self.route(
                                out,
                                &WindowedValue {
                                    value: output,
                                    timestamp: value.timestamp,
                                    windows: value.windows.clone(),
                                    pane: value.pane,
                                },
                                depth - 1,
                            )
                        },
                        &mut |name, delta| metrics.counter(&id, name, delta),
                    ),
                Operator::Builtin {
                    out,
                    id,
                    config,
                    input,
                    output,
                } => builtin(id, config, input, output, &value.value, &mut |output| {
                    self.route(
                        out,
                        &WindowedValue {
                            value: output,
                            timestamp: value.timestamp,
                            windows: value.windows.clone(),
                            pane: value.pane,
                        },
                        depth - 1,
                    )
                }),
                Operator::Sink { coder, sender } => {
                    let bytes =
                        coder.encode(&Value::Windowed(Box::new(value.clone())), Context::Nested)?;
                    if bytes.len() > Limits::default().bytes {
                        return Err(error("output element exceeds 64 MiB"));
                    }
                    for part in bytes.chunks(1024 * 1024) {
                        send_data(sender, &self.bundle, &id, part.to_vec(), false)?;
                    }
                    Ok(())
                }
            };
            let result = result.map_err(|failure| error(format!("transform={id}: {failure}")));
            self.operators.insert(id, operator);
            result?;
        }
        Ok(())
    }
}

fn send_data(
    sender: &DataSender,
    bundle: &str,
    port: &str,
    bytes: Vec<u8>,
    last: bool,
) -> Result<()> {
    let mut data = model::elements::Data::new();
    data.set_instruction_id(bundle);
    data.set_transform_id(port);
    data.set_data(bytes);
    data.set_is_last(last);
    let mut message = model::Elements::new();
    message.data_mut().push(data);
    let (ack, done) = oneshot::channel();
    sender
        .blocking_send((message, last.then_some(ack)))
        .map_err(|_| error("Fn Data output disconnected"))?;
    if last {
        done.blocking_recv()
            .map_err(|_| error("Fn Data output closed before end marker was consumed"))?;
    }
    Ok(())
}

fn builtin(
    id: &str,
    config: &[u8],
    input_coder: &Coder,
    output_coder: &Coder,
    input: &Value,
    emit: &mut dyn FnMut(Value) -> Result<()>,
) -> Result<()> {
    match id {
        CREATE => {
            if input_coder != &Coder::Bytes {
                return Err(error("Create requires an Impulse bytes input"));
            }
            let Value::Iterable(values) = Coder::Iterable(Box::new(output_coder.clone())).decode(
                config,
                Context::Outer,
                Limits::default(),
            )?
            else {
                unreachable!()
            };
            for value in values {
                emit(value)?;
            }
        }
        SUM => {
            let Value::Kv(key, values) = input else {
                return Err(error("SumI64 requires KV input"));
            };
            let Value::Iterable(values) = values.as_ref() else {
                return Err(error("SumI64 requires grouped values"));
            };
            let mut sum = 0_i64;
            for value in values {
                if let Value::I64(v) = value {
                    sum = sum.wrapping_add(*v);
                } else {
                    return Err(error("SumI64 requires i64 values"));
                }
            }
            emit(Value::Kv(key.clone(), Box::new(Value::I64(sum))))?;
        }
        ASSERT_TAG => {
            let bytes = input_coder.encode(input, Context::Nested)?;
            emit(Value::Kv(
                Box::new(Value::I64(0)),
                Box::new(Value::Kv(
                    Box::new(Value::Bool(true)),
                    Box::new(Value::Bytes(bytes)),
                )),
            ))?;
        }
        ASSERT_CHECK => {
            let Value::Kv(_, values) = input else {
                return Err(error("assertion requires KV"));
            };
            let Value::Iterable(values) = values.as_ref() else {
                return Err(error("assertion requires grouping"));
            };
            let mut actual = Vec::new();
            for value in values {
                let Value::Kv(tag, bytes) = value else {
                    return Err(error("invalid assertion tag"));
                };
                if tag.as_ref() == &Value::Bool(true) {
                    actual.push(bytes.as_ref().clone());
                }
            }
            actual.sort_by(|a, b| {
                let (Value::Bytes(a), Value::Bytes(b)) = (a, b) else {
                    return std::cmp::Ordering::Equal;
                };
                a.cmp(b)
            });
            let expected = Coder::Iterable(Box::new(Coder::Bytes)).decode(
                config,
                Context::Outer,
                Limits::default(),
            )?;
            if Value::Iterable(actual) != expected {
                return Err(error("distributed content/multiplicity assertion failed"));
            }
        }
        _ => return Err(error(format!("unknown Rust builtin {id}"))),
    }
    Ok(())
}

async fn bundle(
    registry: Arc<Registry>,
    descriptor: model::ProcessBundleDescriptor,
    bundle: &str,
    worker: &str,
    hub: Arc<Hub>,
    compute: Arc<Semaphore>,
    metrics: Arc<Metrics>,
) -> Result<()> {
    validate_descriptor(&descriptor)?;
    let mut stage = Stage {
        bundle: bundle.to_owned(),
        consumers: HashMap::new(),
        operators: BTreeMap::new(),
        metrics,
    };
    let mut sources = Vec::new();
    let transforms: Vec<_> = descriptor
        .transforms()
        .iter()
        .map(|(id, transform)| (id.to_string(), transform.to_owned()))
        .collect();
    for (id, transform) in transforms {
        let inputs: Vec<_> = transform
            .inputs()
            .iter()
            .map(|(_, value)| value.to_string())
            .collect();
        let outputs: Vec<_> = transform
            .outputs()
            .iter()
            .map(|(_, value)| value.to_string())
            .collect();
        for input in &inputs {
            stage
                .consumers
                .entry(input.clone())
                .or_default()
                .push(id.clone());
        }
        let urn = transform.spec().urn();
        let operator = match urn.to_str()? {
            "beam:runner:source:v1" | "beam:runner:sink:v1" => {
                let port = model::RemoteGrpcPort::parse(transform.spec().payload())?;
                let sender = hub
                    .connect(&endpoint(port.api_service_descriptor())?, worker)
                    .await?;
                let coder = coder(&descriptor, port.coder_id().to_str()?, 64)?;
                if urn == "beam:runner:source:v1" {
                    if outputs.len() != 1 || !inputs.is_empty() {
                        return Err(error("invalid runner source ports"));
                    }
                    if port_value(&coder)? != &collection_coder(&descriptor, &outputs[0])? {
                        return Err(error("source port/collection coder mismatch"));
                    }
                    sources.push((id, outputs[0].clone(), coder));
                    continue;
                }
                if inputs.len() != 1 || !outputs.is_empty() {
                    return Err(error("invalid runner sink ports"));
                }
                if port_value(&coder)? != &collection_coder(&descriptor, &inputs[0])? {
                    return Err(error("sink port/collection coder mismatch"));
                }
                Operator::Sink { coder, sender }
            }
            "beam:transform:flatten:v1" => {
                if outputs.len() != 1 {
                    return Err(error("Flatten requires one output"));
                }
                Operator::Flatten {
                    out: outputs[0].clone(),
                }
            }
            "beam:transform:pardo:v1" => {
                if inputs.len() != 1 || outputs.len() != 1 {
                    return Err(error("Rust ParDo requires one input/output"));
                }
                let pardo = model::ParDoPayload::parse(transform.spec().payload())?;
                if pardo.do_fn().urn() != DOFN
                    || !pardo.side_inputs().is_empty()
                    || !pardo.state_specs().is_empty()
                    || !pardo.timer_family_specs().is_empty()
                    || !pardo.restriction_coder_id().is_empty()
                    || pardo.requests_finalization()
                    || pardo.requires_stable_input()
                    || pardo.requires_time_sorted_input()
                {
                    return Err(error("unsupported Rust ParDo semantics"));
                }
                let payload = model::RustFunctionPayload::parse(pardo.do_fn().payload())?;
                registry.validate(&payload)?;
                let input = collection_coder(&descriptor, &inputs[0])?;
                let output = collection_coder(&descriptor, &outputs[0])?;
                let id = payload.function_id().to_str()?;
                if [CREATE, SUM, ASSERT_TAG, ASSERT_CHECK].contains(&id) {
                    if payload.config_contract() != "beam:rust:config:builtin:v1"
                        || payload.config_version() != 1
                    {
                        return Err(error("builtin configuration mismatch"));
                    }
                    Operator::Builtin {
                        out: outputs[0].clone(),
                        id: id.to_owned(),
                        config: payload.config().to_vec(),
                        input,
                        output,
                    }
                } else {
                    Operator::User {
                        out: outputs[0].clone(),
                        processor: None,
                        payload,
                        input,
                        output,
                    }
                }
            }
            _ => return Err(error(format!("unsupported transform {id}: {urn}"))),
        };
        stage.operators.insert(id, operator);
    }
    let mut readers = Vec::new();
    for (port, collection, coder) in sources {
        readers.push((hub.input(bundle, &port).await?, collection, coder));
    }
    let permit = compute.acquire_owned().await?;
    tokio::task::spawn_blocking(move || -> Result<()> {
        let _permit = permit;
        for operator in stage.operators.values_mut() {
            if let Operator::User {
                processor,
                payload,
                input,
                output,
                ..
            } = operator
            {
                let mut instance = registry.processor(payload, input, output)?;
                instance.start()?;
                *processor = Some(instance);
            }
        }
        for (mut file, collection, coder) in readers {
            let mut buffer = Vec::new();
            let mut chunk = [0; 64 * 1024];
            let mut eof = false;
            loop {
                if buffer.is_empty() && eof {
                    break;
                }
                match coder.decode_prefix(&buffer, Context::Nested, Limits::default()) {
                    Ok((Value::Windowed(value), consumed)) if consumed > 0 => {
                        stage.route(&collection, &value, 256)?;
                        buffer.drain(..consumed);
                    }
                    Err(DecodeError::NeedMore) if !eof => {
                        // Completed input streams are already on disk. Grow
                        // geometrically so a large grouped value is not
                        // decoded and reallocated once per 64 KiB fragment.
                        let target = buffer
                            .len()
                            .max(chunk.len())
                            .saturating_mul(2)
                            .min(Limits::default().bytes + 1);
                        while buffer.len() < target {
                            let wanted = (target - buffer.len()).min(chunk.len());
                            let count = file.read(&mut chunk[..wanted])?;
                            if count == 0 {
                                eof = true;
                                break;
                            }
                            buffer.extend_from_slice(&chunk[..count]);
                        }
                    }
                    Err(error) => return Err(error.into()),
                    _ => return Err(error("runner data must contain complete windowed values")),
                }
            }
        }
        for operator in stage.operators.values_mut() {
            if let Operator::User {
                processor: Some(processor),
                ..
            } = operator
            {
                processor.finish()?;
            }
        }
        for (id, operator) in &stage.operators {
            if let Operator::Sink { sender, .. } = operator {
                send_data(sender, &stage.bundle, id, vec![], true)?;
            }
        }
        Ok(())
    })
    .await
    .map_err(|panic| {
        error(format!(
            "bundle compute panicked; processor discarded: {panic}"
        ))
    })?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Application, DoFn, ProcessContext};

    struct Increment;
    impl DoFn for Increment {
        type Input = i64;
        type Output = i64;
        type Config = ();
        fn from_config(_: ()) -> Result<Self> {
            Ok(Self)
        }
        fn process(&mut self, value: &i64, context: &mut ProcessContext<'_, i64>) -> Result<()> {
            context.counter("seen", 1)?;
            context.emit(value.wrapping_add(1))
        }
    }

    #[test]
    fn fused_fanout_preserves_metadata_and_records_counters() {
        let app = Application::new("test")
            .register_dofn::<Increment>("test:increment:v1")
            .unwrap();
        let payload =
            app.registry
                .payload("test:increment:v1", "beam:rust:config:empty:v1", 1, vec![]);
        let mut processor = app
            .registry
            .processor(&payload, &Coder::VarInt, &Coder::VarInt)
            .unwrap();
        processor.start().unwrap();
        let (sender, mut receiver) = mpsc::channel(4);
        let metrics = Arc::new(Metrics::default());
        let wire = Coder::Windowed(Box::new(Coder::VarInt), Box::new(Coder::GlobalWindow));
        let mut stage = Stage {
            bundle: "bundle-7".to_owned(),
            metrics: metrics.clone(),
            consumers: HashMap::from([
                (
                    "input".to_owned(),
                    vec!["user".to_owned(), "direct".to_owned()],
                ),
                ("incremented".to_owned(), vec!["sink".to_owned()]),
            ]),
            operators: BTreeMap::from([
                (
                    "user".to_owned(),
                    Operator::User {
                        out: "incremented".to_owned(),
                        processor: Some(processor),
                        payload,
                        input: Coder::VarInt,
                        output: Coder::VarInt,
                    },
                ),
                (
                    "direct".to_owned(),
                    Operator::Sink {
                        coder: wire.clone(),
                        sender: sender.clone(),
                    },
                ),
                (
                    "sink".to_owned(),
                    Operator::Sink {
                        coder: wire.clone(),
                        sender,
                    },
                ),
            ]),
        };
        let input = WindowedValue {
            value: Value::I64(17),
            timestamp: -400_000,
            windows: vec![Value::GlobalWindow],
            pane: beam_coders::Pane {
                is_first: false,
                is_last: true,
                timing: beam_coders::Timing::Late,
                index: 30,
                on_time_index: 27,
            },
        };
        stage.route("input", &input, 64).unwrap();
        for _ in 0..2 {
            let (message, _) = receiver.blocking_recv().unwrap();
            let data = message.data().get(0).unwrap();
            assert_eq!(data.instruction_id(), "bundle-7");
            let Value::Windowed(output) = wire
                .decode(data.data(), Context::Nested, Limits::default())
                .unwrap()
            else {
                panic!("windowed output")
            };
            assert_eq!(output.timestamp, input.timestamp);
            assert_eq!(output.windows, input.windows);
            assert_eq!(output.pane, input.pane);
            assert_eq!(
                output.value,
                Value::I64(if data.transform_id() == "sink" {
                    18
                } else {
                    17
                })
            );
        }
        let infos = metrics.infos();
        let counter = infos
            .iter()
            .find(|info| info.labels().get("NAME").is_some_and(|name| name == "seen"))
            .unwrap();
        assert_eq!(counter.labels().get("PTRANSFORM").unwrap(), "user");
        assert_eq!(
            Coder::VarInt
                .decode(counter.payload(), Context::Outer, Limits::default())
                .unwrap(),
            Value::I64(1)
        );
    }

    #[test]
    fn malformed_coder_descriptors_fail_without_recursion_or_index_panics() {
        let mut descriptor = model::ProcessBundleDescriptor::new();
        let mut cyclic = model::Coder::new();
        cyclic.spec_mut().set_urn("beam:coder:iterable:v1");
        cyclic.component_coder_ids_mut().push("cyclic");
        descriptor.coders_mut().insert("cyclic", cyclic);
        assert!(coder(&descriptor, "cyclic", 64).is_err());
        assert!(coder(&descriptor, "missing", 64).is_err());
        let mut wrong = model::Coder::new();
        wrong.spec_mut().set_urn("beam:coder:kv:v1");
        descriptor.coders_mut().insert("wrong", wrong);
        assert!(coder(&descriptor, "wrong", 64).is_err());
    }
}
