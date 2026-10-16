<!--
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
-->

# Implement an experimental Apache Beam Rust SDK

## Objective

Deliver a tested native Rust prototype for bounded pipelines on **Prism
PROCESS**. Rust constructs a portable pipeline and runs compiled application
functions in its own Fn API harness. Scheduling and shuffles remain in Prism.
The tested platform is Linux x86_64.

The completion target is this prototype, rather than the full M0–M2 release
roadmap in [the design](contributor-docs/rust-sdk-design.md). Docker, Flink,
authenticated remote deployment, and distributed recovery are optional follow-up
work. Java expansion and cross-language I/O are outside scope. A separate Java
protocol interoperability gate is unnecessary for this prototype; real Go Beam
API and Prism tests establish its protocol evidence.

## Architectural decisions

- Use the checked-in Beam model without forking public protos. Retain the
  official `protocolbuffers/protobuf` 4.x runtime and matching compiler/WKT.
- Evaluate the official RPC client first. Preserve the reproducible failed
  deadline gate and the tested Tonic adapter for official protobuf messages.
  Keep transport/generated types out of ordinary public SDK APIs.
- Compile user functions into the worker binary, with explicit versioned
  registrations, serializable configuration, and validated build/registry
  identity. Never serialize closures, Rust memory, TypeId, or a dynamic ABI.
- Driver and worker initialize the same registry. Resources are created on
  workers. Native applications require neither Python nor a JVM.
- Use standard Beam element encodings and deterministic keys. Reject unknown
  required semantics and unsupported instructions explicitly.
- Keep the existing contributor design and index entry. Record implementation
  evidence where it refines the design. The broader proposal remains a roadmap.

## Prototype acceptance

- [x] Cargo workspace, pinned toolchain/MSRV, lockfile, generated model and URNs,
  reproducible generation, dependency inventory, and standalone package checks.
- [x] Real Go Beam protocol tests for unary/server/bidirectional streaming,
  metadata/status, deadlines, backpressure, and mTLS with negative cases.
- [x] Shared coder vectors, malformed decoder tests, complete element metadata,
  and unsupported coder declarations.
- [x] Typed graph handles, explicit DoFns/configuration, lifecycle hooks, Create,
  ParDo, Flatten, GroupByKey, wrapping SumI64, and distributed multiset assertions.
- [x] Fallible grouped iteration, disk spooling with a shared budget,
  independent/partial group consumption, and a real 80 MiB grouped-input test.
- [x] PROCESS boot/provision/control/data/logging, descriptor execution,
  finish-bundle outputs, user INFO logs, counter queries, bounded waiting and
  cancellation, and unary RPC deadlines.
- [x] Interleaved input, blocked output, disconnect, end-marker, registration
  bounds/atomicity, panic/failure, and resource cleanup tests.
- [x] Native bounded UTF-8 text reads and writes on the shared local filesystem,
  including empty files, CRLF, invalid UTF-8, missing inputs, and output errors.
- [x] WordCount and native text examples, Gradle wrappers, CI workflow,
  quickstart, explicit support matrix, and exact local validation results.

Run formatting, all-target Clippy, workspace tests, protocol integration,
Prism acceptance, regeneration, package consumption, and dependency inventory
checks. The scripts must fail if a required prerequisite is missing; ordinary
Cargo tests explicitly ignore integration tests that the scripts run separately.
A skipped test is never a passing integration result. Record any qualified
checks, including local-only license audit exclusions.

See [sdks/rust/VALIDATION.md](sdks/rust/VALIDATION.md) for commands/results and
[sdks/rust/features.toml](sdks/rust/features.toml) for the capability matrix.
Publication, hosted CI execution, comparative performance claims, and production
readiness are not completion requirements for this prototype.

## Follow-up work

Further native work can add general custom coder URNs/factories, richer pipeline
options, long-running whole-worker stress tests, real-runner Register acceptance,
comparative startup/throughput/memory benchmarks, and more native connectors.
These do not prevent completing this prototype.

Optional deployment work includes Docker workers, artifact staging/retrieval,
cross-platform build manifests, authenticated submission, and Flink batch with
worker failure/retry acceptance. Rows, expansion/schema transforms, foreign
connectors, and Java-specific interoperability projects are outside this scope.
Advanced streaming, state, timers, side inputs, SDF, external pools, and Rust
expansion servers remain separate roadmap work.

The local text sink atomically publishes each completed bundle's unique shard.
Retries can duplicate shards; this does not provide exactly-once I/O or bundle
finalization. Group and message bounds remain explicit. Do not infer another
runner's support, a speedup, or production guarantees from this prototype.
