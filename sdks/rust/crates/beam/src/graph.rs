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

use crate::registry::{BeamConfig, DoFn, Registry};
use crate::{BeamValue, Bytes, Job, JobOptions, Kv, Result, error};
use beam_coders::{Coder, Context, Value};
use beam_model as model;
use protobuf::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_PIPELINE: AtomicU64 = AtomicU64::new(1);
pub(crate) const DOFN: &str = "beam:dofn:rust:v1";
pub(crate) const CREATE: &str = "beam:rust:builtin:create:v1";
pub(crate) const SUM: &str = "beam:rust:builtin:sum_i64:v1";
pub(crate) const ASSERT_TAG: &str = "beam:rust:builtin:assert_tag:v1";
pub(crate) const ASSERT_CHECK: &str = "beam:rust:builtin:assert_check:v1";

pub struct PCollection<T> {
    pub(crate) id: String,
    owner: u64,
    pub(crate) coder: Coder,
    marker: PhantomData<T>,
}
impl<T> Clone for PCollection<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            owner: self.owner,
            coder: self.coder.clone(),
            marker: PhantomData,
        }
    }
}
pub struct SumI64;

pub struct Pipeline<'a> {
    registry: &'a Registry,
    pub(crate) proto: model::Pipeline,
    owner: u64,
    sequence: usize,
    names: BTreeSet<String>,
    coders: BTreeMap<String, String>,
}

impl<'a> Pipeline<'a> {
    pub(crate) fn new(registry: &'a Registry) -> Result<Self> {
        if registry.build_id.is_empty() {
            return Err(error("application build identity must not be empty"));
        }
        let mut proto = model::Pipeline::new();
        let mut environment = model::Environment::new();
        environment.set_urn("beam:env:process:v1");
        let mut process = model::ProcessPayload::new();
        process.set_command(std::env::current_exe()?.to_string_lossy().as_ref());
        process.set_os(std::env::consts::OS);
        process.set_arch(if cfg!(target_arch = "x86_64") {
            "amd64"
        } else {
            std::env::consts::ARCH
        });
        environment.set_payload(process.serialize()?);
        proto
            .components_mut()
            .environments_mut()
            .insert("rust", environment);
        let mut windows = model::WindowingStrategy::new();
        windows
            .window_fn_mut()
            .set_urn("beam:window_fn:global_windows:v1");
        windows.trigger_mut().default_mut();
        windows.set_merge_status(model::merge_status::Enum::NonMerging);
        windows.set_accumulation_mode(model::accumulation_mode::Enum::Discarding);
        windows.set_output_time(model::output_time::Enum::EndOfWindow);
        windows.set_closing_behavior(model::closing_behavior::Enum::EmitIfNonempty);
        windows.set_on_time_behavior(model::on_time_behavior::Enum::FireAlways);
        windows.set_assigns_to_one_window(true);
        proto
            .components_mut()
            .windowing_strategies_mut()
            .insert("global", windows);
        let mut pipeline = Self {
            registry,
            proto,
            owner: NEXT_PIPELINE.fetch_add(1, Ordering::Relaxed),
            sequence: 0,
            names: BTreeSet::new(),
            coders: BTreeMap::new(),
        };
        let window_coder = pipeline.coder(&Coder::GlobalWindow)?;
        pipeline
            .proto
            .components_mut()
            .windowing_strategies_mut()
            .get_mut("global")
            .unwrap()
            .set_window_coder_id(window_coder);
        Ok(pipeline)
    }

    fn check<T>(&self, input: &PCollection<T>) -> Result<()> {
        if input.owner != self.owner {
            return Err(error("PCollection belongs to a different pipeline"));
        }
        Ok(())
    }

    pub(crate) fn coder(&mut self, coder: &Coder) -> Result<String> {
        let signature = coder.signature();
        if let Some(id) = self.coders.get(&signature) {
            return Ok(id.clone());
        }
        let (urn, components): (&str, Vec<&Coder>) = match coder {
            Coder::Bytes => ("bytes", vec![]),
            Coder::Utf8 => ("string_utf8", vec![]),
            Coder::Bool => ("bool", vec![]),
            Coder::VarInt => ("varint", vec![]),
            Coder::Double => ("double", vec![]),
            Coder::GlobalWindow => ("global_window", vec![]),
            Coder::IntervalWindow => ("interval_window", vec![]),
            Coder::Kv(k, v) => ("kv", vec![k, v]),
            Coder::Iterable(c) => ("iterable", vec![c]),
            Coder::Nullable(c) => ("nullable", vec![c]),
            Coder::LengthPrefix(c) => ("length_prefix", vec![c]),
            Coder::Windowed(v, w) => ("windowed_value", vec![v, w]),
        };
        let mut proto = model::Coder::new();
        proto.spec_mut().set_urn(format!("beam:coder:{urn}:v1"));
        for component in components {
            proto.component_coder_ids_mut().push(self.coder(component)?);
        }
        let id = format!("coder{}", self.coders.len());
        self.proto
            .components_mut()
            .coders_mut()
            .insert(id.as_str(), proto);
        self.coders.insert(signature, id.clone());
        Ok(id)
    }

    fn add<T>(
        &mut self,
        name: &str,
        urn: &str,
        payload: Vec<u8>,
        inputs: &[String],
        coder: Coder,
        rust: bool,
    ) -> Result<PCollection<T>> {
        if name.is_empty() || !self.names.insert(name.to_owned()) {
            return Err(error(format!("empty or duplicate transform name {name}")));
        }
        let id = format!("transform{}", self.sequence);
        self.sequence += 1;
        let output = format!("{id}.out");
        let mut collection = model::PCollection::new();
        collection.set_unique_name(&output);
        collection.set_coder_id(self.coder(&coder)?);
        collection.set_windowing_strategy_id("global");
        collection.set_is_bounded(model::is_bounded::Enum::Bounded);
        self.proto
            .components_mut()
            .pcollections_mut()
            .insert(output.as_str(), collection);
        let mut transform = model::PTransform::new();
        transform.set_unique_name(name);
        transform.spec_mut().set_urn(urn);
        transform.spec_mut().set_payload(payload);
        if rust {
            transform.set_environment_id("rust");
        }
        for (i, input) in inputs.iter().enumerate() {
            transform.inputs_mut().insert(i.to_string().as_str(), input);
        }
        transform.outputs_mut().insert("out", &output);
        self.proto
            .components_mut()
            .transforms_mut()
            .insert(id.as_str(), transform);
        self.proto.root_transform_ids_mut().push(id);
        Ok(PCollection {
            id: output,
            owner: self.owner,
            coder,
            marker: PhantomData,
        })
    }

    fn builtin<T>(
        &mut self,
        name: &str,
        id: &str,
        input: &str,
        output: Coder,
        config: Vec<u8>,
    ) -> Result<PCollection<T>> {
        let envelope = self
            .registry
            .payload(id, "beam:rust:config:builtin:v1", 1, config);
        let mut payload = model::ParDoPayload::new();
        payload.do_fn_mut().set_urn(DOFN);
        payload.do_fn_mut().set_payload(envelope.serialize()?);
        self.add(
            name,
            "beam:transform:pardo:v1",
            payload.serialize()?,
            &[input.to_owned()],
            output,
            true,
        )
    }

    pub fn create<T: BeamValue>(&mut self, name: &str, values: Vec<T>) -> Result<PCollection<T>> {
        let impulse: PCollection<Bytes> = self.add(
            &format!("{name}/Impulse"),
            "beam:transform:impulse:v1",
            vec![],
            &[],
            Coder::Bytes,
            false,
        )?;
        let config =
            Coder::Iterable(Box::new(T::coder())).encode(&values.into_value(), Context::Outer)?;
        self.builtin(name, CREATE, &impulse.id, T::coder(), config)
    }

    pub fn par_do<F: DoFn>(
        &mut self,
        name: &str,
        input: &PCollection<F::Input>,
        config: F::Config,
    ) -> Result<PCollection<F::Output>> {
        self.check(input)?;
        let entry = self.registry.entry::<F>()?;
        if input.coder != entry.input {
            return Err(error("ParDo input coder contract mismatch"));
        }
        let envelope = self.registry.payload(
            &entry.id,
            F::Config::CONTRACT,
            F::Config::VERSION,
            config.encode()?,
        );
        let mut payload = model::ParDoPayload::new();
        payload.do_fn_mut().set_urn(DOFN);
        payload.do_fn_mut().set_payload(envelope.serialize()?);
        self.add(
            name,
            "beam:transform:pardo:v1",
            payload.serialize()?,
            std::slice::from_ref(&input.id),
            entry.output.clone(),
            true,
        )
    }

    pub fn flatten<T: BeamValue>(
        &mut self,
        name: &str,
        inputs: &[PCollection<T>],
    ) -> Result<PCollection<T>> {
        for input in inputs {
            self.check(input)?;
            if input.coder != T::coder() {
                return Err(error("Flatten coder mismatch"));
            }
        }
        self.add(
            name,
            "beam:transform:flatten:v1",
            vec![],
            &inputs.iter().map(|p| p.id.clone()).collect::<Vec<_>>(),
            T::coder(),
            false,
        )
    }

    pub fn group_by_key<K: BeamValue, V: BeamValue>(
        &mut self,
        name: &str,
        input: &PCollection<Kv<K, V>>,
    ) -> Result<PCollection<Kv<K, Vec<V>>>> {
        self.check(input)?;
        if !K::coder().deterministic_key() {
            return Err(error("GroupByKey requires a deterministic key coder"));
        }
        self.add(
            name,
            "beam:transform:group_by_key:v1",
            vec![],
            std::slice::from_ref(&input.id),
            Kv::<K, Vec<V>>::coder(),
            false,
        )
    }

    pub fn combine_per_key<K: BeamValue>(
        &mut self,
        name: &str,
        input: &PCollection<Kv<K, i64>>,
        _: SumI64,
    ) -> Result<PCollection<Kv<K, i64>>> {
        let grouped = self.group_by_key(&format!("{name}/Group"), input)?;
        self.builtin(name, SUM, &grouped.id, Kv::<K, i64>::coder(), vec![])
    }

    /// A distributed multiset assertion. A sentinel guarantees execution for
    /// empty inputs, and GroupByKey brings all values through a real shuffle.
    pub fn assert_contents<T: BeamValue>(
        &mut self,
        name: &str,
        input: &PCollection<T>,
        expected: Vec<T>,
    ) -> Result<()> {
        self.check(input)?;
        type Tagged = Kv<i64, Kv<bool, Bytes>>;
        let tagged: PCollection<Tagged> = self.builtin(
            &format!("{name}/Tag"),
            ASSERT_TAG,
            &input.id,
            Tagged::coder(),
            vec![],
        )?;
        let sentinel = self.create(
            &format!("{name}/Sentinel"),
            vec![Kv::new(0, Kv::new(false, Bytes(vec![])))],
        )?;
        let all = self.flatten(&format!("{name}/Flatten"), &[tagged, sentinel])?;
        let grouped = self.group_by_key(&format!("{name}/Group"), &all)?;
        let mut expected: Vec<Vec<u8>> = expected
            .into_iter()
            .map(|v| T::coder().encode(&v.into_value(), Context::Nested))
            .collect::<std::result::Result<_, _>>()?;
        expected.sort();
        let config = Coder::Iterable(Box::new(Coder::Bytes)).encode(
            &Value::Iterable(expected.into_iter().map(Value::Bytes).collect()),
            Context::Outer,
        )?;
        let _: PCollection<Bytes> =
            self.builtin(name, ASSERT_CHECK, &grouped.id, Coder::Bytes, config)?;
        Ok(())
    }

    pub async fn submit(&self, endpoint: &str, options: JobOptions) -> Result<Job> {
        crate::job::submit(self.proto.clone(), endpoint, options).await
    }
}
