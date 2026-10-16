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
pub mod artifact_retrieval_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// A service to retrieve artifacts for use in a Job.
    #[derive(Debug, Clone)]
    pub struct ArtifactRetrievalServiceClient<T> {
        channel: T,
    }

    impl<T> ArtifactRetrievalServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Resolves the given artifact references into one or more replacement
        /// artifact references (e.g. a Maven dependency into a (transitive) set
        /// of jars.
        pub fn resolve_artifacts<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::ResolveArtifactsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::ResolveArtifactsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.ArtifactRetrievalService/ResolveArtifacts", request)
        }

        /// Retrieves the given artifact as a stream of bytes.
        pub fn get_artifact<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> ServerStreamingCallBuilder<'_, &T, ReqMsgView, crate::GetArtifactResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetArtifactRequest> + Send + Sync {
          ServerStreamingCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.ArtifactRetrievalService/GetArtifact", request)
        }
    }
}
/// Generated client implementations.
pub mod artifact_staging_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// A service that allows the client to act as an ArtifactRetrievalService,
    /// for a particular job with the server initiating requests and receiving
    /// responses.
    ///
    /// A client calls the service with an ArtifactResponseWrapper that has the
    /// staging token set, and thereafter responds to the server's requests.
    #[derive(Debug, Clone)]
    pub struct ArtifactStagingServiceClient<T> {
        channel: T,
    }

    impl<T> ArtifactStagingServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn reverse_artifact_retrieval_service(&self) -> BidiCallBuilder<'_, &T, crate::ArtifactResponseWrapper, crate::ArtifactRequestWrapper> {
          BidiCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.ArtifactStagingService/ReverseArtifactRetrievalService")
        }
    }
}
/// Generated client implementations.
pub mod legacy_artifact_staging_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// A service to stage artifacts for use in a Job.
    #[derive(Debug, Clone)]
    pub struct LegacyArtifactStagingServiceClient<T> {
        channel: T,
    }

    impl<T> LegacyArtifactStagingServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Stage an artifact to be available during job execution. The first request must contain the
        /// name of the artifact. All future requests must contain sequential chunks of the content of
        /// the artifact.
        pub fn put_artifact(&self) -> ClientStreamingCallBuilder<'_, &T, crate::PutArtifactRequest, crate::PutArtifactResponse> {
          ClientStreamingCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.LegacyArtifactStagingService/PutArtifact")
        }

        /// Commit the manifest for a Job. All artifacts must have been successfully uploaded
        /// before this call is made.
        ///
        /// Throws error INVALID\_ARGUMENT if not all of the members of the manifest are present
        pub fn commit_manifest<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::CommitManifestResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::CommitManifestRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.LegacyArtifactStagingService/CommitManifest", request)
        }
    }
}
/// Generated client implementations.
pub mod legacy_artifact_retrieval_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// A service to retrieve artifacts for use in a Job.
    #[derive(Debug, Clone)]
    pub struct LegacyArtifactRetrievalServiceClient<T> {
        channel: T,
    }

    impl<T> LegacyArtifactRetrievalServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Get the manifest for the job
        pub fn get_manifest<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, crate::GetManifestResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::GetManifestRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.LegacyArtifactRetrievalService/GetManifest", request)
        }

        /// Get an artifact staged for the job. The requested artifact must be within the manifest
        pub fn get_artifact<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> ServerStreamingCallBuilder<'_, &T, ReqMsgView, crate::ArtifactChunk>
        where
          ReqMsgView: protobuf::AsView<Proxied = crate::LegacyGetArtifactRequest> + Send + Sync {
          ServerStreamingCallBuilder::new(&self.channel, "/org.apache.beam.model.job_management.v1.LegacyArtifactRetrievalService/GetArtifact", request)
        }
    }
}
