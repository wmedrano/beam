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

# Protocol dependency evidence, 2026-10-15

The registry was rechecked during implementation. The newest official `grpc`
release was 0.9.0. Its `grpc-protobuf` adapter selects the official
`protobuf 4.34.0-release` line. The newest protobuf release observed was
`4.36.2-release`; exact matching 34.0 compiler/runtime/WKT pins were used for
the compatibility experiment and retained for the validated fallback.

## Official client gate

Generated client bindings compiled and performed unary provisioning calls,
metadata/status checks and server streaming against grpc-go using Beam's
checked-in Go bindings. Its deadline gate failed: a call configured for 100 ms
remained pending after an independent 500 ms deadline. The original full gate
also timed out at 30 seconds. In the 0.9.0 source,
`grpc/src/client/transport/tonic/mod.rs` accepts `CallOptions` in `invoke` but
does not apply its deadline to the request or wait.

The upstream source is the
[grpc-v0.9.0 transport](https://github.com/grpc/grpc-rust/blob/grpc-v0.9.0/grpc/src/client/transport/tonic/mod.rs).
This prevents relying on the client's advertised timeout for cancellation and
bounded calls. The opt-in regression reproducer intentionally fails with
exit status 101 on this pinned version:

```bash
cd sdks/rust
bash tools/test-interop.sh  # builds the independent Go peer
BEAM_RUST_INTEROP_SERVER="$PWD/target/tools/interop" \
  cargo test -p beam-model --features official-grpc-probe --test official_grpc \
  --locked -- --ignored
```

The reproducer is separate from passing default tests and production transport.
Generated official client bindings remain available only under that feature.
Reevaluate this probe before changing the RPC dependency decision.

## Validated fallback

Tonic 0.14.6 uses `beam-model::codec::OfficialCodec` to serialize and parse the
same official protobuf messages. Service paths, method names, stream directions
and types are generated from descriptors. There is one Beam message model and
no Prost message conversion layer. The adapter has bounded message sizes and
maps serialization failures to gRPC status.

`tools/test-interop.sh` passes with grpc-go and real Beam service definitions:
unary calls, server streaming, bidirectional byte preservation and half-close,
metadata/status, server-observed deadline cancellation, a blocked 64 MiB stream
while an unrelated RPC progresses, and real mutual TLS. Negative hostname,
untrusted-root and missing-client-identity tests pass. A separate test reaches
the job service of Prism built from this checkout. Prism PROCESS acceptance
then exercises real provisioning, Fn control/data/logging, descriptor lookup,
metrics metadata lookup and job lifecycle with native workers.

This establishes the local experimental path and transport adapter. Java
interop, authenticated SDK deployment, Docker/Flink, foreign environments and
distributed recovery remain separate gates.

## Protobuf generator workaround

Unmodified protoc 34.0 output fails with E0061/E0308 for Beam's `Trigger`.
Generated calls to `TriggerView::default()` bind to the `default` field getter
instead of `std::default::Default`. The generator replaces exactly 18 such
calls with the fully qualified trait call and fails if this count changes.
It does not rename fields or alter the public model. `messages.rs` tests access
through an unset trigger and verifies well-known types and unknown-field
retention after known fields are edited. URN constants come from dynamic
descriptor interpretation of the `beam_urn` extension.

## Earlier SDK review

The [Beam Summit 2023 material](https://beamsummit.org/slides/2023/Sho-Rust.pdf)
and [laysakura/beam rust_sdk branch](https://github.com/laysakura/beam/tree/rust_sdk/sdks/rust)
were reviewed. The repository reports Apache-2.0 licensing; its inspected
manifest uses Prost 0.11, Tonic 0.8 and constructor-based registration. GitHub
reported its last push as 2023-07-03. These dependencies and identity mechanism
need migration for the present contracts. No code was copied from that fork.
