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

//! Intentionally failing, opt-in upstream deadline regression reproducer.
#![cfg(feature = "official-grpc-probe")]

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::Duration;

use beam_model::GetJobStateRequest;
use beam_model::official_grpc::job_service_client::JobServiceClient;
use grpc::client::{Channel, ChannelOptions};
use grpc::credentials::LocalChannelCredentials;
use grpc_protobuf::CallBuilder;

#[tokio::test]
#[ignore = "known grpc 0.9.0 deadline failure; requires BEAM_RUST_INTEROP_SERVER"]
async fn official_client_deadline() {
    let directory = tempfile::tempdir().unwrap();
    let executable = std::env::var("BEAM_RUST_INTEROP_SERVER").unwrap();
    let mut child = Command::new(executable)
        .arg("--directory")
        .arg(directory.path())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let endpoints: serde_json::Value = serde_json::from_str(&line).unwrap();
    let channel = Channel::new(
        format!("dns:///{}", endpoints["plain"].as_str().unwrap()),
        LocalChannelCredentials::new_arc(),
        ChannelOptions::default(),
    );
    let jobs = JobServiceClient::new(channel);
    let mut request = GetJobStateRequest::new();
    request.set_job_id("cancel");
    let result = tokio::time::timeout(
        Duration::from_millis(500),
        jobs.get_state(request)
            .with_timeout(Duration::from_millis(100)),
    )
    .await;
    let _ = child.kill();
    let _ = child.wait();
    let error = result
        .expect("grpc 0.9.0 ignored its 100 ms deadline for more than 500 ms")
        .unwrap_err();
    assert_eq!(error.code(), grpc::StatusCodeError::DeadlineExceeded);
}
