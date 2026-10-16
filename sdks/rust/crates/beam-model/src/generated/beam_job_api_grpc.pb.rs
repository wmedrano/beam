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

// @generated; see tools/generate.py.
/// Generated client implementations.
pub mod job_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// Job Service for running RunnerAPI pipelines
    #[derive(Debug, Clone)]
    pub struct JobServiceClient<T> {
        channel: T,
    }

    impl<T> JobServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Prepare a job for execution. The job will not be executed until a call is made to run with the
        /// returned preparationId.
        pub fn prepare<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::PrepareJobResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::PrepareJobRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/Prepare", request)
        }

        /// Submit the job for execution
        pub fn run<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::RunJobResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::RunJobRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/Run", request)
        }

        /// Get a list of all invoked jobs
        pub fn get_jobs<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::GetJobsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetJobsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/GetJobs", request)
        }

        /// Get the current state of the job
        pub fn get_state<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::JobStateEvent>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetJobStateRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/GetState", request)
        }

        /// Get the job's pipeline
        pub fn get_pipeline<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::GetJobPipelineResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetJobPipelineRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/GetPipeline", request)
        }

        /// Cancel the job
        pub fn cancel<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::CancelJobResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::CancelJobRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/Cancel", request)
        }

        /// Drain the job
        pub fn drain<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::DrainJobResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::DrainJobRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/Drain", request)
        }

        /// Subscribe to a stream of state changes of the job, will immediately return the current state of the job as the first response.
        pub fn get_state_stream<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> ServerStreamingCallBuilder<'_, &T, ReqMsgView, crate::JobStateEvent>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetJobStateRequest> + Send + Sync {
          ServerStreamingCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/GetStateStream", request)
        }

        /// Subscribe to a stream of state changes and messages from the job
        pub fn get_message_stream<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> ServerStreamingCallBuilder<'_, &T, ReqMsgView, crate::JobMessagesResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::JobMessagesRequest> + Send + Sync {
          ServerStreamingCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/GetMessageStream", request)
        }

        /// Fetch metrics for a given job
        pub fn get_job_metrics<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::GetJobMetricsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetJobMetricsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/GetJobMetrics", request)
        }

        /// Get the supported pipeline options of the runner
        pub fn describe_pipeline_options<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::DescribePipelineOptionsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::DescribePipelineOptionsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.JobService/DescribePipelineOptions", request)
        }
    }
}
