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

use crate::{BeamValue, Pipeline, Result, error};
use beam_coders::{Coder, Value};
use beam_model::RustFunctionPayload;
use sha2::{Digest, Sha256};
use std::any::TypeId;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait BeamConfig: Sized + Send + 'static {
    const CONTRACT: &'static str;
    const VERSION: u32;
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Result<Self>;
}
impl BeamConfig for () {
    const CONTRACT: &'static str = "beam:rust:config:empty:v1";
    const VERSION: u32 = 1;
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() {
            Ok(())
        } else {
            Err(error("empty configuration has bytes"))
        }
    }
}

pub trait DoFn: Send + Sized + 'static {
    type Input: BeamValue;
    type Output: BeamValue;
    type Config: BeamConfig;
    fn from_config(config: Self::Config) -> Result<Self>;
    fn setup(&mut self) -> Result<()> {
        Ok(())
    }
    fn start_bundle(&mut self) -> Result<()> {
        Ok(())
    }
    fn process(
        &mut self,
        input: &Self::Input,
        context: &mut ProcessContext<'_, Self::Output>,
    ) -> Result<()>;
    fn finish_bundle(&mut self, _context: &mut FinishContext<'_, Self::Output>) -> Result<()> {
        Ok(())
    }
    fn teardown(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct ProcessContext<'a, T> {
    emit: &'a mut dyn FnMut(Value) -> Result<()>,
    counter: &'a mut dyn FnMut(&str, i64) -> Result<()>,
    log: &'a mut dyn FnMut(&str) -> Result<()>,
    marker: PhantomData<T>,
}
impl<T: BeamValue> ProcessContext<'_, T> {
    pub fn emit(&mut self, value: T) -> Result<()> {
        (self.emit)(value.into_value())
    }
    /// Add to a bundle-local attempted counter in namespace `rust`.
    pub fn counter(&mut self, name: &str, delta: i64) -> Result<()> {
        (self.counter)(name, delta)
    }
    /// Send an INFO entry through Fn Logging with bundle/transform context.
    pub fn log(&mut self, message: &str) -> Result<()> {
        (self.log)(message)
    }
}

/// Finish outputs require an explicit event timestamp (milliseconds since the
/// Unix epoch). The prototype supports only the global window.
pub struct FinishContext<'a, T> {
    emit: &'a mut dyn FnMut(Value, i64) -> Result<()>,
    counter: &'a mut dyn FnMut(&str, i64) -> Result<()>,
    log: &'a mut dyn FnMut(&str) -> Result<()>,
    marker: PhantomData<T>,
}
impl<T: BeamValue> FinishContext<'_, T> {
    pub fn emit(&mut self, value: T, timestamp: i64) -> Result<()> {
        (self.emit)(value.into_value(), timestamp)
    }
    pub fn counter(&mut self, name: &str, delta: i64) -> Result<()> {
        (self.counter)(name, delta)
    }
    pub fn log(&mut self, message: &str) -> Result<()> {
        (self.log)(message)
    }
}

pub(crate) trait Processor: Send {
    fn start(&mut self) -> Result<()>;
    fn process(
        &mut self,
        input: &Value,
        emit: &mut dyn FnMut(Value) -> Result<()>,
        counter: &mut dyn FnMut(&str, i64) -> Result<()>,
        log: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()>;
    fn finish(
        &mut self,
        emit: &mut dyn FnMut(Value, i64) -> Result<()>,
        counter: &mut dyn FnMut(&str, i64) -> Result<()>,
        log: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()>;
}
struct Function<F: DoFn>(F);
impl<F: DoFn> Drop for Function<F> {
    fn drop(&mut self) {
        // Teardown is best effort, including while unwinding another user hook.
        // A second panic must not abort unrelated bundles in this worker.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.0.teardown()));
    }
}
impl<F: DoFn> Processor for Function<F> {
    fn start(&mut self) -> Result<()> {
        self.0.setup()?;
        self.0.start_bundle()
    }
    fn process(
        &mut self,
        input: &Value,
        emit: &mut dyn FnMut(Value) -> Result<()>,
        counter: &mut dyn FnMut(&str, i64) -> Result<()>,
        log: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()> {
        self.0.process(
            &F::Input::from_value(input)?,
            &mut ProcessContext {
                emit,
                counter,
                log,
                marker: PhantomData,
            },
        )
    }
    fn finish(
        &mut self,
        emit: &mut dyn FnMut(Value, i64) -> Result<()>,
        counter: &mut dyn FnMut(&str, i64) -> Result<()>,
        log: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()> {
        self.0.finish_bundle(&mut FinishContext {
            emit,
            counter,
            log,
            marker: PhantomData,
        })
    }
}

pub(crate) struct Entry {
    pub id: String,
    pub input: Coder,
    pub output: Coder,
    pub config_contract: &'static str,
    pub config_version: u32,
    factory: fn(&[u8]) -> Result<Box<dyn Processor>>,
    rust_type: TypeId, // Local lookup only; never a protocol identifier.
}
#[derive(Default)]
pub(crate) struct Registry {
    pub entries: BTreeMap<String, Entry>,
    pub build_id: String,
}
impl Registry {
    pub fn entry<F: DoFn>(&self) -> Result<&Entry> {
        self.entries
            .values()
            .find(|e| e.rust_type == TypeId::of::<F>())
            .ok_or_else(|| error("DoFn is not explicitly registered"))
    }
    pub fn digest(&self) -> Vec<u8> {
        let mut hash = Sha256::new();
        hash.update(b"beam-rust-registry-v1\0");
        for (id, entry) in &self.entries {
            // Length-delimited, sorted logical fields. No protobuf map ordering,
            // TypeId, object memory, symbol names or target-specific addresses.
            for field in [
                id.clone(),
                entry.input.signature(),
                entry.output.signature(),
                entry.config_contract.to_owned(),
                entry.config_version.to_string(),
            ] {
                hash.update((field.len() as u64).to_be_bytes());
                hash.update(field.as_bytes());
            }
        }
        hash.finalize().to_vec()
    }
    pub fn payload(
        &self,
        id: &str,
        contract: &str,
        version: u32,
        config: Vec<u8>,
    ) -> RustFunctionPayload {
        let mut payload = RustFunctionPayload::new();
        payload.set_version(1);
        payload.set_function_id(id);
        payload.set_config_contract(contract);
        payload.set_config_version(version);
        payload.set_config(config);
        payload.set_registry_digest(self.digest());
        payload.set_build_id(&self.build_id);
        payload
    }
    pub fn validate(&self, payload: &RustFunctionPayload) -> Result<()> {
        if payload.version() != 1
            || payload.build_id() != self.build_id.as_str()
            || payload.registry_digest() != self.digest()
        {
            return Err(error(
                "Rust worker application/registry identity mismatch or unsupported payload version",
            ));
        }
        Ok(())
    }
    pub fn processor(
        &self,
        payload: &RustFunctionPayload,
        input: &Coder,
        output: &Coder,
    ) -> Result<Box<dyn Processor>> {
        self.validate(payload)?;
        let id = payload.function_id().to_str()?;
        let entry = self
            .entries
            .get(id)
            .ok_or_else(|| error(format!("unregistered DoFn {id}")))?;
        if input != &entry.input
            || output != &entry.output
            || payload.config_contract() != entry.config_contract
            || payload.config_version() != entry.config_version
        {
            return Err(error(format!(
                "DoFn {id} coder/configuration contract mismatch"
            )));
        }
        (entry.factory)(payload.config())
    }
}

pub struct Application {
    pub(crate) registry: Registry,
}
impl Application {
    pub fn pipeline(&self) -> Result<Pipeline<'_>> {
        Pipeline::new(&self.registry)
    }
    /// `build_id` identifies the complete driver/worker build set. Generate it
    /// from application sources, SDK, lockfile and features in release builds.
    pub fn new(build_id: impl Into<String>) -> Self {
        Self {
            registry: Registry {
                build_id: build_id.into(),
                ..Registry::default()
            },
        }
        .register_dofn::<crate::io::ReadText>("beam:rust:stdlib:read_text:v1")
        .expect("unique built-in text reader")
        .register_dofn::<crate::io::WriteText>("beam:rust:stdlib:write_text:v1")
        .expect("unique built-in text writer")
    }
    pub fn register_dofn<F: DoFn>(mut self, id: &str) -> Result<Self> {
        if id.is_empty()
            || id.starts_with("beam:rust:builtin:")
            || self.registry.entries.contains_key(id)
            || self
                .registry
                .entries
                .values()
                .any(|e| e.rust_type == TypeId::of::<F>())
        {
            return Err(error("duplicate, empty or reserved function registration"));
        }
        fn factory<F: DoFn>(bytes: &[u8]) -> Result<Box<dyn Processor>> {
            Ok(Box::new(Function(F::from_config(F::Config::decode(
                bytes,
            )?)?)))
        }
        self.registry.entries.insert(
            id.to_owned(),
            Entry {
                id: id.to_owned(),
                input: F::Input::coder(),
                output: F::Output::coder(),
                config_contract: F::Config::CONTRACT,
                config_version: F::Config::VERSION,
                factory: factory::<F>,
                rust_type: TypeId::of::<F>(),
            },
        );
        Ok(self)
    }
    pub fn run_from_env(self, build: impl FnOnce(&mut Pipeline<'_>) -> Result<()>) -> Result<()> {
        let args: BTreeMap<String, String> = std::env::args()
            .skip(1)
            .filter_map(|arg| {
                arg.strip_prefix("--")
                    .and_then(|s| s.split_once('='))
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
            })
            .collect();
        let runtime = tokio::runtime::Runtime::new()?;
        if args.contains_key("id") {
            return runtime.block_on(crate::harness::run(Arc::new(self.registry), args));
        }
        let mut pipeline = Pipeline::new(&self.registry)?;
        build(&mut pipeline)?;
        let endpoint = args
            .get("endpoint")
            .ok_or_else(|| error("driver requires --endpoint=host:port"))?;
        runtime.block_on(async {
            let mut job = pipeline
                .submit(
                    endpoint,
                    crate::JobOptions {
                        name: args
                            .get("job_name")
                            .cloned()
                            .unwrap_or_else(|| "rust-beam".to_owned()),
                        rpc_timeout: std::time::Duration::from_millis(
                            args.get("rpc_timeout_ms")
                                .map(|value| value.parse::<u64>())
                                .transpose()?
                                .unwrap_or(10_000),
                        ),
                    },
                )
                .await?;
            eprintln!("submitted job {}", job.id);
            let wait = args
                .get("wait_ms")
                .map(|value| value.parse::<u64>())
                .transpose()?
                .map(std::time::Duration::from_millis)
                .unwrap_or(std::time::Duration::from_secs(120));
            job.wait(wait).await?;
            if args
                .get("print_counters")
                .is_some_and(|value| value == "true")
            {
                for counter in job.counters().await? {
                    eprintln!(
                        "counter committed={} urn={} labels={:?} value={}",
                        counter.committed, counter.urn, counter.labels, counter.value
                    );
                }
            }
            Ok(())
        })
    }
}
