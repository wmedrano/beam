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

//! Private Tonic adapter for official protobuf messages. gRPC framing and
//! message-size enforcement belong to Tonic; no Prost model is generated.

use std::marker::PhantomData;

use bytes::{Buf, BufMut};
use protobuf::{Parse, Serialize};
use tonic::Status;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};

pub struct OfficialCodec<T, U>(PhantomData<fn() -> (T, U)>);

impl<T, U> OfficialCodec<T, U> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Default for OfficialCodec<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Serialize + Send + 'static, U: Parse + Send + 'static> Codec for OfficialCodec<T, U> {
    type Encode = T;
    type Decode = U;
    type Encoder = Self;
    type Decoder = Self;

    fn encoder(&mut self) -> Self {
        Self::new()
    }
    fn decoder(&mut self) -> Self {
        Self::new()
    }
}

impl<T: Serialize, U> Encoder for OfficialCodec<T, U> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: T, destination: &mut EncodeBuf<'_>) -> Result<(), Status> {
        let bytes = item
            .serialize()
            .map_err(|error| Status::internal(error.to_string()))?;
        destination.put_slice(&bytes);
        Ok(())
    }
}

impl<T, U: Parse> Decoder for OfficialCodec<T, U> {
    type Item = U;
    type Error = Status;

    fn decode(&mut self, source: &mut DecodeBuf<'_>) -> Result<Option<U>, Status> {
        let bytes = source.copy_to_bytes(source.remaining());
        U::parse(&bytes)
            .map(Some)
            .map_err(|error| Status::internal(error.to_string()))
    }
}
