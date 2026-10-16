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
use beam_coders::{Coder, Context, Limits, Value};
use std::collections::BTreeMap;

/// A sum counter returned by the runner. Attempted and committed observations
/// are separate records; no runner commitment semantics are inferred.
pub struct CounterResult {
    pub urn: String,
    pub labels: BTreeMap<String, String>,
    pub value: i64,
    pub committed: bool,
}
use beam_model as model;
use model::job_service_client::JobServiceClient;
use std::time::Duration;
use tonic::transport::{Channel, Endpoint};

pub struct JobOptions {
    pub name: String,
    /// Bounds each unary submission, metrics, or cancellation call.
    pub rpc_timeout: Duration,
}
impl Default for JobOptions {
    fn default() -> Self {
        Self {
            name: "rust-beam".to_owned(),
            rpc_timeout: Duration::from_secs(10),
        }
    }
}

pub(crate) async fn connect(endpoint: &str) -> Result<Channel> {
    if endpoint.contains("://") && !endpoint.starts_with("http://") {
        return Err(error(
            "this local PROCESS path requires an http endpoint; remote authentication is not configured",
        ));
    }
    let uri = if endpoint.starts_with("http://") {
        endpoint.to_owned()
    } else {
        format!("http://{endpoint}")
    };
    Ok(Endpoint::from_shared(uri)?
        .connect_timeout(Duration::from_secs(10))
        .connect()
        .await?)
}

pub struct Job {
    client: JobServiceClient,
    pub id: String,
    rpc_timeout: Duration,
}

pub(crate) async fn submit(
    pipeline: model::Pipeline,
    endpoint: &str,
    options: JobOptions,
) -> Result<Job> {
    if options.name.is_empty() || options.rpc_timeout.is_zero() {
        return Err(error("job name and RPC timeout must be nonempty"));
    }
    let mut client = JobServiceClient::new(connect(endpoint).await?);
    let mut prepare = model::PrepareJobRequest::new();
    prepare.set_pipeline(pipeline);
    prepare.set_job_name(options.name);
    prepare.pipeline_options_mut();
    let prepared = tokio::time::timeout(
        options.rpc_timeout,
        client.prepare(timed_request(prepare, options.rpc_timeout)),
    )
    .await??
    .into_inner();
    let mut request = model::RunJobRequest::new();
    request.set_preparation_id(prepared.preparation_id());
    request.set_retrieval_token(
        model::urns::COMMIT_MANIFEST_RESPONSE_CONSTANTS_NO_ARTIFACTS_STAGED_TOKEN,
    );
    let response = tokio::time::timeout(
        options.rpc_timeout,
        client.run(timed_request(request, options.rpc_timeout)),
    )
    .await??
    .into_inner();
    Ok(Job {
        client,
        id: response.job_id().to_string(),
        rpc_timeout: options.rpc_timeout,
    })
}

impl Job {
    pub async fn counters(&mut self) -> Result<Vec<CounterResult>> {
        let mut request = model::GetJobMetricsRequest::new();
        request.set_job_id(&self.id);
        let response = tokio::time::timeout(
            self.rpc_timeout,
            self.client
                .get_job_metrics(timed_request(request, self.rpc_timeout)),
        )
        .await??
        .into_inner();
        let mut results = Vec::new();
        for (committed, infos) in [
            (false, response.metrics().attempted()),
            (true, response.metrics().committed()),
        ] {
            for info in infos.iter() {
                if info.r#type() != "beam:metrics:sum_int64:v1" {
                    continue;
                }
                let Value::I64(value) =
                    Coder::VarInt.decode(info.payload(), Context::Outer, Limits::default())?
                else {
                    unreachable!()
                };
                results.push(CounterResult {
                    urn: info.urn().to_string(),
                    labels: info
                        .labels()
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                    value,
                    committed,
                });
            }
        }
        Ok(results)
    }
    pub async fn cancel(&mut self) -> Result<()> {
        let mut request = model::CancelJobRequest::new();
        request.set_job_id(&self.id);
        tokio::time::timeout(
            self.rpc_timeout,
            self.client.cancel(timed_request(request, self.rpc_timeout)),
        )
        .await??;
        Ok(())
    }
    pub async fn wait(&mut self, duration: Duration) -> Result<()> {
        let mut request = model::JobMessagesRequest::new();
        request.set_job_id(&self.id);
        let wait = async {
            let mut messages = self.client.get_message_stream(request).await?.into_inner();
            while let Some(message) = messages.message().await? {
                if message.has_message_response() {
                    eprintln!("{}", message.message_response().message_text());
                }
                if message.has_state_response() {
                    let state = message.state_response().state();
                    if state == model::job_state::Enum::Done {
                        return Ok(());
                    }
                    if [
                        model::job_state::Enum::Failed,
                        model::job_state::Enum::Cancelled,
                        model::job_state::Enum::Drained,
                        model::job_state::Enum::Updated,
                    ]
                    .contains(&state)
                    {
                        return Err(error(format!(
                            "Beam job {} terminated in {state:?}",
                            self.id
                        )));
                    }
                }
            }
            Err(error("Job message stream closed before terminal state"))
        };
        match tokio::time::timeout(duration, wait).await {
            Ok(result) => result,
            Err(_) => {
                self.cancel().await?;
                Err(error("job wait timed out; cancellation requested"))
            }
        }
    }
}

fn timed_request<T>(value: T, timeout: Duration) -> tonic::Request<T> {
    let mut request = tonic::Request::new(value);
    request.set_timeout(timeout);
    request
}
