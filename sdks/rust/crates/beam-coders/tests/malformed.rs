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

use beam_coders::{Coder, Context, Error, Limits, Pane, Timing, Value, WindowedValue};

#[test]
fn nullable_keeps_nested_presence_distinct() {
    let coder = Coder::Nullable(Box::new(Coder::Nullable(Box::new(Coder::VarInt))));
    for (wire, value) in [
        (vec![0], Value::Nullable(None)),
        (
            vec![1, 0],
            Value::Nullable(Some(Box::new(Value::Nullable(None)))),
        ),
        (
            vec![1, 1, 42],
            Value::Nullable(Some(Box::new(Value::Nullable(Some(Box::new(Value::I64(
                42,
            ))))))),
        ),
    ] {
        assert_eq!(
            coder
                .decode(&wire, Context::Nested, Limits::default())
                .unwrap(),
            value
        );
        assert_eq!(coder.encode(&value, Context::Nested).unwrap(), wire);
    }
}

#[test]
fn rejects_bad_lengths_counts_and_noncanonical_types() {
    let limits = Limits {
        bytes: 16,
        items: 10,
        depth: 4,
    };
    assert_eq!(
        Coder::Bytes.decode(&[255; 10], Context::Nested, limits),
        Err(Error::Invalid("varint exceeds 64 bits"))
    );
    assert!(matches!(
        Coder::Bytes.decode(&[100], Context::Nested, limits),
        Err(Error::Limit(_))
    ));
    assert!(
        Coder::Utf8
            .decode(&[1, 255], Context::Nested, limits)
            .is_err()
    );
    assert!(Coder::Bool.decode(&[2], Context::Nested, limits).is_err());
    assert!(
        Coder::Nullable(Box::new(Coder::Bytes))
            .decode(&[2], Context::Nested, limits)
            .is_err()
    );
    assert!(
        Coder::Iterable(Box::new(Coder::GlobalWindow))
            .decode(&[127, 255, 255, 255], Context::Nested, limits)
            .is_err()
    );
    assert!(
        Coder::Iterable(Box::new(Coder::Bytes))
            .decode(
                &[
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1
                ],
                Context::Nested,
                limits
            )
            .is_err()
    );
    assert!(
        Coder::Iterable(Box::new(Coder::Bytes))
            .decode(&[255, 255, 255, 254], Context::Nested, limits)
            .is_err()
    );
    assert!(!Coder::Double.deterministic_key());
    assert!(!Coder::Kv(Box::new(Coder::Utf8), Box::new(Coder::Double)).deterministic_key());
    assert!(Coder::Kv(Box::new(Coder::Utf8), Box::new(Coder::VarInt)).deterministic_key());
}

#[test]
fn fragmented_and_coalesced_windowed_values_preserve_metadata() {
    let coder = Coder::Windowed(Box::new(Coder::Utf8), Box::new(Coder::GlobalWindow));
    let value = Value::Windowed(Box::new(WindowedValue {
        value: Value::String("光线".to_owned()),
        timestamp: -1234,
        windows: vec![Value::GlobalWindow],
        pane: Pane {
            is_first: false,
            is_last: true,
            timing: Timing::Late,
            index: 30,
            on_time_index: 27,
        },
    }));
    let bytes = coder.encode(&value, Context::Nested).unwrap();
    for split in 0..bytes.len() {
        assert_eq!(
            coder.decode_prefix(&bytes[..split], Context::Nested, Limits::default()),
            Err(Error::NeedMore)
        );
    }
    let coalesced = bytes.repeat(3);
    let mut consumed = 0;
    for _ in 0..3 {
        let (decoded, size) = coder
            .decode_prefix(&coalesced[consumed..], Context::Nested, Limits::default())
            .unwrap();
        assert_eq!(decoded, value);
        consumed += size;
    }
    assert_eq!(consumed, coalesced.len());
    assert!(
        coder
            .decode(&coalesced, Context::Nested, Limits::default())
            .is_err()
    );
}

#[test]
fn malformed_input_smoke_fuzz_is_bounded_and_never_panics() {
    let coders = [
        Coder::VarInt,
        Coder::Utf8,
        Coder::Iterable(Box::new(Coder::Bytes)),
        Coder::Windowed(Box::new(Coder::VarInt), Box::new(Coder::GlobalWindow)),
    ];
    let mut seed = 0x1234_5678_u64;
    for length in 0..128 {
        for _ in 0..32 {
            let bytes: Vec<u8> = (0..length)
                .map(|_| {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    seed as u8
                })
                .collect();
            for coder in &coders {
                let _ = coder.decode(
                    &bytes,
                    Context::Nested,
                    Limits {
                        bytes: 128,
                        items: 16,
                        depth: 8,
                    },
                );
            }
        }
    }
}
