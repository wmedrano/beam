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
slice**, including native local text I/O. Docker distribution and Flink are
optional follow-ups; Java expansion and cross-language I/O are outside scope.
The prototype acceptance in [TASK.md](../../TASK.md) is complete locally. The
broader M1/M2 release roadmap remains unfinished; [features.toml](features.toml)
distinguishes those follow-ups from prototype acceptance.

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
| Ordinary Rust tests | 23 passed, plus two compile-fail doctests. Three integration tests are explicitly ignored by this command and run by the scripts below. |
| Standard coders | 111 shared reference cases checked for decoding and canonical encoding, with nested/outer contexts. The exact unsupported coder set is checked. Additional tests cover malformed inputs, bounded decoder smoke fuzzing, frame fragmentation, metadata, and nested nullable presence. |
| Go protocol interoperability | Two integration tests passed against the independent grpc-go Beam API peer and checkout Prism Job service. Includes unary/server/bidirectional streaming, metadata, status, server-observed deadline cancellation, backpressure while an unrelated RPC progresses, and mTLS with negative hostname/trust/client-identity cases. |
| Prism PROCESS | One integration test passed all 23 scenarios below, using real native worker processes and runner shuffles. |
| Packaged crates | All three exact Cargo tarballs were extracted outside Beam, rebuilt and tested offline with local patches for unpublished sibling crates, and consumed by a separately built and executed application. `PROTOC` was unavailable during this check. Normalized archive timestamps cannot reuse stale Beam artifacts; the script clears those artifacts while retaining third-party caches. |
| Dependency inventory | All-feature metadata for 217 external dependencies matches `DEPENDENCIES.csv`; none lack a license field. This is metadata inventory, not an ASF release approval. |
| Regeneration | All 22 generated files match the authoritative inputs, including the required 18 qualified `Default` calls. Shared coder test data also matches the model source. |

The 23 runner scenarios are WordCount; fan-out/Flatten with three 2 MiB byte
elements; CPU-heavy processing; a user error; a process-hook panic; a worker
identity mismatch; an incorrect content assertion; an assertion on empty input;
an incorrect multiplicity assertion; invalid UTF-8 configuration; a factory
panic; a setup error; a setup panic; a finish error; cancellation; an 80 MiB
shuffled group; partial group consumption; chained finish-only emissions; native
UTF-8/CRLF text round trip; an empty file; a missing file; invalid UTF-8; and an
invalid output path. The acceptance app also checks empty collections,
repeated values, wrapping sums,
and distributed assertions. Successful jobs report committed user counters.
Group tests also check independent concurrent iterators, malformed tails,
partial consumption across block/group boundaries, and release of disk budgets.
Harness tests check interleaved inputs while another bundle waits, blocked output
and end-marker acknowledgement, disconnect wakeups, registration limits and
atomicity, finish dependency ordering, and log timestamps/transform context.
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
The CPU and large-element scenarios are correctness checks. Long-running
whole-worker stress, real-runner Register acceptance, hosted CI, and distributed
retry acceptance remain future evidence. Targeted concurrent harness tests pass;
that does not establish sustained distributed operation. The local text sink
provides atomic publication per bundle, with possible duplicate shards on retry.

Prism sends the tested 80 MiB group in one Fn Data message. Fn Data accepts at
most 128 MiB per message; scalar/items and outputs remain capped at 64 MiB.
Group spools share a separate 1 GiB live-file budget, in addition to the 1 GiB
raw input-spool budget. General custom coder URNs, arbitrary pipeline options,
and richer connectors are future native work. Docker, Flink, authenticated
submission, and artifacts have no enabled support. Java expansion, foreign
I/O, portable Rows, and advanced streaming remain outside prototype scope.
