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
pub mod provision_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// A service to provide runtime provisioning information to the SDK harness
    /// worker instances -- such as pipeline options, resource constraints and
    /// other job metadata -- needed by an SDK harness instance to initialize.
    #[derive(Debug, Clone)]
    pub struct ProvisionServiceClient<T> {
        channel: T,
    }

    impl<T> ProvisionServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Get provision information for the SDK harness worker instance.
        pub fn get_provision_info<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::GetProvisionInfoResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetProvisionInfoRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.fn_execution.v1.ProvisionService/GetProvisionInfo", request)
        }
    }
}
