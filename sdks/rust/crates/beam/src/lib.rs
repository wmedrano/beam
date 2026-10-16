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

//! Experimental native Beam SDK. This implementation is under development;
//! consult the tested feature matrix before deploying workers.
//!
//! Typed handles reject incompatible element types:
//! ```compile_fail
//! use apache_beam::{Application, PCollection};
//! let app = Application::new("example-build");
//! let mut pipeline = app.pipeline().unwrap();
//! let _: PCollection<String> = pipeline.create("Numbers", vec![1_i64]).unwrap();
//! ```
//! Distributed functions require explicit DoFn implementations:
//! ```compile_fail
//! use apache_beam::Application;
//! let app = Application::new("example-build");
//! app.register_dofn::<fn(i64) -> i64>("captured:closure:v1").unwrap();
//! ```

//! Borrowed emission is checked against the declared output type.
//! ```compile_fail
//! use apache_beam::ProcessContext;
//! fn incompatible(cx: &mut ProcessContext<'_, i64>) {
//!     cx.emit("not an integer").unwrap();
//! }
//! ```
//! Key/value components must match their declared types.
//! ```compile_fail
//! use apache_beam::{Kv, ProcessContext};
//! fn incompatible(cx: &mut ProcessContext<'_, Kv<String, i64>>) {
//!     cx.emit_kv("key", "not an integer").unwrap();
//! }
//! ```
//! Only keyed output contexts expose `emit_kv`.
//! ```compile_fail
//! use apache_beam::ProcessContext;
//! fn incompatible(cx: &mut ProcessContext<'_, String>) {
//!     cx.emit_kv("key", &1_i64).unwrap();
//! }
//! ```
//! Finish emission has the same type checks.
//! ```compile_fail
//! use apache_beam::{FinishContext, Kv};
//! fn incompatible(cx: &mut FinishContext<'_, Kv<String, i64>>) {
//!     cx.emit_kv(&1_i64, &2_i64, 0).unwrap();
//! }
//! ```
//! Emission borrows rather than consumes values.
//! ```compile_fail
//! use apache_beam::ProcessContext;
//! fn incompatible(cx: &mut ProcessContext<'_, i64>) {
//!     cx.emit(1_i64).unwrap();
//! }
//! ```
mod graph;
mod harness;
mod io;
mod job;
mod metrics;
mod registry;
mod values;

pub use graph::{PCollection, Pipeline, SumI64};
pub use job::{CounterResult, Job, JobOptions};
pub use registry::{Application, BeamConfig, DoFn, FinishContext, ProcessContext};
pub use values::{BeamValue, Bytes, EmitValue, Grouped, Kv};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn error(message: impl Into<String>) -> Error {
    std::io::Error::other(message.into()).into()
}
