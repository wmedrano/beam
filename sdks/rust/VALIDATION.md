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

# Prism slice validation, 2026-10-15

The agreed scope of this change is a tested native **Prism PROCESS vertical
slice**. Docker distribution, Flink, and cross-language I/O are follow-up work.
This record does not mark the original M0–M2 milestones complete; the remaining
gates are listed in [features.toml](features.toml).

Tests used Linux x86_64, Rust 1.96.1, Go 1.26.2, Python 3.14.8 with
protobuf 7.36.1, and protoc 34.0. Gradle checks used Java 21.0.12.1+1.
Python/protoc are regeneration tools, Go builds the test peers and runner,
and Java runs Gradle; native applications do not depend on these runtimes.
The model baseline is Beam `1a3f1015445246911b4cd890e3686cc7ad2f2902`, with
this change's two isolated Prism fixes: pass the PROCESS worker ID without
shell quotes, and decode output elements across Fn Data message boundaries.

## Rust, protocol, runner, and package checks

Commands below run from `sdks/rust`, with the tools above on `PATH`.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
bash tools/test-interop.sh
bash tools/test-prism.sh
python3 tools/test-packages.py
python3 tools/dependencies.py --check
python3 tools/generate.py --check \
  --protoc .tools/protoc/bin/protoc \
  --plugin .tools/grpc/bin/protoc-gen-rust-grpc \
  --include .tools/protoc/include
```

All commands passed. The regeneration tools were installed using
`tools/bootstrap-protocol-tools.py`; see the [README](README.md) for setup.

| Check | Observed result |
| --- | --- |
| Formatting and Clippy | Passed, including all-target/all-feature Clippy with warnings denied. |
| Ordinary Rust tests | 15 passed, plus two compile-fail doctests. Three integration tests are explicitly ignored by this command and run by the scripts below. |
| Standard coders | 111 shared reference cases checked for decoding and canonical encoding, with nested/outer contexts. The exact unsupported coder set is checked. Additional tests cover malformed inputs, bounded decoder smoke fuzzing, frame fragmentation, metadata, and nested nullable presence. |
| Go protocol interoperability | Two integration tests passed against the independent grpc-go Beam API peer and checkout Prism Job service. Includes unary/server/bidirectional streaming, metadata, status, server-observed deadline cancellation, backpressure while an unrelated RPC progresses, and mTLS with negative hostname/trust/client-identity cases. |
| Prism PROCESS | One integration test passed all 15 scenarios below, using real native worker processes and runner shuffles. |
| Packaged crates | All three exact Cargo tarballs were extracted outside Beam, tested offline with local patches for unpublished sibling crates, and consumed by a separately built and executed application. `PROTOC` was unavailable during this check. |
| Dependency inventory | All-feature metadata for 217 external dependencies matches `DEPENDENCIES.csv`; none lack a license field. This is metadata inventory, not an ASF release approval. |
| Regeneration | All 22 generated files match the authoritative inputs, including the required 18 qualified `Default` calls. Shared coder test data also matches the model source. |

The 15 runner scenarios are WordCount; fan-out/Flatten with three 2 MiB byte
elements; CPU-heavy processing; a user error; a process-hook panic; a worker
identity mismatch; an incorrect content assertion; an assertion on empty input;
an incorrect multiplicity assertion; invalid UTF-8 configuration; a factory
panic; a setup error; a setup panic; a finish error; and cancellation. The
acceptance app also checks empty collections, repeated values, wrapping sums,
and distributed assertions. Successful jobs report committed user counters.
The cancellation case observes terminal `CANCELLED` after the driver's request.
Expected failure cases check their diagnostics and that Prism remains alive.

## Go and Gradle integration

From the repository root:

```bash
go -C sdks test ./go/pkg/beam/runners/prism/internal/... -timeout 180s
./gradlew :sdks:rust:tasks --all --console=plain
./gradlew :rustPreCommit :sdks:rust:regenerationCheck --console=plain
git diff --check
```

All passed. The Go suite includes a real helper-process argument test, output
decoding at every two-chunk split, and one-byte fragments. Gradle discovers
the Rust tasks and runs the core checks and regeneration check. The added
GitHub Actions workflow invokes protocol, runner, regeneration, package, and
inventory checks; hosted CI execution has not been observed locally.

`./gradlew :rat --console=plain` found nine pre-existing local `.jj` bookkeeping
files with unknown licenses. Repeating the audit with a temporary, uncommitted
Gradle init script excluding only `.jj/**` passed. No repository-wide `.jj`
license exclusion was added. The exact local audit was:

```groovy
// /tmp/beam-rust-rat-local.init.gradle
gradle.projectsEvaluated {
  rootProject.tasks.findByName('rat')?.exclude('.jj/**')
}
```

```bash
./gradlew -I /tmp/beam-rust-rat-local.init.gradle :rat --console=plain
```

## Dependency gate and remaining evidence

The opt-in official `grpc 0.9.0` deadline reproducer fails with exit status 101:
its 100 ms call deadline still has not completed after 500 ms. This is an
expected failing dependency gate, excluded from the passing default suite.
[protocol-decisions.md](protocol-decisions.md) records the command, upstream
implementation evidence, and the tested Tonic adapter for the same official
protobuf messages.

No comparative throughput, memory, or startup benchmark has been established.
The CPU and large-element scenarios are correctness checks. Java protocol
interop, sustained adversarial/concurrent harness stress, lazy fallible grouped
iteration and hot-key spooling, Register acceptance, authenticated SDK
deployment, and worker retry acceptance remain unverified. Docker, Flink,
artifacts, Rows, expansion, foreign I/O, and advanced streaming features have
no enabled support in this slice.
