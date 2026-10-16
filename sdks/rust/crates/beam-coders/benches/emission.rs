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

//! Compare owned output construction with synchronous borrowed encoding.
use beam_coders::{Coder, Context, Value, ValueRef};
use std::hint::black_box;
use std::time::{Duration, Instant};

fn measure(mut run: impl FnMut(), iterations: usize) -> Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        run();
    }
    start.elapsed()
}

fn main() {
    for (label, length, iterations) in [("small", 32, 100_000), ("large", 1024 * 1024, 1_000)] {
        let key = "reused-key".repeat(8);
        let bytes = vec![42; length];
        let coder = Coder::Kv(Box::new(Coder::Utf8), Box::new(Coder::Bytes));
        let byte_view = ValueRef::Bytes(&bytes);
        let view = ValueRef::Kv(&key, &byte_view);
        let expected = coder.encode_ref(view, Context::Nested).unwrap();
        let owned = || {
            Value::Kv(
                Box::new(Value::String(key.clone())),
                Box::new(Value::Bytes(bytes.clone())),
            )
        };
        assert_eq!(coder.encode(&owned(), Context::Nested).unwrap(), expected);
        for _ in 0..100 {
            black_box(coder.encode(&owned(), Context::Nested).unwrap());
            black_box(coder.encode_ref(view, Context::Nested).unwrap());
        }
        let owned_time = measure(
            || {
                black_box(coder.encode(&black_box(owned()), Context::Nested).unwrap());
            },
            iterations,
        );
        let borrowed_time = measure(
            || {
                black_box(coder.encode_ref(black_box(view), Context::Nested).unwrap());
            },
            iterations,
        );
        println!(
            "{label}: {iterations} outputs, owned={owned_time:?}, borrowed={borrowed_time:?}, speedup={:.2}x",
            owned_time.as_secs_f64() / borrowed_time.as_secs_f64()
        );
    }
}
