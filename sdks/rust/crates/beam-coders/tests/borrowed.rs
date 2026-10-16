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

use beam_coders::{Coder, Context, Error, Limits, Pane, Value, ValueRef, ValueView};
use std::cell::Cell;

struct Observed<'a> {
    bytes: &'a [u8],
    visits: &'a Cell<usize>,
}
impl ValueView for Observed<'_> {
    fn value_ref(&self) -> ValueRef<'_> {
        self.visits.set(self.visits.get() + 1);
        ValueRef::Bytes(self.bytes)
    }
}

#[test]
fn encoder_traverses_borrowed_children_without_materializing_payloads() {
    let bytes = vec![42; 1024 * 1024];
    let visits = Cell::new(0);
    let observed = Observed {
        bytes: &bytes,
        visits: &visits,
    };
    let key = "borrowed key".value_ref();
    let kv = ValueRef::Kv(&key, &observed);
    let windows = [Value::GlobalWindow];
    let windowed = ValueRef::Windowed {
        value: &kv,
        timestamp: -1234,
        windows: &windows,
        pane: Pane::default(),
    };
    let coder = Coder::Windowed(
        Box::new(Coder::Kv(Box::new(Coder::Utf8), Box::new(Coder::Bytes))),
        Box::new(Coder::GlobalWindow),
    );
    for context in [Context::Outer, Context::Nested] {
        let encoded = coder.encode_ref(windowed, context).unwrap();
        assert_eq!(
            encoded,
            coder
                .encode(&windowed.to_owned().unwrap(), context)
                .unwrap()
        );
        let decoded = coder.decode(&encoded, context, Limits::default()).unwrap();
        assert_eq!(decoded, windowed.to_owned().unwrap());
    }
    // Each direct encode visits the original child once. The materialization
    // checks above each visit once too; none of them hide an eager conversion.
    assert_eq!(visits.get(), 6);
    assert_eq!(observed.bytes.as_ptr(), bytes.as_ptr());
}

#[test]
fn borrowed_composites_round_trip_and_preserve_nested_nullable_presence() {
    let items = vec![Some("first".to_owned()), None, Some(String::new())];
    let coder = Coder::Iterable(Box::new(Coder::Nullable(Box::new(Coder::Utf8))));
    let nested = Some(Some("nested".to_owned()));
    let nested_coder = Coder::Nullable(Box::new(Coder::Nullable(Box::new(Coder::Utf8))));
    for (coder, view) in [
        (coder, items.value_ref()),
        (nested_coder, nested.value_ref()),
    ] {
        for context in [Context::Outer, Context::Nested] {
            let owned = view.to_owned().unwrap();
            let bytes = coder.encode_ref(view, context).unwrap();
            assert_eq!(bytes, coder.encode(&owned, context).unwrap());
            assert_eq!(
                coder.decode(&bytes, context, Limits::default()).unwrap(),
                owned
            );
        }
    }
}

#[test]
fn borrowed_encoding_keeps_type_errors_and_depth_limits() {
    assert_eq!(
        Coder::Utf8.encode_ref(ValueRef::I64(1), Context::Nested),
        Err(Error::TypeMismatch)
    );
    let mut coder = Coder::VarInt;
    for _ in 0..65 {
        coder = Coder::LengthPrefix(Box::new(coder));
    }
    assert_eq!(
        coder.encode_ref(ValueRef::I64(1), Context::Nested),
        Err(Error::Limit("coder depth"))
    );
    let items = vec![1_i64, 2];
    let result = items
        .value_ref()
        .try_for_each::<Error>(|_| Err(Error::Invalid("visitor failure")));
    assert_eq!(result, Err(Error::Invalid("visitor failure")));
}
