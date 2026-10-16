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

use crate::{Result, error};
use beam_coders::{Coder, Context, Value};
use beam_model::MonitoringInfo;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Mutex;

#[derive(Default)]
pub(crate) struct Metrics {
    elements: Mutex<BTreeMap<String, i64>>,
    counters: Mutex<BTreeMap<(String, String), i64>>,
}

#[derive(Default)]
pub(crate) struct Catalog(Mutex<BTreeMap<String, MonitoringInfo>>);
impl Catalog {
    pub fn record(&self, infos: Vec<MonitoringInfo>) -> Result<Vec<(String, Vec<u8>)>> {
        let mut catalog = self.0.lock().unwrap();
        let mut result = Vec::new();
        for mut info in infos {
            let labels: BTreeMap<_, _> = info
                .labels()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            let mut hash = Sha256::new();
            for field in [info.urn().to_string(), info.r#type().to_string()]
                .into_iter()
                .chain(labels.into_iter().flat_map(|(k, v)| [k, v]))
            {
                hash.update((field.len() as u64).to_be_bytes());
                hash.update(field.as_bytes());
            }
            let id = format!("{:x}", hash.finalize());
            if catalog.len() >= 16_384 && !catalog.contains_key(&id) {
                return Err(error("worker monitoring metadata limit exceeded"));
            }
            let payload = info.payload().to_vec();
            info.set_payload(&[][..]);
            catalog.entry(id.clone()).or_insert(info);
            result.push((id, payload));
        }
        Ok(result)
    }
    pub fn get(&self, id: &str) -> Option<MonitoringInfo> {
        self.0.lock().unwrap().get(id).cloned()
    }
}

impl Metrics {
    pub fn element(&self, collection: &str) {
        let mut elements = self.elements.lock().unwrap();
        let value = elements.entry(collection.to_owned()).or_default();
        *value = value.wrapping_add(1);
    }
    pub fn counter(&self, transform: &str, name: &str, delta: i64) -> Result<()> {
        if name.is_empty() || name.len() > 256 {
            return Err(error("counter name must contain 1 to 256 bytes"));
        }
        let mut counters = self.counters.lock().unwrap();
        if counters.len() >= 1024
            && !counters.contains_key(&(transform.to_owned(), name.to_owned()))
        {
            return Err(error("too many user counters in bundle"));
        }
        let value = counters
            .entry((transform.to_owned(), name.to_owned()))
            .or_default();
        *value = value.wrapping_add(delta);
        Ok(())
    }
    pub fn infos(&self) -> Vec<MonitoringInfo> {
        let mut result = Vec::new();
        for (collection, value) in self.elements.lock().unwrap().iter() {
            let mut info = info("beam:metric:element_count:v1", *value);
            info.labels_mut().insert("PCOLLECTION", collection);
            result.push(info);
        }
        for ((transform, name), value) in self.counters.lock().unwrap().iter() {
            let mut info = info("beam:metric:user:sum_int64:v1", *value);
            info.labels_mut().insert("PTRANSFORM", transform);
            info.labels_mut().insert("NAMESPACE", "rust");
            info.labels_mut().insert("NAME", name);
            result.push(info);
        }
        result
    }
}

fn info(urn: &str, value: i64) -> MonitoringInfo {
    let mut info = MonitoringInfo::new();
    info.set_urn(urn);
    info.set_type("beam:metrics:sum_int64:v1");
    info.set_payload(
        Coder::VarInt
            .encode(&Value::I64(value), Context::Outer)
            .expect("i64 encoding is infallible"),
    );
    info
}
