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

use apache_beam::{BeamValue, Grouped, Kv};
use beam_coders::{Coder, Context, Limits, SpoolBudget, Value, ValueRef, ValueView};
use std::io::Cursor;

fn decode(
    bytes: &[u8],
    coder: &Coder,
    budget: SpoolBudget,
    limits: Limits,
) -> Result<Value, beam_coders::Error> {
    coder.read_spooled(
        &mut Cursor::new(bytes),
        &mut tempfile::tempfile,
        budget,
        limits,
    )
}

#[test]
fn groups_exceed_item_limit_and_support_independent_partial_iteration() {
    let coder = Coder::Iterable(Box::new(Coder::Utf8));
    let expected: Vec<_> = (0..100)
        .map(|i| Value::String(format!("{i:04}{}", "x".repeat(96))))
        .collect();
    let bytes = coder
        .encode(&Value::Iterable(expected.clone()), Context::Nested)
        .unwrap();
    let limits = Limits {
        bytes: 256,
        ..Limits::default()
    };
    assert!(coder.decode(&bytes, Context::Nested, limits).is_err());
    let budget = SpoolBudget::new(64 * 1024);
    let value = decode(&bytes, &coder, budget.clone(), limits).unwrap();
    assert!(budget.used() > limits.bytes);
    let group = Grouped::<String>::from_value(&value).unwrap();
    let mut first = group.iter().unwrap();
    assert_eq!(
        first.next().unwrap().unwrap(),
        "0000".to_owned() + &"x".repeat(96)
    );
    assert_eq!(group.iter().unwrap().count(), 100);
    assert_eq!(first.count(), 99);
    std::thread::scope(|scope| {
        let a = scope.spawn(|| {
            group
                .iter()
                .unwrap()
                .collect::<apache_beam::Result<Vec<_>>>()
                .unwrap()
        });
        let b = scope.spawn(|| {
            group
                .iter()
                .unwrap()
                .collect::<apache_beam::Result<Vec<_>>>()
                .unwrap()
        });
        assert_eq!(a.join().unwrap(), b.join().unwrap());
    });
    assert_eq!(coder.encode(&value, Context::Nested).unwrap(), bytes);
    drop(group);
    drop(value);
    assert_eq!(budget.used(), 0);
}

#[test]
fn partial_consumption_preserves_next_group_and_block_boundaries() {
    let coder = Coder::Iterable(Box::new(Coder::VarInt));
    let mut bytes = vec![255, 255, 255, 255, 2, 10, 11, 1, 12, 0];
    bytes.extend_from_slice(&[0, 0, 0, 1, 99]);
    let budget = SpoolBudget::new(1024);
    let mut reader = Cursor::new(bytes);
    let first = coder
        .read_spooled(
            &mut reader,
            &mut tempfile::tempfile,
            budget.clone(),
            Limits::default(),
        )
        .unwrap();
    assert_eq!(
        first.iterable().unwrap().next().unwrap().unwrap(),
        Value::I64(10)
    );
    let second = coder
        .read_spooled(
            &mut reader,
            &mut tempfile::tempfile,
            budget.clone(),
            Limits::default(),
        )
        .unwrap();
    assert_eq!(
        second
            .iterable()
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![Value::I64(99)]
    );
    assert_eq!(
        first
            .iterable()
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![Value::I64(10), Value::I64(11), Value::I64(12)]
    );
    drop((first, second));
    assert_eq!(budget.used(), 0);
}

#[test]
fn malformed_tail_and_exhausted_budget_release_spools() {
    let coder = Coder::Iterable(Box::new(Coder::Utf8));
    let budget = SpoolBudget::new(1024);
    for bytes in [
        &[0, 0, 0, 2, 1, b'a', 1, 255][..],
        &[0, 0, 0, 2, 1, b'a', 5, b'b'][..],
    ] {
        assert!(decode(bytes, &coder, budget.clone(), Limits::default()).is_err());
        assert_eq!(budget.used(), 0);
    }
    let tiny = SpoolBudget::new(16);
    assert!(
        decode(
            &[0, 0, 0, 2, 1, b'a', 1, b'b'],
            &coder,
            tiny.clone(),
            Limits::default()
        )
        .is_err()
    );
    assert_eq!(tiny.used(), 0);
}

#[test]
fn borrowed_keyed_group_inputs_retain_shared_spools_without_materializing() {
    let coder = Coder::Iterable(Box::new(Coder::Utf8));
    let expected = Value::Iterable(
        (0..32)
            .map(|i| Value::String(format!("item-{i}")))
            .collect(),
    );
    let encoded = coder.encode(&expected, Context::Nested).unwrap();
    let budget = SpoolBudget::new(64 * 1024);
    let value = decode(&encoded, &coder, budget.clone(), Limits::default()).unwrap();
    let key = "borrowed".value_ref();
    let input = Kv::<String, Grouped<String>>::from_ref(ValueRef::Kv(&key, &value)).unwrap();
    let retained = budget.used();
    assert!(retained > 0);
    drop(value);
    assert_eq!(budget.used(), retained);
    assert_eq!(input.key, "borrowed");
    let mut partial = input.value.iter().unwrap();
    assert_eq!(partial.next().unwrap().unwrap(), "item-0");
    assert_eq!(input.value.iter().unwrap().count(), 32);
    assert_eq!(partial.count(), 31);
    drop(input);
    assert_eq!(budget.used(), 0);
}
