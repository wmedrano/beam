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

use beam_model::*;
use protobuf::prelude::*;

#[test]
fn unknown_fields_survive_known_field_edits() {
    // Unknown field 100 (varint), 101 (bytes), followed by a known string.
    let wire = b"\xa0\x06\x96\x01\xaa\x06\x03new\x0a\x03old";
    let mut spec = FunctionSpec::parse(wire).unwrap();
    assert_eq!(spec.urn(), "old");
    spec.set_urn("changed");
    let encoded = spec.serialize().unwrap();
    assert!(encoded.windows(4).any(|w| w == b"\xa0\x06\x96\x01"));
    assert!(encoded.windows(6).any(|w| w == b"\xaa\x06\x03new"));
    assert_eq!(FunctionSpec::parse(&encoded).unwrap().urn(), "changed");
}

#[test]
fn well_known_types_and_default_trigger() {
    let mut event = JobStateEvent::new();
    event.timestamp_mut().set_seconds(123);
    event.timestamp_mut().set_nanos(456);
    let decoded = JobStateEvent::parse(&event.serialize().unwrap()).unwrap();
    assert_eq!(decoded.timestamp().seconds(), 123);
    assert_eq!(decoded.timestamp().nanos(), 456);

    let mut provision = GetProvisionInfoResponse::new();
    let mut value = protobuf_well_known_types::Value::new();
    value.set_number_value(42.0);
    provision
        .info_mut()
        .pipeline_options_mut()
        .fields_mut()
        .insert("parallelism", value);
    let decoded = GetProvisionInfoResponse::parse(&provision.serialize().unwrap()).unwrap();
    assert_eq!(
        decoded
            .info()
            .pipeline_options()
            .fields()
            .get("parallelism")
            .unwrap()
            .number_value(),
        42.0
    );

    // Regression for protoc's Default/default field-name collision.
    let strategy = WindowingStrategy::new();
    let _ = strategy.trigger().default();
}

#[test]
fn urns_come_from_custom_options() {
    assert_eq!(urns::STANDARD_CODERS_ENUM_VARINT, "beam:coder:varint:v1");
    assert_eq!(
        urns::STANDARD_PTRANSFORMS_PRIMITIVES_GROUP_BY_KEY,
        "beam:transform:group_by_key:v1"
    );
    assert!(urns::ALL.len() > 100);
}

#[test]
fn malformed_messages_are_rejected() {
    assert!(FunctionSpec::parse(b"\x0a\xff").is_err());
    assert!(GetProvisionInfoResponse::parse(b"\x0a\x04\x0a").is_err());
}
