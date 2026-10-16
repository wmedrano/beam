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

use std::collections::BTreeSet;

use beam_coders::{Coder, Context, Limits, Pane, Timing, Value, WindowedValue};
use serde::Deserialize;
use serde_yaml::Value as Yaml;

#[derive(Deserialize)]
struct Spec {
    urn: String,
    #[serde(default)]
    components: Vec<Spec>,
    #[serde(default)]
    non_deterministic: bool,
}

impl Spec {
    fn coder(&self) -> Option<Coder> {
        let components: Option<Vec<Coder>> = self.components.iter().map(Self::coder).collect();
        let components = components?;
        Some(match self.urn.as_str() {
            "beam:coder:bytes:v1" => Coder::Bytes,
            "beam:coder:string_utf8:v1" => Coder::Utf8,
            "beam:coder:bool:v1" => Coder::Bool,
            "beam:coder:varint:v1" => Coder::VarInt,
            "beam:coder:double:v1" => Coder::Double,
            "beam:coder:kv:v1" => Coder::Kv(
                Box::new(components[0].clone()),
                Box::new(components[1].clone()),
            ),
            "beam:coder:iterable:v1" => Coder::Iterable(Box::new(components[0].clone())),
            "beam:coder:nullable:v1" => Coder::Nullable(Box::new(components[0].clone())),
            "beam:coder:length_prefix:v1" => Coder::LengthPrefix(Box::new(components[0].clone())),
            "beam:coder:global_window:v1" => Coder::GlobalWindow,
            "beam:coder:interval_window:v1" => Coder::IntervalWindow,
            "beam:coder:windowed_value:v1" => Coder::Windowed(
                Box::new(components[0].clone()),
                Box::new(components[1].clone()),
            ),
            _ => return None,
        })
    }
}

fn latin1(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| u8::try_from(c as u32).unwrap())
        .collect()
}

fn value(coder: &Coder, yaml: &Yaml) -> Value {
    match coder {
        Coder::Bytes => Value::Bytes(latin1(yaml.as_str().unwrap())),
        Coder::Utf8 => Value::String(yaml.as_str().unwrap().to_owned()),
        Coder::Bool => Value::Bool(yaml.as_bool().unwrap()),
        Coder::VarInt => Value::I64(yaml.as_i64().unwrap()),
        Coder::Double => Value::F64(yaml.as_str().unwrap().parse().unwrap()),
        Coder::Kv(k, v) => Value::Kv(
            Box::new(value(k, &yaml["key"])),
            Box::new(value(v, &yaml["value"])),
        ),
        Coder::Iterable(c) => Value::Iterable(
            yaml.as_sequence()
                .unwrap()
                .iter()
                .map(|y| value(c, y))
                .collect(),
        ),
        Coder::Nullable(_) if yaml.is_null() => Value::Nullable(None),
        Coder::Nullable(c) => Value::Nullable(Some(Box::new(value(c, yaml)))),
        Coder::LengthPrefix(c) => value(c, yaml),
        Coder::GlobalWindow => Value::GlobalWindow,
        Coder::IntervalWindow => Value::IntervalWindow {
            end: yaml["end"].as_i64().unwrap(),
            span: yaml["span"].as_i64().unwrap(),
        },
        Coder::Windowed(c, w) => {
            let pane = &yaml["pane"];
            Value::Windowed(Box::new(WindowedValue {
                value: value(c, &yaml["value"]),
                timestamp: yaml["timestamp"].as_i64().unwrap(),
                windows: yaml["windows"]
                    .as_sequence()
                    .unwrap()
                    .iter()
                    .map(|y| value(w, y))
                    .collect(),
                pane: Pane {
                    is_first: pane["is_first"].as_bool().unwrap(),
                    is_last: pane["is_last"].as_bool().unwrap(),
                    timing: match pane["timing"].as_str().unwrap() {
                        "EARLY" => Timing::Early,
                        "ON_TIME" => Timing::OnTime,
                        "LATE" => Timing::Late,
                        "UNKNOWN" => Timing::Unknown,
                        _ => panic!("timing"),
                    },
                    index: pane["index"].as_i64().unwrap(),
                    on_time_index: pane["on_time_index"].as_i64().unwrap(),
                },
            }))
        }
    }
}

#[test]
fn shared_standard_coder_vectors() {
    let mut checked = 0;
    let mut unsupported = BTreeSet::new();
    for doc in serde_yaml::Deserializer::from_str(include_str!("data/standard_coders.yaml")) {
        let yaml = Yaml::deserialize(doc).unwrap();
        let spec: Spec = serde_yaml::from_value(yaml["coder"].clone()).unwrap();
        let Some(coder) = spec.coder() else {
            unsupported.insert(spec.urn);
            continue;
        };
        let contexts = match yaml["nested"].as_bool() {
            Some(true) => vec![Context::Nested],
            Some(false) => vec![Context::Outer],
            None => vec![Context::Outer, Context::Nested],
        };
        for (encoded, expected) in yaml["examples"].as_mapping().unwrap() {
            let wire = latin1(encoded.as_str().unwrap());
            let expected = value(&coder, expected);
            for context in &contexts {
                let decoded = coder
                    .decode(&wire, *context, Limits::default())
                    .unwrap_or_else(|e| panic!("{}: {e}, {wire:?}", spec.urn));
                // Compare canonical wire bytes, preserving signed zero and NaN
                // payloads, and allowing non-deterministic iterable block forms.
                let canonical = coder.encode(&expected, *context).unwrap();
                assert_eq!(
                    coder.encode(&decoded, *context).unwrap(),
                    canonical,
                    "{}",
                    spec.urn
                );
                if !spec.non_deterministic {
                    assert_eq!(canonical, wire, "{}", spec.urn);
                }
                checked += 1;
            }
        }
    }
    assert_eq!(
        checked, 111,
        "review coverage when the shared model changes"
    );
    eprintln!("{checked} standard coder/context cases; unsupported: {unsupported:?}");
    assert_eq!(
        unsupported,
        [
            "custom_window",
            "param_windowed_value",
            "row",
            "sharded_key",
            "state_backed_iterable",
            "timer"
        ]
        .into_iter()
        .map(|name| format!("beam:coder:{name}:v1"))
        .collect()
    );
}
