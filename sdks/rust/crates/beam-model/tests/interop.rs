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

use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;

use beam_model::beam_fn_data_client::BeamFnDataClient;
use beam_model::job_service_client::JobServiceClient;
use beam_model::provision_service_client::ProvisionServiceClient;
use beam_model::*;
use protobuf::prelude::*;
use tokio::time::{sleep, timeout};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};
use tonic::{Code, Request};

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn worker<T>(value: T) -> Request<T> {
    let mut request = Request::new(value);
    request
        .metadata_mut()
        .insert("worker_id", "rust-interop".parse().unwrap());
    request.set_timeout(Duration::from_secs(3));
    request
}

async fn local(target: &str) -> Result<Channel, tonic::transport::Error> {
    Endpoint::from_shared(format!("http://{target}"))?
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(3))
        .connect()
        .await
}

fn state(id: &str) -> GetJobStateRequest {
    let mut request = GetJobStateRequest::new();
    request.set_job_id(id);
    request
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires Go Beam API peer; run tools/test-interop.sh"]
async fn go_protocol_gate() {
    timeout(Duration::from_secs(30), async {
        let directory = tempfile::tempdir().unwrap();
        let executable =
            std::env::var("BEAM_RUST_INTEROP_SERVER").expect("set BEAM_RUST_INTEROP_SERVER");
        let mut server = Server(
            Command::new(executable)
                .arg("--directory")
                .arg(directory.path())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap(),
        );
        let mut line = String::new();
        BufReader::new(server.0.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        let endpoints: serde_json::Value = serde_json::from_str(&line).unwrap();
        let channel = local(endpoints["plain"].as_str().unwrap()).await.unwrap();
        let mut provision = ProvisionServiceClient::new(channel.clone());
        let info = provision
            .get_provision_info(worker(GetProvisionInfoRequest::new()))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(
            info.info()
                .pipeline_options()
                .fields()
                .get("language")
                .unwrap()
                .string_value(),
            "rust"
        );
        let error = provision
            .get_provision_info(GetProvisionInfoRequest::new())
            .await
            .unwrap_err();
        assert_eq!(error.code(), Code::PermissionDenied);
        assert_eq!(error.message(), "worker_id mismatch");

        let mut jobs = JobServiceClient::new(channel.clone());
        let error = jobs.get_state(state("missing")).await.unwrap_err();
        assert_eq!(error.code(), Code::NotFound);
        assert_eq!(error.message(), "unknown job: missing");
        let mut events = jobs
            .get_state_stream(state("test"))
            .await
            .unwrap()
            .into_inner();
        let mut states = Vec::new();
        while let Some(event) = events.message().await.unwrap() {
            states.push(event.state());
        }
        assert_eq!(
            states,
            vec![
                job_state::Enum::Starting,
                job_state::Enum::Running,
                job_state::Enum::Done
            ]
        );

        let mut request = Request::new(state("cancel"));
        request.set_timeout(Duration::from_millis(100));
        let error = jobs.get_state(request).await.unwrap_err();
        assert!(matches!(
            error.code(),
            Code::DeadlineExceeded | Code::Cancelled
        ));
        for _ in 0..100 {
            if jobs
                .get_state(state("cancelled"))
                .await
                .unwrap()
                .get_ref()
                .state()
                == job_state::Enum::Cancelled
            {
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(
            jobs.get_state(state("cancelled"))
                .await
                .unwrap()
                .get_ref()
                .state(),
            job_state::Enum::Cancelled
        );

        let mut data = BeamFnDataClient::new(channel.clone());
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let mut responses = data
            .data(worker(ReceiverStream::new(rx)))
            .await
            .unwrap()
            .into_inner();
        for i in 0..10 {
            let mut element = elements::Data::new();
            element.set_instruction_id(format!("bundle-{}", i % 2));
            element.set_transform_id("port");
            element.set_data(vec![0, 255, i]);
            let mut message = Elements::new();
            message.data_mut().push(element);
            let wire = message.serialize().unwrap();
            tx.send(message).await.unwrap();
            assert_eq!(
                responses
                    .message()
                    .await
                    .unwrap()
                    .unwrap()
                    .serialize()
                    .unwrap(),
                wire
            );
        }
        drop(tx);
        assert!(responses.message().await.unwrap().is_none());

        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let mut request = Request::new(ReceiverStream::new(rx));
        request
            .metadata_mut()
            .insert("test-backpressure", "true".parse().unwrap());
        let mut responses = data.data(request).await.unwrap().into_inner();
        let sent = Arc::new(AtomicUsize::new(0));
        let counter = sent.clone();
        let sender = tokio::spawn(async move {
            for _ in 0..64 {
                let mut part = elements::Data::new();
                part.set_data(vec![42; 1024 * 1024]);
                let mut message = Elements::new();
                message.data_mut().push(part);
                tx.send(message).await.unwrap();
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        sleep(Duration::from_millis(300)).await;
        assert!(
            sent.load(Ordering::SeqCst) < 64,
            "transport must apply backpressure while peer is blocked"
        );
        // The blocked data stream must allow unrelated calls on the same channel.
        jobs.get_state(state("unblock")).await.unwrap();
        let mut received = 0;
        while let Some(message) = responses.message().await.unwrap() {
            assert_eq!(message.data().get(0).unwrap().data().len(), 1024 * 1024);
            received += 1;
        }
        sender.await.unwrap();
        assert_eq!(received, 64);

        // Real mTLS handshake checks trust, hostname, and client identity.
        let ca = std::fs::read(directory.path().join("ca.pem")).unwrap();
        let cert = std::fs::read(directory.path().join("identity.pem")).unwrap();
        let key = std::fs::read(directory.path().join("identity.key")).unwrap();
        let config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(&ca))
            .identity(Identity::from_pem(cert, key));
        let endpoint =
            Endpoint::from_shared(format!("https://{}", endpoints["tls"].as_str().unwrap()))
                .unwrap()
                .connect_timeout(Duration::from_secs(1))
                .timeout(Duration::from_secs(2));
        let secure = endpoint
            .clone()
            .tls_config(config.clone())
            .unwrap()
            .connect()
            .await
            .unwrap();
        ProvisionServiceClient::new(secure)
            .get_provision_info(worker(GetProvisionInfoRequest::new()))
            .await
            .unwrap();
        assert!(
            endpoint
                .clone()
                .tls_config(config.domain_name("untrusted.example"))
                .unwrap()
                .connect()
                .await
                .is_err()
        );
        // Depending on TLS version, missing client identity fails during either
        // handshake or the first application request.
        if let Ok(no_identity) = endpoint
            .clone()
            .tls_config(ClientTlsConfig::new().ca_certificate(Certificate::from_pem(&ca)))
            .unwrap()
            .connect()
            .await
        {
            assert!(
                ProvisionServiceClient::new(no_identity)
                    .get_provision_info(worker(GetProvisionInfoRequest::new()))
                    .await
                    .is_err()
            );
        }
        assert!(
            endpoint
                .tls_config(ClientTlsConfig::new())
                .unwrap()
                .connect()
                .await
                .is_err()
        );
    })
    .await
    .expect("Go interoperability gate timed out");
}

#[tokio::test]
#[ignore = "requires Prism built from this checkout; run tools/test-interop.sh"]
async fn prism_job_service() {
    let executable = std::env::var("BEAM_RUST_PRISM").expect("set BEAM_RUST_PRISM");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let _server = Server(
        Command::new(executable)
            .args(["--job_port", &port.to_string(), "--serve_http=false"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let mut jobs = timeout(Duration::from_secs(15), async {
        loop {
            if let Ok(channel) = local(&format!("127.0.0.1:{port}")).await {
                break JobServiceClient::new(channel);
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap();
    assert!(
        jobs.get_jobs(GetJobsRequest::new())
            .await
            .unwrap()
            .get_ref()
            .job_info()
            .is_empty()
    );
    let error = jobs.get_state(state("missing")).await.unwrap_err();
    // Prism's current implementation returns Unknown for missing IDs.
    assert_eq!(error.code(), Code::Unknown);
    assert!(error.message().contains("missing"));
}
