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

use apache_beam::{Application, DoFn, Kv, ProcessContext, Result, SumI64};

struct Identity;
impl DoFn for Identity {
    type Input = i64;
    type Output = i64;
    type Config = ();
    fn from_config(_: ()) -> Result<Self> {
        panic!("construction must not initialize worker functions")
    }
    fn process(&mut self, value: &i64, context: &mut ProcessContext<'_, i64>) -> Result<()> {
        context.emit(value)
    }
}

#[test]
fn construction_keeps_functions_on_the_worker_and_rejects_unregistered_functions() {
    let app = Application::new("test")
        .register_dofn::<Identity>("test:identity:v1")
        .unwrap();
    let mut p = app.pipeline().unwrap();
    let input = p.create("Input", vec![1_i64]).unwrap();
    p.par_do::<Identity>("Identity", &input, ()).unwrap();
    let unregistered = Application::new("test");
    let mut p = unregistered.pipeline().unwrap();
    let input = p.create("Input", vec![1_i64]).unwrap();
    assert!(p.par_do::<Identity>("Identity", &input, ()).is_err());
}

#[test]
fn handles_and_names_are_checked() {
    let app = Application::new("test");
    let mut left = app.pipeline().unwrap();
    let mut right = app.pipeline().unwrap();
    let input = left.create("Input", vec![1_i64]).unwrap();
    assert!(
        right
            .flatten("Invalid", std::slice::from_ref(&input))
            .is_err()
    );
    assert!(left.create("Input", vec![2_i64]).is_err());
    let a = left
        .flatten("Branch a", std::slice::from_ref(&input))
        .unwrap();
    let b = left.flatten("Branch b", &[input]).unwrap();
    left.flatten("Merge", &[a, b]).unwrap();
}

#[test]
fn grouping_rejects_nondeterministic_keys() {
    let app = Application::new("test");
    let mut p = app.pipeline().unwrap();
    let floats = p.create("Floats", vec![Kv::new(0.0, 1_i64)]).unwrap();
    assert!(p.group_by_key("No float keys", &floats).is_err());
    assert!(
        p.combine_per_key("No float combine", &floats, SumI64)
            .is_err()
    );
    let deterministic = p
        .create("Strings", vec![Kv::new("key".to_owned(), i64::MAX)])
        .unwrap();
    p.combine_per_key("Valid", &deterministic, SumI64).unwrap();
}

#[test]
fn conflicting_or_reserved_registrations_are_rejected() {
    assert!(
        Application::new("test")
            .register_dofn::<Identity>("")
            .is_err()
    );
    assert!(
        Application::new("test")
            .register_dofn::<Identity>("beam:rust:builtin:impersonate:v1")
            .is_err()
    );
    let app = Application::new("test")
        .register_dofn::<Identity>("test:identity:v1")
        .unwrap();
    assert!(app.register_dofn::<Identity>("test:other:v1").is_err());
}

#[test]
fn native_text_io_constructs_without_accessing_worker_files() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("not-created");
    let app = Application::new("test");
    let mut pipeline = app.pipeline().unwrap();
    let input = pipeline
        .read_text("Read", vec![missing.to_str().unwrap().to_owned()])
        .unwrap();
    pipeline
        .write_text("Write", &input, missing.to_str().unwrap())
        .unwrap();
    assert!(!missing.exists());
    assert!(pipeline.read_text("Invalid", vec![String::new()]).is_err());
    assert!(pipeline.write_text("Invalid output", &input, "").is_err());
}
