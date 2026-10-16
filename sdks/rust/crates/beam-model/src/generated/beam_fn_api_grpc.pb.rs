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
pub mod beam_fn_control_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// An API that describes the work that a SDK harness is meant to do.
    /// Stable
    #[derive(Debug, Clone)]
    pub struct BeamFnControlClient<T> {
        channel: T,
    }

    impl<T> BeamFnControlClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Instructions sent by the runner to the SDK requesting different types
        /// of work.
        ///
        /// Header metadata has the specified keys pairs:
        /// - "worker\_id": the id of the sdk
        pub fn control(&self) -> BidiCallBuilder<'_, &T, crate::InstructionResponse, crate::InstructionRequest> {
          BidiCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnControl/Control")
        }

        /// Used to get the full process bundle descriptors for bundles one
        /// is asked to process.
        ///
        /// Header metadata has the specified keys pairs:
        /// - "worker\_id": the id of the sdk
        pub fn get_process_bundle_descriptor<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::ProcessBundleDescriptor>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetProcessBundleDescriptorRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnControl/GetProcessBundleDescriptor", request)
        }
    }
}
/// Generated client implementations.
pub mod beam_fn_data_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// Stable
    #[derive(Debug, Clone)]
    pub struct BeamFnDataClient<T> {
        channel: T,
    }

    impl<T> BeamFnDataClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Used to send data between harnesses. Sdks default to using an unnamed data stream
        /// (without "data\_stream\_id" header value) for bundles unless the runner requests another named stream to be
        /// used for a bundle. SDKs can advertise that they support named data streams with the capability
        /// \`beam:protocol:named\_data\_streams:v1\`.
        ///
        /// Header metadata has the specified keys pairs:
        /// - "worker\_id": value is the id of the sdk
        /// - "data\_stream\_id": value is the id of the data stream, distinguishing it from other data streams from the same
        ///   sdk. This field should only be populated if requested in a received ProcessBundleRequest from the runner.
        pub fn data(&self) -> BidiCallBuilder<'_, &T, crate::Elements, crate::Elements> {
          BidiCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnData/Data")
        }
    }
}
/// Generated client implementations.
pub mod beam_fn_state_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// An API for the SDK harness to access state stored by the runner on behalf
    /// of the SDK. State is scoped to the currently processing bundle and can be
    /// used for user state, side inputs, and runner-managed references.
    /// Stable
    #[derive(Debug, Clone)]
    pub struct BeamFnStateClient<T> {
        channel: T,
    }

    impl<T> BeamFnStateClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Used to get/append/clear state stored by the runner on behalf of the SDK.
        ///
        /// Header metadata has the specified keys pairs:
        /// - "worker\_id": the id of the sdk
        pub fn state(&self) -> BidiCallBuilder<'_, &T, crate::StateRequest, crate::StateResponse> {
          BidiCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnState/State")
        }
    }
}
/// Generated client implementations.
pub mod beam_fn_logging_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// Stable
    #[derive(Debug, Clone)]
    pub struct BeamFnLoggingClient<T> {
        channel: T,
    }

    impl<T> BeamFnLoggingClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Allows for the SDK to emit log entries which the runner can
        /// associate with the active job.
        ///
        /// Header metadata has the specified keys pairs:
        /// - "worker\_id": the id of the sdk
        pub fn logging(&self) -> BidiCallBuilder<'_, &T, crate::log_entry::List, crate::LogControl> {
          BidiCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnLogging/Logging")
        }
    }
}
/// Generated client implementations.
pub mod beam_fn_external_worker_pool_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// An API for the runner to manage external SDK worker processes.
    /// Allows the runner to start and stop SDK worker instances, typically used
    /// for containerized or process-based execution environments.
    #[derive(Debug, Clone)]
    pub struct BeamFnExternalWorkerPoolClient<T> {
        channel: T,
    }

    impl<T> BeamFnExternalWorkerPoolClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Start the SDK worker with the given ID.
        pub fn start_worker<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::StartWorkerResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::StartWorkerRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnExternalWorkerPool/StartWorker", request)
        }

        /// Stop the SDK worker.
        pub fn stop_worker<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::StopWorkerResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::StopWorkerRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnExternalWorkerPool/StopWorker", request)
        }
    }
}
/// Generated client implementations.
pub mod beam_fn_worker_status_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// API for SDKs to report debug-related statuses to runner during pipeline execution.
    #[derive(Debug, Clone)]
    pub struct BeamFnWorkerStatusClient<T> {
        channel: T,
    }

    impl<T> BeamFnWorkerStatusClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Header metadata has the specified keys pairs:
        /// - "worker\_id": the id of the sdk
        pub fn worker_status(&self) -> BidiCallBuilder<'_, &T, crate::WorkerStatusResponse, crate::WorkerStatusRequest> {
          BidiCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.BeamFnWorkerStatus/WorkerStatus")
        }
    }
}
