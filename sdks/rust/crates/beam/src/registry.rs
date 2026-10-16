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

use crate::{BeamValue, EmitValue, Kv, Pipeline, Result, error};
use beam_coders::{Coder, ValueRef};
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
    emit: &'a mut dyn FnMut(ValueRef<'_>) -> Result<()>,
    counter: &'a mut dyn FnMut(&str, i64) -> Result<()>,
    log: &'a mut dyn FnMut(&str) -> Result<()>,
    marker: PhantomData<T>,
}
impl<T: BeamValue> ProcessContext<'_, T> {
    /// Emit a borrowed value synchronously. The runtime finishes using the
    /// reference before returning, so the caller can immediately reuse storage.
    /// Errors do not roll back outputs already delivered to other consumers.
    pub fn emit<E: EmitValue<T> + ?Sized>(&mut self, value: &E) -> Result<()> {
        (self.emit)(value.value_ref())
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

impl<K: BeamValue, V: BeamValue> ProcessContext<'_, Kv<K, V>> {
    /// Emit independently borrowed key and value components synchronously.
    pub fn emit_kv<EK: EmitValue<K> + ?Sized, EV: EmitValue<V> + ?Sized>(
        &mut self,
        key: &EK,
        value: &EV,
    ) -> Result<()> {
        let key = key.value_ref();
        let value = value.value_ref();
        (self.emit)(ValueRef::Kv(&key, &value))
    }
}

/// Finish outputs require an explicit event timestamp (milliseconds since the
/// Unix epoch). The prototype supports only the global window.
pub struct FinishContext<'a, T> {
    emit: &'a mut dyn FnMut(ValueRef<'_>, i64) -> Result<()>,
    counter: &'a mut dyn FnMut(&str, i64) -> Result<()>,
    log: &'a mut dyn FnMut(&str) -> Result<()>,
    marker: PhantomData<T>,
}
impl<T: BeamValue> FinishContext<'_, T> {
    /// Emit a borrowed value with an explicit event timestamp in milliseconds.
    /// The reference is used only during this synchronous call.
    pub fn emit<E: EmitValue<T> + ?Sized>(&mut self, value: &E, timestamp: i64) -> Result<()> {
        (self.emit)(value.value_ref(), timestamp)
    }
    pub fn counter(&mut self, name: &str, delta: i64) -> Result<()> {
        (self.counter)(name, delta)
    }
    pub fn log(&mut self, message: &str) -> Result<()> {
        (self.log)(message)
    }
}

impl<K: BeamValue, V: BeamValue> FinishContext<'_, Kv<K, V>> {
    /// Emit borrowed key/value components with an event timestamp in milliseconds.
    pub fn emit_kv<EK: EmitValue<K> + ?Sized, EV: EmitValue<V> + ?Sized>(
        &mut self,
        key: &EK,
        value: &EV,
        timestamp: i64,
    ) -> Result<()> {
        let key = key.value_ref();
        let value = value.value_ref();
        (self.emit)(ValueRef::Kv(&key, &value), timestamp)
    }
}

pub(crate) trait Processor: Send {
    fn start(&mut self) -> Result<()>;
    fn process(
        &mut self,
        input: ValueRef<'_>,
        emit: &mut dyn FnMut(ValueRef<'_>) -> Result<()>,
        counter: &mut dyn FnMut(&str, i64) -> Result<()>,
        log: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()>;
    fn finish(
        &mut self,
        emit: &mut dyn FnMut(ValueRef<'_>, i64) -> Result<()>,
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
        input: ValueRef<'_>,
        emit: &mut dyn FnMut(ValueRef<'_>) -> Result<()>,
        counter: &mut dyn FnMut(&str, i64) -> Result<()>,
        log: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()> {
        self.0.process(
            &F::Input::from_ref(input)?,
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
        emit: &mut dyn FnMut(ValueRef<'_>, i64) -> Result<()>,
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

#[cfg(test)]
mod emission_tests {
    use super::*;
    use crate::Bytes;
    use beam_coders::{Context, Value, ValueView};

    #[test]
    fn borrowed_strings_and_bytes_can_be_reused_after_each_emission() {
        let mut text = String::with_capacity(64);
        let pointer = text.as_ptr();
        let mut actual = Vec::new();
        let mut capture = |value: ValueRef<'_>| -> Result<()> {
            let ValueRef::String(value) = value else {
                panic!("expected string")
            };
            assert_eq!(value.as_ptr(), pointer);
            actual.push(value.to_owned());
            Ok(())
        };
        {
            let mut context = ProcessContext::<String> {
                emit: &mut capture,
                counter: &mut |_, _| Ok(()),
                log: &mut |_| Ok(()),
                marker: PhantomData,
            };
            for word in ["red", "green", "blue"] {
                text.clear();
                text.push_str(word);
                context.emit(text.as_str()).unwrap();
                context.emit(&text).unwrap();
            }
        }
        assert_eq!(actual, ["red", "red", "green", "green", "blue", "blue"]);

        let mut bytes = vec![0_u8; 4];
        let pointer = bytes.as_ptr();
        let mut actual = Vec::new();
        let mut capture = |value: ValueRef<'_>| -> Result<()> {
            let ValueRef::Bytes(value) = value else {
                panic!("expected bytes")
            };
            assert_eq!(value.as_ptr(), pointer);
            actual.push(value.to_vec());
            Ok(())
        };
        {
            let mut context = ProcessContext::<Bytes> {
                emit: &mut capture,
                counter: &mut |_, _| Ok(()),
                log: &mut |_| Ok(()),
                marker: PhantomData,
            };
            for byte in 0..3 {
                bytes.fill(byte);
                context.emit(bytes.as_slice()).unwrap();
            }
        }
        assert_eq!(actual, [vec![0; 4], vec![1; 4], vec![2; 4]]);
    }

    // This type deliberately cannot be cloned or converted to an owned Value.
    struct BorrowOnly(String);
    impl ValueView for BorrowOnly {
        fn value_ref(&self) -> ValueRef<'_> {
            self.0.value_ref()
        }
    }
    impl BeamValue for BorrowOnly {
        fn coder() -> Coder {
            Coder::Utf8
        }
        fn into_value(self) -> Value {
            panic!("emission must not create owned Values")
        }
        fn from_ref(value: ValueRef<'_>) -> Result<Self> {
            Ok(Self(String::from_ref(value)?))
        }
    }

    #[test]
    fn exact_and_key_value_emission_borrow_non_clone_components() {
        let key = BorrowOnly("key".to_owned());
        let bytes = Bytes(vec![1, 2, 3]);
        let key_pointer = key.0.as_ptr();
        let byte_pointer = bytes.0.as_ptr();
        let mut count = 0;
        let mut capture = |value: ValueRef<'_>| -> Result<()> {
            let ValueRef::Kv(k, v) = value else {
                panic!("expected kv")
            };
            let ValueRef::String(k) = k.value_ref() else {
                panic!("expected key")
            };
            let ValueRef::Bytes(v) = v.value_ref() else {
                panic!("expected value")
            };
            assert_eq!(k.as_ptr(), key_pointer);
            assert_eq!(v.as_ptr(), byte_pointer);
            count += 1;
            Ok(())
        };
        {
            let mut context = ProcessContext::<Kv<BorrowOnly, Bytes>> {
                emit: &mut capture,
                counter: &mut |_, _| Ok(()),
                log: &mut |_| Ok(()),
                marker: PhantomData,
            };
            context.emit_kv(&key, &bytes).unwrap();
            context.emit_kv(&key, bytes.0.as_slice()).unwrap();
            context.emit(&Kv::new(key, bytes)).unwrap();
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn finish_emits_borrowed_components_with_timestamps_and_propagates_errors() {
        let coder = Kv::<String, Bytes>::coder();
        let mut actual = Vec::new();
        let mut capture = |value: ValueRef<'_>, timestamp| -> Result<()> {
            actual.push((coder.encode_ref(value, Context::Nested)?, timestamp));
            if actual.len() == 2 {
                return Err(error("requested emission failure"));
            }
            Ok(())
        };
        {
            let mut context = FinishContext::<Kv<String, Bytes>> {
                emit: &mut capture,
                counter: &mut |_, _| Ok(()),
                log: &mut |_| Ok(()),
                marker: PhantomData,
            };
            context.emit_kv("key", &[1_u8, 2][..], -1234).unwrap();
            let value = Kv::new("next".to_owned(), Bytes(vec![3]));
            let failure = context.emit(&value, 5678).unwrap_err();
            assert!(failure.to_string().contains("requested emission failure"));
            assert_eq!(value.key, "next");
        }
        assert_eq!(
            actual.iter().map(|v| v.1).collect::<Vec<_>>(),
            [-1234, 5678]
        );
    }

    #[test]
    fn process_emission_propagates_errors_before_the_next_buffer_mutation() {
        let mut capture = |_: ValueRef<'_>| Err(error("downstream failed"));
        let mut context = ProcessContext::<String> {
            emit: &mut capture,
            counter: &mut |_, _| Ok(()),
            log: &mut |_| Ok(()),
            marker: PhantomData,
        };
        let mut value = "original".to_owned();
        assert!(
            context
                .emit(&value)
                .unwrap_err()
                .to_string()
                .contains("downstream failed")
        );
        value.clear();
        context.emit("temporary").unwrap_err();
    }
}
