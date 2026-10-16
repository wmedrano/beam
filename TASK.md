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

Implement the native Rust SDK described in
[contributor-docs/rust-sdk-design.md](contributor-docs/rust-sdk-design.md).
Deliver the code, documentation, build integration, and tests for milestones
M0 through M2: bounded pipelines on Prism, application worker containers,
Flink portable batch execution, and cross-language I/O. M1 is the first working
vertical slice; M2 is the completion target for this task. Streaming and the
advanced features in M3/M4 are follow-up work.

The repository root is `beam/`; paths and commands below are relative to it.
The design was researched against commit `1a3f101544` on 2026-10-15. Read the
current repository instructions and implementation before making changes.
Preserve the existing design and its contributor documentation index entry,
including any uncommitted changes. Update the design when implementation
evidence requires a change and explain the reason.

## Decisions to carry forward

- Build on the existing Runner, Job, Fn, Provision, Artifact, and Expansion
  APIs. Reuse portable runners for scheduling, shuffles, watermarks, and recovery.
- The user specifically raised using the `protobuf` crate and preferring an
  official RPC implementation. Prefer the official `protocolbuffers/protobuf`
  4.x runtime. The older community `protobuf` 3.x is a different implementation;
  do not select it just because a bare crate name resolves to it.
- Evaluate the official `grpc` client first. Recheck its current releases and
  compatibility rather than treating the design's preview version as a pin.
  Keep generated RPC types out of the ordinary public SDK API.
- The harness initiates its Fn API streams as a gRPC client. Start with PROCESS
  locally and DOCKER for distribution; neither needs an SDK-side gRPC server.
  EXTERNAL worker-pool and Rust expansion servers are deferred.
- If the official client fails the M0 gate, record the failure and evaluate
  Tonic with a private codec/bindings for official protobuf messages. Prove that
  adapter works. Do not silently replace the preferred message runtime with
  `prost` or maintain parallel model representations throughout the SDK.
- Compile application functions into the worker binary and use explicit,
  versioned registrations. Transmit function identity and configuration;
  reconstruct resources during setup. Do not serialize closures, object memory,
  TypeId, addresses, or a Rust dynamic-library ABI.
- Use standard Beam transform and coder encodings. Keep proposed Rust-specific
  payloads inside SDK-specific FunctionSpecs and private protobuf definitions.
- Ordinary Rust construction and worker execution must work without a Python
  or JVM runtime. Foreign connectors may use their own language environments.

## Read first

The design contains the complete architectural rationale, proposed traits,
WordCount API sketch, ownership rules, and feature boundaries. Its snippets
are proposals, not existing executable APIs. Refine names and ergonomics as
needed while preserving the contracts above.

| Area | Source |
| --- | --- |
| Pipeline representation, URNs, coders, environments, requirements | [beam_runner_api.proto](model/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/beam_runner_api.proto) |
| Schemas and external transform configuration | [schema.proto](model/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/schema.proto), [external_transforms.proto](model/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/external_transforms.proto) |
| Job submission and lifecycle | [beam_job_api.proto](model/job-management/src/main/proto/org/apache/beam/model/job_management/v1/beam_job_api.proto) |
| Artifact staging and retrieval | [beam_artifact_api.proto](model/job-management/src/main/proto/org/apache/beam/model/job_management/v1/beam_artifact_api.proto) |
| Bundle execution, streaming RPCs, logging and state | [beam_fn_api.proto](model/fn-execution/src/main/proto/org/apache/beam/model/fn_execution/v1/beam_fn_api.proto) |
| Worker provisioning | [beam_provision_api.proto](model/fn-execution/src/main/proto/org/apache/beam/model/fn_execution/v1/beam_provision_api.proto) |
| Expansion and schema transform discovery | [beam_expansion_api.proto](model/job-management/src/main/proto/org/apache/beam/model/job_management/v1/beam_expansion_api.proto) |
| Coder reference vectors | [standard_coders.yaml](model/fn-execution/src/main/resources/org/apache/beam/model/fnexecution/v1/standard_coders.yaml) |
| Go harness translation | [translate.go](sdks/go/pkg/beam/core/runtime/exec/translate.go) |
| Container startup | [Go boot.go](sdks/go/container/boot.go) |
| Portable submission and expansion reference | [TypeScript SDK](sdks/typescript/src/apache_beam) |
| Prism environment startup | [environments.go](sdks/go/pkg/beam/runners/prism/internal/environments.go) |
| Existing Python/Rust integration | [wordcount_rust](sdks/python/apache_beam/examples/wordcount_rust/README.md) |

Use the checked-in protocol definitions as the source of truth. Older READMEs
may describe Prism limitations that its implementation has since addressed.
Review the earlier Rust SDK work linked in the design before duplicating it;
verify compatibility, maintenance status, and licensing before reusing code.

## M0: establish protocol and dependency compatibility

- [ ] Create a Cargo workspace under `sdks/rust/`, with a pinned contributor
  toolchain, declared MSRV, lockfile, and the crate boundaries in the design.
  Add components when needed rather than creating empty placeholder crates.
- [ ] Select and record an exact compatible protobuf compiler/runtime,
  well-known types package, RPC library, and service generator. Document native
  compiler and upb/kernel requirements, including Linux worker builds.
- [ ] Generate Beam protocol messages and client bindings from the repository's
  `model/` definitions. Generate standard URN constants from model options.
  Keep private Rust payload definitions under `sdks/rust/proto/`.
- [ ] Check in generated sources and required native glue, with reproducible
  regeneration and license handling. Package all necessary inputs so consuming
  a crate does not need the Beam checkout or a code generator.
- [ ] Prove interoperability with real Go/Java Beam services, including unary,
  server-streaming, and bidirectional calls; worker metadata, status propagation,
  cancellation, backpressure, and TLS configuration. Add a real TLS handshake
  test before claiming authenticated remote support.
- [ ] Test well-known types, `beam_urn` options, and unknown-field retention.
  Retention does not imply support for unknown required semantics.
- [ ] Implement the standard coders needed by M1, consuming the shared reference
  vectors in nested and outer contexts. Explicitly identify unsupported coders.

Complete this gate before investing in the larger public API. If a dependency
cannot satisfy it, leave a reproducible failing example and an evidence-based
decision record. An actual external blocker must be reported with its precise
dependency and impact; do not hide it behind stubbed protocol behavior.

## M1: run a native bounded pipeline on Prism

- [ ] Implement Pipeline, typed PCollection handles, component registration,
  default/explicit coder selection, and graph validation. Reject handles from
  different pipelines and incompatible coder or function contracts.
- [ ] Add the application entrypoint and explicit DoFn/coder factory registry.
  Driver and worker modes initialize the same registry; worker mode skips
  pipeline construction. Separate serializable configuration from resources.
- [ ] Implement lifecycle hooks, borrowed immutable inputs, owned outputs,
  timestamp/window/pane preservation, and bundle-local function instances.
- [ ] Implement Create via Impulse plus a Rust decoding ParDo, single-output
  ParDo, Flatten, GroupByKey, and keyed combine lowered to grouping plus
  reduction. SumI64 must have partition-independent arithmetic as in the design.
  Do not advertise optimizable combines until their lifted stages work.
- [ ] Validate deterministic key coders. Do not use arbitrary protobuf encoding,
  floats, or unordered maps as implicit key encodings.
- [ ] Support global windows/default triggering/zero lateness. Decode complete
  windowed values and both known-length and block iterable encodings. Grouped
  values need fallible iteration and bounded buffering/spooling for large keys.
- [ ] Implement PROCESS boot/provisioning and Fn control/data/logging. Fetch
  bundle descriptors with GetProcessBundleDescriptor and support Register where
  the tested runner requires compatibility.
- [ ] Execute the runner's descriptor, including source/sink RemoteGrpcPorts.
  Handle fused stages, multiple inputs, fan-out, and interleaved bundles.
  Decode elements across data-message boundaries and close every output stream
  before successfully completing its bundle.
- [ ] Keep RPC/control processing responsive during CPU work. Bound compute
  concurrency and buffered bytes; prevent one blocked bundle from starving
  unrelated streams. Implement progress, element/user metrics, and useful logs.
- [ ] Propagate user/transport errors. Discard processors after panics, avoid
  replaying active bundles on reconnect, and leave retries/commit decisions to
  the runner. Reject unsupported instructions/features explicitly.
- [ ] Implement the Job API client needed to submit and observe PROCESS jobs,
  including options, state/messages, metrics, waiting, and cancellation. M1 can
  use a locally available executable with no staged application dependencies;
  M2 extends this path to complete artifact and foreign-environment handling.
- [ ] Add a real WordCount example and distributed content/multiplicity
  assertions. Run it against the checked-out Prism through a genuine shuffle.

M1 acceptance includes empty input, repeated values, fan-out, concurrent bundles,
fragmented/coalesced data frames, partial iterable consumption, malformed input,
missing registrations, incompatible configuration, mismatched worker identity,
and failure propagation. A local iterator simulation or mocked runner alone
does not satisfy this milestone. Build Prism from this checkout for integration
tests and retain any demonstrated runner bug fixes as isolated changes.

## M2: complete experimental distribution and cross-language I/O

- [ ] Build application worker images with boot entrypoint and Linux executable.
  Use standard Docker environments and immutable image references. Keep driver
  and worker build identity consistent across target platforms.
- [ ] Finish artifact staging/retrieval using returned endpoints/tokens and
  reverse retrieval. Add legacy support only where a tested runner needs it.
  Preserve foreign dependencies. If downloading worker executables, use FILE
  artifacts and the staging_to role, with manifest/checksum/path validation.
- [ ] Implement the documented Row/schema subset with nullability, nested Rows,
  field encoding positions, and checked numeric conversions. Reject unsupported
  logical types with field-specific errors.
- [ ] Add Expansion API clients and schema transform discovery/configuration.
  Merge all components/references/requirements, retain foreign environments and
  dependencies, and resolve deferred artifacts. Verify returned output coders;
  requested output coders are only hints.
- [ ] Choose and document a bounded Java/Python connector pair for acceptance.
  Test a foreign read -> Rust transform -> foreign write pipeline, plus a
  foreign Row -> Rust -> foreign Row round trip.
- [ ] Run bounded examples on Prism DOCKER and the Flink portable runner.
  Test cancellation, worker failure/retry on a suitable runner, artifact
  integrity, and authenticated transport where claimed. Do not infer support
  for another runner or managed service from one successful portable job.
- [ ] Add Cargo checks, Gradle sdks:rust wrappers/root precommit integration,
  and appropriate Prism/Docker/Flink CI jobs. Keep unrelated builds unaffected.
- [ ] Supply a quickstart, runnable examples, dependency/MSRV regeneration
  instructions, deployment guide, and an explicit SDK/runner feature matrix.
- [ ] Verify packaged crates from outside the workspace, maintain ASF headers
  and dependency/container notices, and add narrowly scoped generated-file
  exclusions where required. Prepare release metadata; publication is not part
  of this task.

## Validation and completion criteria

Use meaningful unit tests, shared coder vectors, protocol tests, compile-fail
API examples, and real runner jobs. Fake services should exercise adversarial
streaming/error conditions without replacing integration tests. Fuzz binary
decoders and malformed descriptors where appropriate. Run the checks relevant
to each change and record exact commands and results.

Provide these Cargo entrypoints or document an equally direct equivalent:

```bash
cargo fmt --manifest-path sdks/rust/Cargo.toml --all -- --check
cargo clippy --manifest-path sdks/rust/Cargo.toml --workspace --all-targets --locked -- -D warnings
cargo test --manifest-path sdks/rust/Cargo.toml --workspace --locked
```

Also provide reproducible commands for protocol regeneration, packaged-crate
validation, Prism PROCESS/DOCKER WordCount, the Flink job, and cross-language
acceptance. Mark prerequisites and make missing optional infrastructure explicit;
never report a skipped integration test as passing. Keep capabilities disabled
until their implementations and acceptance tests pass.

Record initial performance baselines for WordCount, CPU-heavy processing,
large-element transfer, and skewed aggregation against an equivalent existing
SDK on the same runner. Include throughput, startup/build time, memory, and
backpressure observations; do not claim an unmeasured speedup.

The implementation is complete for this task when M0-M2 acceptance passes, the
published support matrix matches actual tests, and a contributor can reproduce
the examples from the documentation. Report changed behavior, dependency
decisions, exact verification results, and remaining limitations. Distinguish
implemented functionality from the M3/M4 roadmap. Do not stop at generated
bindings, an API skeleton, or an unverified "works on portable runners" claim.

## Deferred work

M3/M4 cover event-time streaming/windowing beyond the initial global strategy,
side inputs, multiple outputs, state-backed iterables, durable user state/timers,
bundle finalization, SDF/restriction tracking, dynamic splitting, EXTERNAL pools,
and Rust expansion servers. Preserve architectural room for these features,
but keep their APIs and capability claims out of the initial experimental scope.
Imported foreign transforms can use advanced features in their own harnesses
without implying the Rust harness implements them.
