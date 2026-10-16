<!--
Licensed to the Apache Software Foundation (ASF) under one or more
contributor license agreements. See the NOTICE file distributed with
this work for additional information regarding copyright ownership.
The ASF licenses this file to You under the Apache License, Version 2.0
(the "License"); you may not use this file except in compliance with
the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
-->

# Experimental native Rust SDK

This change establishes a bounded **Prism PROCESS vertical slice**: Rust builds
a portable pipeline and the same application executable runs its registered
functions over the Fn API. WordCount, grouping, distributed multiset assertions,
and failure propagation execute against Prism built from this checkout.

The prototype scope in [TASK.md](../../TASK.md) is implemented and locally
validated. The broader M1/M2 release roadmap remains unfinished.
[features.toml](features.toml) records tested features and optional follow-up work;
[VALIDATION.md](VALIDATION.md) records exact commands and observed results.
Docker, Flink, and remote artifacts are optional deployment work. Java
expansion, foreign I/O, and Rows are outside this prototype’s scope. The
[design](../../contributor-docs/rust-sdk-design.md) describes the larger roadmap.

## Quickstart

The tested platform is Linux x86_64. Install Rustup, a C99 compiler/linker, and
Go matching `sdks/go.mod` (tested: Go 1.26.2). Rust 1.96 is the declared MSRV;
the pinned contributor toolchain is **1.96.1**. Cargo builds need neither
`protoc`, Go, Python nor a JVM. Go is needed to build the runner for these tests.

From the repository root:

```bash
cd sdks/rust
rustup toolchain install 1.96.1 --profile minimal --component rustfmt,clippy
bash tools/test-prism.sh
```

The acceptance command builds the checked-out Prism and the native
[WordCount application](crates/beam/examples/wordcount.rs), runs actual PROCESS
workers, and checks 23 scenarios. Missing prerequisites fail the command.
Nothing is substituted for a missing runner.

To run WordCount separately, start the job service in one terminal:

```bash
cd sdks
mkdir -p rust/target/tools
go build -mod=readonly -o rust/target/tools/prism ./go/cmd/prism
rust/target/tools/prism --job_port=8073 --serve_http=false
```

In another terminal:

```bash
cd sdks/rust
cargo run -p apache-beam --example wordcount --locked -- \
  --endpoint=127.0.0.1:8073 --print_counters=true
```

The worker asserts counts `red=3`, `blue=2`, `green=1` through a genuine
GroupByKey shuffle. The driver observes job state and counters; assertions
execute in workers. The separate
[acceptance application](crates/beam/examples/acceptance.rs) adds doubled counts
after fan-out/Flatten, empty input, repeated values, wrapping sums and lifecycle
tests. Run it with `--example acceptance`; `--case=large` additionally sends
three 2 MiB byte elements through a shuffle, `--case=slow` exercises CPU work,
and `--wait_ms=100 --case=slow` exercises timeout and cancellation.
`--case=hot_group` reads an 80 MiB shuffled group, `--case=partial_group` leaves
group iterators partly consumed, and `--case=finish_emit` checks chained
finish-bundle emissions.
Assertions compare nested element encodings, including floating-point bit
patterns when floats are used as values.

## Native text I/O

The [text I/O application](crates/beam/examples/text_io.rs) copies UTF-8 lines
using Rust workers. With the same Prism service running:

```bash
cd sdks/rust
cargo run -p apache-beam --example text_io --locked -- \
  --endpoint=127.0.0.1:8073 --input=/absolute/path/input.txt \
  --output=/absolute/path/output-shards
```

`Pipeline::read_text` reads paths on workers, accepting LF, CRLF, empty files,
and unterminated final lines. `Pipeline::write_text` writes one line per element
and emits completed shard paths. Embedded CR/LF in output elements is rejected.
Driver construction performs no file I/O; workers must share the filesystem.
Each bundle publishes a unique shard atomically on successful finish. Retries
can duplicate shards, and later bundle failures can leave published files.
There is no exactly-once or distributed commit guarantee. Files are not split
into restrictions or watched for changes.

## Application contract

Implement `DoFn` with associated `Input`, `Output`, and `Config` types, then
register it with an explicit versioned ID on `Application`. In the construction
closure use `Pipeline::create`, `par_do`, `flatten`, `group_by_key`,
`combine_per_key(..., SumI64)`, and `assert_contents`. Handles are typed and
bound to their originating pipeline. Graph construction never invokes function
factories. Worker mode initializes the same registry and skips the closure.

`DoFn` receives immutable borrowed inputs and emits owned outputs through
`ProcessContext`. Setup/start/process/finish/teardown run on a bundle-local
instance. Resources belong in setup; teardown is best effort.
`finish_bundle` receives `FinishContext`, whose `emit(value, timestamp)` requires
an explicit event timestamp in milliseconds. Finish runs in dependency order,
so upstream finish outputs reach downstream functions before those finish.
The current window is always global. Factories and hooks run on the compute
pool. Panics fail their bundle and discard its
processors. The SDK never serializes closures, object memory, Rust type IDs,
addresses, or a dynamic-library ABI.

`BeamConfig` declares a stable contract and version and supplies fallible
encoding/decoding. Empty and UTF-8 string configurations are built in; the
acceptance example also implements its own configuration contract. Official protobuf messages can implement
this trait using their serialize/parse operations and an explicit application
contract. Configuration belongs to the pipeline; discover secrets on workers.

`Application::new(build_id)` requires a nonempty identifier shared by the
driver and worker. For a release build, generate it from application and SDK
sources, lockfile and features. The example uses a fixed development ID and the
same local executable. A sorted, length-delimited logical registry manifest is
hashed separately. Its payload version, build ID, function/coder contracts,
configuration contract/version, and registry digest are checked before setup.
Cross-platform build manifests are follow-up work.

`SumI64` uses wrapping arithmetic and lowers to GroupByKey plus a reduction
ParDo. Optimizable/lifted combines are not emitted. Default key coders exclude
floating-point values. Custom `BeamValue` implementations must preserve the
semantics and determinism of the standard coder they select.

`Job` provides bounded waiting, cancellation requests, and attempted/committed
sum counters. Cancellation acknowledgement is a request acknowledgement;
terminal cancellation must be observed through the job service. Submission
accepts a job name, a positive unary RPC timeout, and empty Beam pipeline
options. CLI flags include `--job_name` and `--rpc_timeout_ms` (default 10000). The local PROCESS
path uses plaintext endpoints and rejects unsupported authentication URNs and
HTTPS configuration. The private protocol adapter has separate mTLS tests;
that does not establish authenticated SDK deployment support.

## Execution limits

The harness executes the runner's descriptor, including RemoteGrpcPort coders
and source/sink transform IDs. It fetches descriptors by ID and accepts Register.
It handles fan-out, multiple inputs, concurrent bundles, arbitrary input frame
boundaries, and explicit output end markers. CPU work is separated from RPC
processing. Active work is failed on transport loss; the SDK does not replay
bundles on reconnect.

Global windows, default triggering, and zero lateness are required. Complete
timestamp/window/pane metadata travels with each element. Fn Data messages are
limited to 128 MiB; other RPC messages default to 4 MiB. Decoders limit each
item and encoded output to 64 MiB, aggregate collection items to 1,000,000, and
nesting to 64. Inputs spool to temporary files under a 1 GiB worker disk budget with at
most 128 input streams. At most 128 bundle tasks and four compute tasks are
admitted, and fetched descriptors are cached up to 128 entries. Outbound queues
have eight slots with chunks of at most 1 MiB.

`group_by_key` returns `Kv<K, Grouped<V>>`. Use `for value in group.iter()?`
and propagate each item's `Result`. Completed input groups are validated and
spooled before processing. Iterators decode one owned item at a time, maintain
independent cursors, and may be partially consumed. The worker shares a separate
1 GiB decoded-group disk budget across live groups and retained iterators;
files and budget are released with the last reference. `Vec<V>` remains an
explicit materializing conversion for applications that want one. Limits bound
SDK inputs, not allocations made by arbitrary user code. Forwarding a complete
group still obeys the 64 MiB output-element limit. Prism currently sends large
groups in one RPC message, which can hit the 128 MiB message cap before the
spool budget.

MonitoringInfo records cover element counts and user sum counters. Short IDs
are used only when provisioning advertises the protocol, with metadata lookup
on the control stream. `ProcessContext::log` and `FinishContext::log` send INFO
messages with timestamp, bundle, and transform through Fn Logging. Messages
are capped at 16 KiB and the queue at 32 entries; full/disconnected queues fall
back to stderr. External logging-facade integration and configurable levels are
future work. User errors include worker/bundle/transform context.

## Dependency decision and regeneration

| Component | Exact selection |
| --- | --- |
| Message compiler | `protoc 34.0` |
| Official Rust runtime / well-known types | `protobuf =4.34.0-release`, `protobuf-well-known-types =4.34.0-release` |
| Kernel | upb, bundled and compiled by the official runtime's C99 build |
| Active RPC implementation | `tonic =0.14.6`, private official-message codec |
| Service generator | Descriptor-driven `tools/generate.py` |
| Evaluated official client and plugin | `grpc =0.9.0`, `grpc-protobuf =0.9.0`, `protoc-gen-rust-grpc 0.9.0` |
| Regeneration descriptor inspection | Python `protobuf ==7.36.1` |

See [protocol-decisions.md](protocol-decisions.md) for the official-client
failure and fallback evidence. The older community `protobuf 3.x` runtime
and Prost message generation are not selected. The official 4.x runtime and
well-known types must match their compiler exactly. Cargo.lock pins transitive
dependencies. [DEPENDENCIES.csv](DEPENDENCIES.csv) inventories license metadata
for all features, including the opt-in official-client reproducer.
Regenerate it with `python3 tools/dependencies.py`; `--check` detects drift.

Normal builds compile checked-in generated Rust with upb's native implementation
from the runtime crate. They perform no generation, compiler download, or CMake
build. Target workers need a compatible C toolchain; cross-compilation and
other worker targets remain unvalidated. Regeneration can use upstream prebuilt
tools or build the gRPC plugin with C++17 and CMake 3.14+.

Regenerate from the authoritative model definitions:

```bash
cd sdks/rust
python3 -m venv .tools/python
.tools/python/bin/pip install -r tools/requirements.txt
python3 tools/bootstrap-protocol-tools.py  # checksum-pinned Linux x86_64 tools
.tools/python/bin/python tools/generate.py \
  --protoc .tools/protoc/bin/protoc --plugin .tools/grpc/bin/protoc-gen-rust-grpc \
  --include .tools/protoc/include
# Repeat with --check to detect drift.
```

The generator records input hashes, copies authoritative coder vectors for
standalone package tests, derives URNs from custom enum options, and applies
the documented `TriggerView::default` qualification workaround. Generated
files include ASF headers. The private Rust FunctionSpec envelope lives in
`proto/beam_rust.proto`; public Beam protos are not forked.

## Checks and packaging

```bash
cd sdks/rust
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
bash tools/test-interop.sh
bash tools/test-prism.sh
python3 tools/test-packages.py
```

Ordinary Cargo tests leave three runner/protocol tests explicitly ignored.
The two scripts run them with their real dependencies. Protocol tests cover
unary/server-streaming/bidirectional calls, worker metadata, status propagation,
cancellation, backpressure and a real mTLS handshake with negative hostname,
trust and identity cases. The protocol suite also calls the checkout's Prism.
A separate Java protocol gate and Java expansion are outside scope.

Package validation extracts the exact Cargo tarballs into `/tmp`, resolves
unpublished sibling crates with local patches, tests them offline, and builds
and runs a separate native consumer. `PROTOC` is set to an unavailable path.
Library tarballs exclude a Cargo lockfile so unpublished siblings can be
resolved; the development workspace retains its checked-in lockfile. Each
package includes LICENSE and NOTICE. Crates are `publish = false`; publication
and ASF release approval are outside this change.

Gradle entrypoints are `:rustPreCommit`, `:sdks:rust:protocolTest`,
`:sdks:rust:prismTest`, `:sdks:rust:packageTest`, and
`:sdks:rust:regenerationCheck`. Rust checks are explicit aggregates and do not
add a Rust prerequisite to unrelated SDK builds. CI runs generation, core
checks, protocol/TLS tests, Prism acceptance and package consumption.

The source model baseline is Beam `1a3f1015445246911b4cd890e3686cc7ad2f2902`.
Prism acceptance requires this change's PROCESS argument and fragmented-data
fixes. No other runner or Beam-version compatibility is inferred. Comparative
performance benchmarks and broader release/distribution work remain optional
follow-ups; no numerical speedup is claimed.
