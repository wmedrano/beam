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
pub mod expansion_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// Job Service for constructing pipelines
    #[derive(Debug, Clone)]
    pub struct ExpansionServiceClient<T> {
        channel: T,
    }

    impl<T> ExpansionServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn expand<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::ExpansionResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::ExpansionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.expansion.v1.ExpansionService/Expand", request)
        }

        ///A RPC to discover already registered SchemaTransformProviders.
        /// See https://s.apache.org/easy-multi-language for more details.
        pub fn discover_schema_transform<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::DiscoverSchemaTransformResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::DiscoverSchemaTransformRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.expansion.v1.ExpansionService/DiscoverSchemaTransform", request)
        }
    }
}
