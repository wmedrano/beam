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
const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ResolveArtifactsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResolveArtifactsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResolveArtifactsRequest>
}

impl ::protobuf::Message for ResolveArtifactsRequest {}

impl ::std::default::Default for ResolveArtifactsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResolveArtifactsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResolveArtifactsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ResolveArtifactsRequestMut`.
unsafe impl Sync for ResolveArtifactsRequest {}

// SAFETY:
// - `ResolveArtifactsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ResolveArtifactsRequest {}

impl ::protobuf::Proxied for ResolveArtifactsRequest {
  type View<'msg> = ResolveArtifactsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResolveArtifactsRequest {}

impl ::protobuf::MutProxied for ResolveArtifactsRequest {
  type Mut<'msg> = ResolveArtifactsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResolveArtifactsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResolveArtifactsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResolveArtifactsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResolveArtifactsRequestView<'msg> {
  type Message = ResolveArtifactsRequest;
}

impl ::std::fmt::Debug for ResolveArtifactsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResolveArtifactsRequestView<'_> {
  fn default() -> ResolveArtifactsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResolveArtifactsRequest>> for ResolveArtifactsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResolveArtifactsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResolveArtifactsRequestView<'msg> {

  pub fn to_owned(&self) -> ResolveArtifactsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // artifacts: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn artifacts(self) -> ::protobuf::RepeatedView<'msg, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // preferred_urns: repeated string
  pub fn preferred_urns(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ResolveArtifactsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ResolveArtifactsRequestView<'_> {}

// SAFETY:
// - `ResolveArtifactsRequestView` is `Send` because while its alive a `ResolveArtifactsRequestMut` cannot.
// - `ResolveArtifactsRequestView` does not use thread-local data.
unsafe impl Send for ResolveArtifactsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ResolveArtifactsRequestView<'msg> {
  type Proxied = ResolveArtifactsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ResolveArtifactsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResolveArtifactsRequestView<'msg> {
  fn into_view<'shorter>(self) -> ResolveArtifactsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResolveArtifactsRequest> for ResolveArtifactsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResolveArtifactsRequest {
    let mut dst = ResolveArtifactsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResolveArtifactsRequest> for ResolveArtifactsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResolveArtifactsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ResolveArtifactsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResolveArtifactsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResolveArtifactsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResolveArtifactsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResolveArtifactsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResolveArtifactsRequestMut<'msg> {
  type Message = ResolveArtifactsRequest;
}

impl ::std::fmt::Debug for ResolveArtifactsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsRequest>> for ResolveArtifactsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResolveArtifactsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ResolveArtifactsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // artifacts: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn artifacts(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn artifacts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactInformation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_artifacts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactInformation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // preferred_urns: repeated string
  pub fn preferred_urns(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn preferred_urns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_preferred_urns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `ResolveArtifactsRequestMut` does not perform any shared mutation.
unsafe impl Send for ResolveArtifactsRequestMut<'_> {}

// SAFETY:
// - `ResolveArtifactsRequestMut` does not perform any shared mutation.
unsafe impl Sync for ResolveArtifactsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ResolveArtifactsRequestMut<'msg> {
  type Proxied = ResolveArtifactsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ResolveArtifactsRequest> {
    ResolveArtifactsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResolveArtifactsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResolveArtifactsRequest>
  where
      'msg: 'shorter {
    ResolveArtifactsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ResolveArtifactsRequestMut<'msg> {
  type MutProxied = ResolveArtifactsRequest;
  fn as_mut(&mut self) -> ResolveArtifactsRequestMut<'msg> {
    ResolveArtifactsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResolveArtifactsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ResolveArtifactsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResolveArtifactsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResolveArtifactsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResolveArtifactsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResolveArtifactsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // artifacts: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn artifacts(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn artifacts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactInformation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_artifacts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactInformation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // preferred_urns: repeated string
  pub fn preferred_urns(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn preferred_urns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_preferred_urns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl ResolveArtifactsRequest

impl ::std::ops::Drop for ResolveArtifactsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResolveArtifactsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResolveArtifactsRequest {
  type Proxied = Self;
  fn as_view(&self) -> ResolveArtifactsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResolveArtifactsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResolveArtifactsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResolveArtifactsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__ResolveArtifactsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__ResolveArtifactsRequest_msg_init.0, &[<super::ArtifactInformation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__ResolveArtifactsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResolveArtifactsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResolveArtifactsRequest {
  type Msg = ResolveArtifactsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResolveArtifactsRequest {
  type Msg = ResolveArtifactsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResolveArtifactsRequestMut<'_> {
  type Msg = ResolveArtifactsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResolveArtifactsRequestMut<'_> {
  type Msg = ResolveArtifactsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResolveArtifactsRequestView<'_> {
  type Msg = ResolveArtifactsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResolveArtifactsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ResolveArtifactsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResolveArtifactsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResolveArtifactsResponse>
}

impl ::protobuf::Message for ResolveArtifactsResponse {}

impl ::std::default::Default for ResolveArtifactsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResolveArtifactsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResolveArtifactsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ResolveArtifactsResponseMut`.
unsafe impl Sync for ResolveArtifactsResponse {}

// SAFETY:
// - `ResolveArtifactsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ResolveArtifactsResponse {}

impl ::protobuf::Proxied for ResolveArtifactsResponse {
  type View<'msg> = ResolveArtifactsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResolveArtifactsResponse {}

impl ::protobuf::MutProxied for ResolveArtifactsResponse {
  type Mut<'msg> = ResolveArtifactsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResolveArtifactsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResolveArtifactsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResolveArtifactsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResolveArtifactsResponseView<'msg> {
  type Message = ResolveArtifactsResponse;
}

impl ::std::fmt::Debug for ResolveArtifactsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResolveArtifactsResponseView<'_> {
  fn default() -> ResolveArtifactsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResolveArtifactsResponse>> for ResolveArtifactsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResolveArtifactsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResolveArtifactsResponseView<'msg> {

  pub fn to_owned(&self) -> ResolveArtifactsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // replacements: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn replacements(self) -> ::protobuf::RepeatedView<'msg, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ResolveArtifactsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ResolveArtifactsResponseView<'_> {}

// SAFETY:
// - `ResolveArtifactsResponseView` is `Send` because while its alive a `ResolveArtifactsResponseMut` cannot.
// - `ResolveArtifactsResponseView` does not use thread-local data.
unsafe impl Send for ResolveArtifactsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ResolveArtifactsResponseView<'msg> {
  type Proxied = ResolveArtifactsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ResolveArtifactsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResolveArtifactsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ResolveArtifactsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResolveArtifactsResponse> for ResolveArtifactsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResolveArtifactsResponse {
    let mut dst = ResolveArtifactsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResolveArtifactsResponse> for ResolveArtifactsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResolveArtifactsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ResolveArtifactsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResolveArtifactsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ResolveArtifactsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResolveArtifactsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResolveArtifactsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResolveArtifactsResponseMut<'msg> {
  type Message = ResolveArtifactsResponse;
}

impl ::std::fmt::Debug for ResolveArtifactsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsResponse>> for ResolveArtifactsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResolveArtifactsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResolveArtifactsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ResolveArtifactsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // replacements: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn replacements(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn replacements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactInformation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_replacements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactInformation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ResolveArtifactsResponseMut` does not perform any shared mutation.
unsafe impl Send for ResolveArtifactsResponseMut<'_> {}

// SAFETY:
// - `ResolveArtifactsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ResolveArtifactsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ResolveArtifactsResponseMut<'msg> {
  type Proxied = ResolveArtifactsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ResolveArtifactsResponse> {
    ResolveArtifactsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResolveArtifactsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResolveArtifactsResponse>
  where
      'msg: 'shorter {
    ResolveArtifactsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ResolveArtifactsResponseMut<'msg> {
  type MutProxied = ResolveArtifactsResponse;
  fn as_mut(&mut self) -> ResolveArtifactsResponseMut<'msg> {
    ResolveArtifactsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResolveArtifactsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ResolveArtifactsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResolveArtifactsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResolveArtifactsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResolveArtifactsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResolveArtifactsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // replacements: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn replacements(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn replacements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactInformation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_replacements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactInformation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ResolveArtifactsResponse

impl ::std::ops::Drop for ResolveArtifactsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResolveArtifactsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResolveArtifactsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ResolveArtifactsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResolveArtifactsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResolveArtifactsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResolveArtifactsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__ResolveArtifactsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__ResolveArtifactsResponse_msg_init.0, &[<super::ArtifactInformation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__ResolveArtifactsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResolveArtifactsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResolveArtifactsResponse {
  type Msg = ResolveArtifactsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResolveArtifactsResponse {
  type Msg = ResolveArtifactsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResolveArtifactsResponseMut<'_> {
  type Msg = ResolveArtifactsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResolveArtifactsResponseMut<'_> {
  type Msg = ResolveArtifactsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResolveArtifactsResponseView<'_> {
  type Msg = ResolveArtifactsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResolveArtifactsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResolveArtifactsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetArtifactRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetArtifactRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetArtifactRequest>
}

impl ::protobuf::Message for GetArtifactRequest {}

impl ::std::default::Default for GetArtifactRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetArtifactRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetArtifactRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetArtifactRequestMut`.
unsafe impl Sync for GetArtifactRequest {}

// SAFETY:
// - `GetArtifactRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetArtifactRequest {}

impl ::protobuf::Proxied for GetArtifactRequest {
  type View<'msg> = GetArtifactRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetArtifactRequest {}

impl ::protobuf::MutProxied for GetArtifactRequest {
  type Mut<'msg> = GetArtifactRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetArtifactRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetArtifactRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetArtifactRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetArtifactRequestView<'msg> {
  type Message = GetArtifactRequest;
}

impl ::std::fmt::Debug for GetArtifactRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetArtifactRequestView<'_> {
  fn default() -> GetArtifactRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetArtifactRequest>> for GetArtifactRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetArtifactRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetArtifactRequestView<'msg> {

  pub fn to_owned(&self) -> GetArtifactRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // artifact: optional message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn has_artifact(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn artifact_opt(self) -> ::protobuf::Optional<super::ArtifactInformationView<'msg>> {
        ::protobuf::Optional::new(self.artifact(), self.has_artifact())
  }
  pub fn artifact(self) -> super::ArtifactInformationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactInformationView::default())
  }

}

// SAFETY:
// - `GetArtifactRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetArtifactRequestView<'_> {}

// SAFETY:
// - `GetArtifactRequestView` is `Send` because while its alive a `GetArtifactRequestMut` cannot.
// - `GetArtifactRequestView` does not use thread-local data.
unsafe impl Send for GetArtifactRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetArtifactRequestView<'msg> {
  type Proxied = GetArtifactRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetArtifactRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetArtifactRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetArtifactRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetArtifactRequest> for GetArtifactRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetArtifactRequest {
    let mut dst = GetArtifactRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetArtifactRequest> for GetArtifactRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetArtifactRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetArtifactRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetArtifactRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetArtifactRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetArtifactRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetArtifactRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetArtifactRequestMut<'msg> {
  type Message = GetArtifactRequest;
}

impl ::std::fmt::Debug for GetArtifactRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactRequest>> for GetArtifactRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetArtifactRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetArtifactRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // artifact: optional message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn has_artifact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_artifact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn artifact_opt(&self) -> ::protobuf::Optional<super::ArtifactInformationView<'_>> {
        ::protobuf::Optional::new(self.artifact(), self.has_artifact())
  }
  pub fn artifact(&self) -> super::ArtifactInformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactInformationView::default())
  }
  pub fn artifact_mut(&mut self) -> super::ArtifactInformationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_artifact(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArtifactInformation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `GetArtifactRequestMut` does not perform any shared mutation.
unsafe impl Send for GetArtifactRequestMut<'_> {}

// SAFETY:
// - `GetArtifactRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetArtifactRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetArtifactRequestMut<'msg> {
  type Proxied = GetArtifactRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetArtifactRequest> {
    GetArtifactRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetArtifactRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetArtifactRequest>
  where
      'msg: 'shorter {
    GetArtifactRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetArtifactRequestMut<'msg> {
  type MutProxied = GetArtifactRequest;
  fn as_mut(&mut self) -> GetArtifactRequestMut<'msg> {
    GetArtifactRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetArtifactRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetArtifactRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetArtifactRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetArtifactRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetArtifactRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetArtifactRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // artifact: optional message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn has_artifact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_artifact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn artifact_opt(&self) -> ::protobuf::Optional<super::ArtifactInformationView<'_>> {
        ::protobuf::Optional::new(self.artifact(), self.has_artifact())
  }
  pub fn artifact(&self) -> super::ArtifactInformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactInformationView::default())
  }
  pub fn artifact_mut(&mut self) -> super::ArtifactInformationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_artifact(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArtifactInformation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GetArtifactRequest

impl ::std::ops::Drop for GetArtifactRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetArtifactRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetArtifactRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetArtifactRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetArtifactRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetArtifactRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetArtifactRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetArtifactRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetArtifactRequest_msg_init.0, &[<super::ArtifactInformation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetArtifactRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetArtifactRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetArtifactRequest {
  type Msg = GetArtifactRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetArtifactRequest {
  type Msg = GetArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetArtifactRequestMut<'_> {
  type Msg = GetArtifactRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetArtifactRequestMut<'_> {
  type Msg = GetArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetArtifactRequestView<'_> {
  type Msg = GetArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetArtifactRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetArtifactResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetArtifactResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetArtifactResponse>
}

impl ::protobuf::Message for GetArtifactResponse {}

impl ::std::default::Default for GetArtifactResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetArtifactResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetArtifactResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetArtifactResponseMut`.
unsafe impl Sync for GetArtifactResponse {}

// SAFETY:
// - `GetArtifactResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetArtifactResponse {}

impl ::protobuf::Proxied for GetArtifactResponse {
  type View<'msg> = GetArtifactResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetArtifactResponse {}

impl ::protobuf::MutProxied for GetArtifactResponse {
  type Mut<'msg> = GetArtifactResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetArtifactResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetArtifactResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetArtifactResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetArtifactResponseView<'msg> {
  type Message = GetArtifactResponse;
}

impl ::std::fmt::Debug for GetArtifactResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetArtifactResponseView<'_> {
  fn default() -> GetArtifactResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetArtifactResponse>> for GetArtifactResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetArtifactResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetArtifactResponseView<'msg> {

  pub fn to_owned(&self) -> GetArtifactResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // data: optional bytes
  pub fn data(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `GetArtifactResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetArtifactResponseView<'_> {}

// SAFETY:
// - `GetArtifactResponseView` is `Send` because while its alive a `GetArtifactResponseMut` cannot.
// - `GetArtifactResponseView` does not use thread-local data.
unsafe impl Send for GetArtifactResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GetArtifactResponseView<'msg> {
  type Proxied = GetArtifactResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetArtifactResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetArtifactResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetArtifactResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetArtifactResponse> for GetArtifactResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetArtifactResponse {
    let mut dst = GetArtifactResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetArtifactResponse> for GetArtifactResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetArtifactResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetArtifactResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetArtifactResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetArtifactResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetArtifactResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetArtifactResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetArtifactResponseMut<'msg> {
  type Message = GetArtifactResponse;
}

impl ::std::fmt::Debug for GetArtifactResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactResponse>> for GetArtifactResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetArtifactResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetArtifactResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GetArtifactResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetArtifactResponseMut` does not perform any shared mutation.
unsafe impl Send for GetArtifactResponseMut<'_> {}

// SAFETY:
// - `GetArtifactResponseMut` does not perform any shared mutation.
unsafe impl Sync for GetArtifactResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GetArtifactResponseMut<'msg> {
  type Proxied = GetArtifactResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetArtifactResponse> {
    GetArtifactResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetArtifactResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetArtifactResponse>
  where
      'msg: 'shorter {
    GetArtifactResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetArtifactResponseMut<'msg> {
  type MutProxied = GetArtifactResponse;
  fn as_mut(&mut self) -> GetArtifactResponseMut<'msg> {
    GetArtifactResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetArtifactResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetArtifactResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetArtifactResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetArtifactResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetArtifactResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetArtifactResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetArtifactResponse

impl ::std::ops::Drop for GetArtifactResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetArtifactResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetArtifactResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetArtifactResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetArtifactResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetArtifactResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetArtifactResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetArtifactResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetArtifactResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetArtifactResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetArtifactResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetArtifactResponse {
  type Msg = GetArtifactResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetArtifactResponse {
  type Msg = GetArtifactResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetArtifactResponseMut<'_> {
  type Msg = GetArtifactResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetArtifactResponseMut<'_> {
  type Msg = GetArtifactResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetArtifactResponseView<'_> {
  type Msg = GetArtifactResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetArtifactResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetArtifactResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ArtifactRequestWrapper_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ArtifactRequestWrapper {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ArtifactRequestWrapper>
}

impl ::protobuf::Message for ArtifactRequestWrapper {}

impl ::std::default::Default for ArtifactRequestWrapper {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ArtifactRequestWrapper {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ArtifactRequestWrapper` is `Sync` because it does not implement interior mutability.
//    Neither does `ArtifactRequestWrapperMut`.
unsafe impl Sync for ArtifactRequestWrapper {}

// SAFETY:
// - `ArtifactRequestWrapper` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ArtifactRequestWrapper {}

impl ::protobuf::Proxied for ArtifactRequestWrapper {
  type View<'msg> = ArtifactRequestWrapperView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ArtifactRequestWrapper {}

impl ::protobuf::MutProxied for ArtifactRequestWrapper {
  type Mut<'msg> = ArtifactRequestWrapperMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArtifactRequestWrapperView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactRequestWrapper>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactRequestWrapperView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ArtifactRequestWrapperView<'msg> {
  type Message = ArtifactRequestWrapper;
}

impl ::std::fmt::Debug for ArtifactRequestWrapperView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ArtifactRequestWrapperView<'_> {
  fn default() -> ArtifactRequestWrapperView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactRequestWrapper>> for ArtifactRequestWrapperView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactRequestWrapper>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactRequestWrapperView<'msg> {

  pub fn to_owned(&self) -> ArtifactRequestWrapper {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // resolve_artifact: optional message org.apache.beam.model.job_management.v1.ResolveArtifactsRequest
  pub fn has_resolve_artifact(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn resolve_artifact_opt(self) -> ::protobuf::Optional<super::ResolveArtifactsRequestView<'msg>> {
        ::protobuf::Optional::new(self.resolve_artifact(), self.has_resolve_artifact())
  }
  pub fn resolve_artifact(self) -> super::ResolveArtifactsRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResolveArtifactsRequestView::default())
  }

  // get_artifact: optional message org.apache.beam.model.job_management.v1.GetArtifactRequest
  pub fn has_get_artifact(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn get_artifact_opt(self) -> ::protobuf::Optional<super::GetArtifactRequestView<'msg>> {
        ::protobuf::Optional::new(self.get_artifact(), self.has_get_artifact())
  }
  pub fn get_artifact(self) -> super::GetArtifactRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GetArtifactRequestView::default())
  }

  pub fn request(self) -> super::artifact_request_wrapper::RequestOneof<'msg> {
    match self.request_case() {
      super::artifact_request_wrapper::RequestCase::ResolveArtifact =>
          super::artifact_request_wrapper::RequestOneof::ResolveArtifact(self.resolve_artifact()),
      super::artifact_request_wrapper::RequestCase::GetArtifact =>
          super::artifact_request_wrapper::RequestOneof::GetArtifact(self.get_artifact()),
      _ => super::artifact_request_wrapper::RequestOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn request_case(self) -> super::artifact_request_wrapper::RequestCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::artifact_request_wrapper::RequestCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ArtifactRequestWrapperView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArtifactRequestWrapperView<'_> {}

// SAFETY:
// - `ArtifactRequestWrapperView` is `Send` because while its alive a `ArtifactRequestWrapperMut` cannot.
// - `ArtifactRequestWrapperView` does not use thread-local data.
unsafe impl Send for ArtifactRequestWrapperView<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactRequestWrapperView<'msg> {
  type Proxied = ArtifactRequestWrapper;
  fn as_view(&self) -> ::protobuf::View<'msg, ArtifactRequestWrapper> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactRequestWrapperView<'msg> {
  fn into_view<'shorter>(self) -> ArtifactRequestWrapperView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactRequestWrapper> for ArtifactRequestWrapperView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactRequestWrapper {
    let mut dst = ArtifactRequestWrapper::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactRequestWrapper> for ArtifactRequestWrapperMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactRequestWrapper {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ArtifactRequestWrapper {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactRequestWrapperView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactRequestWrapperMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArtifactRequestWrapperMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactRequestWrapper>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactRequestWrapperMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ArtifactRequestWrapperMut<'msg> {
  type Message = ArtifactRequestWrapper;
}

impl ::std::fmt::Debug for ArtifactRequestWrapperMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactRequestWrapper>> for ArtifactRequestWrapperMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactRequestWrapper>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactRequestWrapperMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactRequestWrapper> {
    self.inner
  }

  pub fn to_owned(&self) -> ArtifactRequestWrapper {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // resolve_artifact: optional message org.apache.beam.model.job_management.v1.ResolveArtifactsRequest
  pub fn has_resolve_artifact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_resolve_artifact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn resolve_artifact_opt(&self) -> ::protobuf::Optional<super::ResolveArtifactsRequestView<'_>> {
        ::protobuf::Optional::new(self.resolve_artifact(), self.has_resolve_artifact())
  }
  pub fn resolve_artifact(&self) -> super::ResolveArtifactsRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResolveArtifactsRequestView::default())
  }
  pub fn resolve_artifact_mut(&mut self) -> super::ResolveArtifactsRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_resolve_artifact(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResolveArtifactsRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // get_artifact: optional message org.apache.beam.model.job_management.v1.GetArtifactRequest
  pub fn has_get_artifact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_get_artifact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn get_artifact_opt(&self) -> ::protobuf::Optional<super::GetArtifactRequestView<'_>> {
        ::protobuf::Optional::new(self.get_artifact(), self.has_get_artifact())
  }
  pub fn get_artifact(&self) -> super::GetArtifactRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GetArtifactRequestView::default())
  }
  pub fn get_artifact_mut(&mut self) -> super::GetArtifactRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_get_artifact(&mut self,
    val: impl ::protobuf::IntoProxied<super::GetArtifactRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn request(&self) -> super::artifact_request_wrapper::RequestOneof<'_> {
    match &self.request_case() {
      super::artifact_request_wrapper::RequestCase::ResolveArtifact =>
          super::artifact_request_wrapper::RequestOneof::ResolveArtifact(self.resolve_artifact()),
      super::artifact_request_wrapper::RequestCase::GetArtifact =>
          super::artifact_request_wrapper::RequestOneof::GetArtifact(self.get_artifact()),
      _ => super::artifact_request_wrapper::RequestOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn request_case(&self) -> super::artifact_request_wrapper::RequestCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::artifact_request_wrapper::RequestCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ArtifactRequestWrapperMut` does not perform any shared mutation.
unsafe impl Send for ArtifactRequestWrapperMut<'_> {}

// SAFETY:
// - `ArtifactRequestWrapperMut` does not perform any shared mutation.
unsafe impl Sync for ArtifactRequestWrapperMut<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactRequestWrapperMut<'msg> {
  type Proxied = ArtifactRequestWrapper;
  fn as_view(&self) -> ::protobuf::View<'_, ArtifactRequestWrapper> {
    ArtifactRequestWrapperView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactRequestWrapperMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ArtifactRequestWrapper>
  where
      'msg: 'shorter {
    ArtifactRequestWrapperView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ArtifactRequestWrapperMut<'msg> {
  type MutProxied = ArtifactRequestWrapper;
  fn as_mut(&mut self) -> ArtifactRequestWrapperMut<'msg> {
    ArtifactRequestWrapperMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ArtifactRequestWrapperMut<'msg> {
  fn into_mut<'shorter>(self) -> ArtifactRequestWrapperMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ArtifactRequestWrapper {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ArtifactRequestWrapper> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ArtifactRequestWrapperView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ArtifactRequestWrapperMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // resolve_artifact: optional message org.apache.beam.model.job_management.v1.ResolveArtifactsRequest
  pub fn has_resolve_artifact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_resolve_artifact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn resolve_artifact_opt(&self) -> ::protobuf::Optional<super::ResolveArtifactsRequestView<'_>> {
        ::protobuf::Optional::new(self.resolve_artifact(), self.has_resolve_artifact())
  }
  pub fn resolve_artifact(&self) -> super::ResolveArtifactsRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResolveArtifactsRequestView::default())
  }
  pub fn resolve_artifact_mut(&mut self) -> super::ResolveArtifactsRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_resolve_artifact(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResolveArtifactsRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // get_artifact: optional message org.apache.beam.model.job_management.v1.GetArtifactRequest
  pub fn has_get_artifact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_get_artifact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn get_artifact_opt(&self) -> ::protobuf::Optional<super::GetArtifactRequestView<'_>> {
        ::protobuf::Optional::new(self.get_artifact(), self.has_get_artifact())
  }
  pub fn get_artifact(&self) -> super::GetArtifactRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GetArtifactRequestView::default())
  }
  pub fn get_artifact_mut(&mut self) -> super::GetArtifactRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_get_artifact(&mut self,
    val: impl ::protobuf::IntoProxied<super::GetArtifactRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn request(&self) -> super::artifact_request_wrapper::RequestOneof<'_> {
    match &self.request_case() {
      super::artifact_request_wrapper::RequestCase::ResolveArtifact =>
          super::artifact_request_wrapper::RequestOneof::ResolveArtifact(self.resolve_artifact()),
      super::artifact_request_wrapper::RequestCase::GetArtifact =>
          super::artifact_request_wrapper::RequestOneof::GetArtifact(self.get_artifact()),
      _ => super::artifact_request_wrapper::RequestOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn request_case(&self) -> super::artifact_request_wrapper::RequestCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::artifact_request_wrapper::RequestCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ArtifactRequestWrapper

impl ::std::ops::Drop for ArtifactRequestWrapper {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ArtifactRequestWrapper {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ArtifactRequestWrapper {
  type Proxied = Self;
  fn as_view(&self) -> ArtifactRequestWrapperView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ArtifactRequestWrapper {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArtifactRequestWrapperMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArtifactRequestWrapper {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__ArtifactRequestWrapper_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$g~33^J1|K1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__ArtifactRequestWrapper_msg_init.0, &[<super::ResolveArtifactsRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::GetArtifactRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__ArtifactRequestWrapper_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactRequestWrapper {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactRequestWrapper {
  type Msg = ArtifactRequestWrapper;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactRequestWrapper> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactRequestWrapper {
  type Msg = ArtifactRequestWrapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactRequestWrapper> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactRequestWrapperMut<'_> {
  type Msg = ArtifactRequestWrapper;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactRequestWrapper> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactRequestWrapperMut<'_> {
  type Msg = ArtifactRequestWrapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactRequestWrapper> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactRequestWrapperView<'_> {
  type Msg = ArtifactRequestWrapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactRequestWrapper> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactRequestWrapperMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod artifact_request_wrapper {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RequestOneof<'msg> {
  ResolveArtifact(::protobuf::View<'msg, super::super::ResolveArtifactsRequest>) = 1000,
  GetArtifact(::protobuf::View<'msg, super::super::GetArtifactRequest>) = 1001,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RequestCase {
  ResolveArtifact = 1000,
  GetArtifact = 1001,

  not_set = 0
}

impl RequestCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RequestCase> {
    match v {
      0 => Some(RequestCase::not_set),
      1000 => Some(RequestCase::ResolveArtifact),
      1001 => Some(RequestCase::GetArtifact),
      _ => None
    }
  }
}
}  // pub mod artifact_request_wrapper


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ArtifactResponseWrapper_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ArtifactResponseWrapper {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ArtifactResponseWrapper>
}

impl ::protobuf::Message for ArtifactResponseWrapper {}

impl ::std::default::Default for ArtifactResponseWrapper {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ArtifactResponseWrapper {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ArtifactResponseWrapper` is `Sync` because it does not implement interior mutability.
//    Neither does `ArtifactResponseWrapperMut`.
unsafe impl Sync for ArtifactResponseWrapper {}

// SAFETY:
// - `ArtifactResponseWrapper` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ArtifactResponseWrapper {}

impl ::protobuf::Proxied for ArtifactResponseWrapper {
  type View<'msg> = ArtifactResponseWrapperView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ArtifactResponseWrapper {}

impl ::protobuf::MutProxied for ArtifactResponseWrapper {
  type Mut<'msg> = ArtifactResponseWrapperMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArtifactResponseWrapperView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactResponseWrapper>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactResponseWrapperView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ArtifactResponseWrapperView<'msg> {
  type Message = ArtifactResponseWrapper;
}

impl ::std::fmt::Debug for ArtifactResponseWrapperView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ArtifactResponseWrapperView<'_> {
  fn default() -> ArtifactResponseWrapperView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactResponseWrapper>> for ArtifactResponseWrapperView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactResponseWrapper>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactResponseWrapperView<'msg> {

  pub fn to_owned(&self) -> ArtifactResponseWrapper {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // staging_token: optional string
  pub fn staging_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // is_last: optional bool
  pub fn is_last(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // resolve_artifact_response: optional message org.apache.beam.model.job_management.v1.ResolveArtifactsResponse
  pub fn has_resolve_artifact_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn resolve_artifact_response_opt(self) -> ::protobuf::Optional<super::ResolveArtifactsResponseView<'msg>> {
        ::protobuf::Optional::new(self.resolve_artifact_response(), self.has_resolve_artifact_response())
  }
  pub fn resolve_artifact_response(self) -> super::ResolveArtifactsResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResolveArtifactsResponseView::default())
  }

  // get_artifact_response: optional message org.apache.beam.model.job_management.v1.GetArtifactResponse
  pub fn has_get_artifact_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn get_artifact_response_opt(self) -> ::protobuf::Optional<super::GetArtifactResponseView<'msg>> {
        ::protobuf::Optional::new(self.get_artifact_response(), self.has_get_artifact_response())
  }
  pub fn get_artifact_response(self) -> super::GetArtifactResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GetArtifactResponseView::default())
  }

  pub fn response(self) -> super::artifact_response_wrapper::ResponseOneof<'msg> {
    match self.response_case() {
      super::artifact_response_wrapper::ResponseCase::ResolveArtifactResponse =>
          super::artifact_response_wrapper::ResponseOneof::ResolveArtifactResponse(self.resolve_artifact_response()),
      super::artifact_response_wrapper::ResponseCase::GetArtifactResponse =>
          super::artifact_response_wrapper::ResponseOneof::GetArtifactResponse(self.get_artifact_response()),
      _ => super::artifact_response_wrapper::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(self) -> super::artifact_response_wrapper::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::artifact_response_wrapper::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ArtifactResponseWrapperView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArtifactResponseWrapperView<'_> {}

// SAFETY:
// - `ArtifactResponseWrapperView` is `Send` because while its alive a `ArtifactResponseWrapperMut` cannot.
// - `ArtifactResponseWrapperView` does not use thread-local data.
unsafe impl Send for ArtifactResponseWrapperView<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactResponseWrapperView<'msg> {
  type Proxied = ArtifactResponseWrapper;
  fn as_view(&self) -> ::protobuf::View<'msg, ArtifactResponseWrapper> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactResponseWrapperView<'msg> {
  fn into_view<'shorter>(self) -> ArtifactResponseWrapperView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactResponseWrapper> for ArtifactResponseWrapperView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactResponseWrapper {
    let mut dst = ArtifactResponseWrapper::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactResponseWrapper> for ArtifactResponseWrapperMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactResponseWrapper {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ArtifactResponseWrapper {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactResponseWrapperView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactResponseWrapperMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArtifactResponseWrapperMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactResponseWrapper>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactResponseWrapperMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ArtifactResponseWrapperMut<'msg> {
  type Message = ArtifactResponseWrapper;
}

impl ::std::fmt::Debug for ArtifactResponseWrapperMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactResponseWrapper>> for ArtifactResponseWrapperMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactResponseWrapper>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactResponseWrapperMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactResponseWrapper> {
    self.inner
  }

  pub fn to_owned(&self) -> ArtifactResponseWrapper {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // staging_token: optional string
  pub fn staging_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // is_last: optional bool
  pub fn is_last(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_last(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // resolve_artifact_response: optional message org.apache.beam.model.job_management.v1.ResolveArtifactsResponse
  pub fn has_resolve_artifact_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_resolve_artifact_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn resolve_artifact_response_opt(&self) -> ::protobuf::Optional<super::ResolveArtifactsResponseView<'_>> {
        ::protobuf::Optional::new(self.resolve_artifact_response(), self.has_resolve_artifact_response())
  }
  pub fn resolve_artifact_response(&self) -> super::ResolveArtifactsResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResolveArtifactsResponseView::default())
  }
  pub fn resolve_artifact_response_mut(&mut self) -> super::ResolveArtifactsResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_resolve_artifact_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResolveArtifactsResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // get_artifact_response: optional message org.apache.beam.model.job_management.v1.GetArtifactResponse
  pub fn has_get_artifact_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_get_artifact_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn get_artifact_response_opt(&self) -> ::protobuf::Optional<super::GetArtifactResponseView<'_>> {
        ::protobuf::Optional::new(self.get_artifact_response(), self.has_get_artifact_response())
  }
  pub fn get_artifact_response(&self) -> super::GetArtifactResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GetArtifactResponseView::default())
  }
  pub fn get_artifact_response_mut(&mut self) -> super::GetArtifactResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_get_artifact_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::GetArtifactResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn response(&self) -> super::artifact_response_wrapper::ResponseOneof<'_> {
    match &self.response_case() {
      super::artifact_response_wrapper::ResponseCase::ResolveArtifactResponse =>
          super::artifact_response_wrapper::ResponseOneof::ResolveArtifactResponse(self.resolve_artifact_response()),
      super::artifact_response_wrapper::ResponseCase::GetArtifactResponse =>
          super::artifact_response_wrapper::ResponseOneof::GetArtifactResponse(self.get_artifact_response()),
      _ => super::artifact_response_wrapper::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::artifact_response_wrapper::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::artifact_response_wrapper::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ArtifactResponseWrapperMut` does not perform any shared mutation.
unsafe impl Send for ArtifactResponseWrapperMut<'_> {}

// SAFETY:
// - `ArtifactResponseWrapperMut` does not perform any shared mutation.
unsafe impl Sync for ArtifactResponseWrapperMut<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactResponseWrapperMut<'msg> {
  type Proxied = ArtifactResponseWrapper;
  fn as_view(&self) -> ::protobuf::View<'_, ArtifactResponseWrapper> {
    ArtifactResponseWrapperView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactResponseWrapperMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ArtifactResponseWrapper>
  where
      'msg: 'shorter {
    ArtifactResponseWrapperView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ArtifactResponseWrapperMut<'msg> {
  type MutProxied = ArtifactResponseWrapper;
  fn as_mut(&mut self) -> ArtifactResponseWrapperMut<'msg> {
    ArtifactResponseWrapperMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ArtifactResponseWrapperMut<'msg> {
  fn into_mut<'shorter>(self) -> ArtifactResponseWrapperMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ArtifactResponseWrapper {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ArtifactResponseWrapper> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ArtifactResponseWrapperView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ArtifactResponseWrapperMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // staging_token: optional string
  pub fn staging_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // is_last: optional bool
  pub fn is_last(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_last(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // resolve_artifact_response: optional message org.apache.beam.model.job_management.v1.ResolveArtifactsResponse
  pub fn has_resolve_artifact_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_resolve_artifact_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn resolve_artifact_response_opt(&self) -> ::protobuf::Optional<super::ResolveArtifactsResponseView<'_>> {
        ::protobuf::Optional::new(self.resolve_artifact_response(), self.has_resolve_artifact_response())
  }
  pub fn resolve_artifact_response(&self) -> super::ResolveArtifactsResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResolveArtifactsResponseView::default())
  }
  pub fn resolve_artifact_response_mut(&mut self) -> super::ResolveArtifactsResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_resolve_artifact_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResolveArtifactsResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // get_artifact_response: optional message org.apache.beam.model.job_management.v1.GetArtifactResponse
  pub fn has_get_artifact_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_get_artifact_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn get_artifact_response_opt(&self) -> ::protobuf::Optional<super::GetArtifactResponseView<'_>> {
        ::protobuf::Optional::new(self.get_artifact_response(), self.has_get_artifact_response())
  }
  pub fn get_artifact_response(&self) -> super::GetArtifactResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GetArtifactResponseView::default())
  }
  pub fn get_artifact_response_mut(&mut self) -> super::GetArtifactResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_get_artifact_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::GetArtifactResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn response(&self) -> super::artifact_response_wrapper::ResponseOneof<'_> {
    match &self.response_case() {
      super::artifact_response_wrapper::ResponseCase::ResolveArtifactResponse =>
          super::artifact_response_wrapper::ResponseOneof::ResolveArtifactResponse(self.resolve_artifact_response()),
      super::artifact_response_wrapper::ResponseCase::GetArtifactResponse =>
          super::artifact_response_wrapper::ResponseOneof::GetArtifactResponse(self.get_artifact_response()),
      _ => super::artifact_response_wrapper::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::artifact_response_wrapper::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::artifact_response_wrapper::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ArtifactResponseWrapper

impl ::std::ops::Drop for ArtifactResponseWrapper {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ArtifactResponseWrapper {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ArtifactResponseWrapper {
  type Proxied = Self;
  fn as_view(&self) -> ArtifactResponseWrapperView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ArtifactResponseWrapper {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArtifactResponseWrapperMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArtifactResponseWrapper {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__ArtifactResponseWrapper_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X/Pe~33^J1|K1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__ArtifactResponseWrapper_msg_init.0, &[<super::ResolveArtifactsResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::GetArtifactResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__ArtifactResponseWrapper_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactResponseWrapper {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactResponseWrapper {
  type Msg = ArtifactResponseWrapper;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactResponseWrapper> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactResponseWrapper {
  type Msg = ArtifactResponseWrapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactResponseWrapper> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactResponseWrapperMut<'_> {
  type Msg = ArtifactResponseWrapper;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactResponseWrapper> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactResponseWrapperMut<'_> {
  type Msg = ArtifactResponseWrapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactResponseWrapper> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactResponseWrapperView<'_> {
  type Msg = ArtifactResponseWrapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactResponseWrapper> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactResponseWrapperMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod artifact_response_wrapper {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ResponseOneof<'msg> {
  ResolveArtifactResponse(::protobuf::View<'msg, super::super::ResolveArtifactsResponse>) = 1000,
  GetArtifactResponse(::protobuf::View<'msg, super::super::GetArtifactResponse>) = 1001,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ResponseCase {
  ResolveArtifactResponse = 1000,
  GetArtifactResponse = 1001,

  not_set = 0
}

impl ResponseCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ResponseCase> {
    match v {
      0 => Some(ResponseCase::not_set),
      1000 => Some(ResponseCase::ResolveArtifactResponse),
      1001 => Some(ResponseCase::GetArtifactResponse),
      _ => None
    }
  }
}
}  // pub mod artifact_response_wrapper


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ArtifactMetadata_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ArtifactMetadata {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ArtifactMetadata>
}

impl ::protobuf::Message for ArtifactMetadata {}

impl ::std::default::Default for ArtifactMetadata {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ArtifactMetadata {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ArtifactMetadata` is `Sync` because it does not implement interior mutability.
//    Neither does `ArtifactMetadataMut`.
unsafe impl Sync for ArtifactMetadata {}

// SAFETY:
// - `ArtifactMetadata` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ArtifactMetadata {}

impl ::protobuf::Proxied for ArtifactMetadata {
  type View<'msg> = ArtifactMetadataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ArtifactMetadata {}

impl ::protobuf::MutProxied for ArtifactMetadata {
  type Mut<'msg> = ArtifactMetadataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArtifactMetadataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactMetadataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ArtifactMetadataView<'msg> {
  type Message = ArtifactMetadata;
}

impl ::std::fmt::Debug for ArtifactMetadataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ArtifactMetadataView<'_> {
  fn default() -> ArtifactMetadataView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactMetadata>> for ArtifactMetadataView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactMetadataView<'msg> {

  pub fn to_owned(&self) -> ArtifactMetadata {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // permissions: optional uint32
  pub fn permissions(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // sha256: optional string
  pub fn sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ArtifactMetadataView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArtifactMetadataView<'_> {}

// SAFETY:
// - `ArtifactMetadataView` is `Send` because while its alive a `ArtifactMetadataMut` cannot.
// - `ArtifactMetadataView` does not use thread-local data.
unsafe impl Send for ArtifactMetadataView<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactMetadataView<'msg> {
  type Proxied = ArtifactMetadata;
  fn as_view(&self) -> ::protobuf::View<'msg, ArtifactMetadata> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactMetadataView<'msg> {
  fn into_view<'shorter>(self) -> ArtifactMetadataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactMetadata> for ArtifactMetadataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactMetadata {
    let mut dst = ArtifactMetadata::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactMetadata> for ArtifactMetadataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactMetadata {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ArtifactMetadata {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactMetadataView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactMetadataMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArtifactMetadataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactMetadataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ArtifactMetadataMut<'msg> {
  type Message = ArtifactMetadata;
}

impl ::std::fmt::Debug for ArtifactMetadataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactMetadata>> for ArtifactMetadataMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactMetadataMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactMetadata> {
    self.inner
  }

  pub fn to_owned(&self) -> ArtifactMetadata {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // permissions: optional uint32
  pub fn permissions(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_permissions(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // sha256: optional string
  pub fn sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ArtifactMetadataMut` does not perform any shared mutation.
unsafe impl Send for ArtifactMetadataMut<'_> {}

// SAFETY:
// - `ArtifactMetadataMut` does not perform any shared mutation.
unsafe impl Sync for ArtifactMetadataMut<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactMetadataMut<'msg> {
  type Proxied = ArtifactMetadata;
  fn as_view(&self) -> ::protobuf::View<'_, ArtifactMetadata> {
    ArtifactMetadataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactMetadataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ArtifactMetadata>
  where
      'msg: 'shorter {
    ArtifactMetadataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ArtifactMetadataMut<'msg> {
  type MutProxied = ArtifactMetadata;
  fn as_mut(&mut self) -> ArtifactMetadataMut<'msg> {
    ArtifactMetadataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ArtifactMetadataMut<'msg> {
  fn into_mut<'shorter>(self) -> ArtifactMetadataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ArtifactMetadata {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ArtifactMetadata> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ArtifactMetadataView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ArtifactMetadataMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // permissions: optional uint32
  pub fn permissions(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_permissions(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // sha256: optional string
  pub fn sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl ArtifactMetadata

impl ::std::ops::Drop for ArtifactMetadata {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ArtifactMetadata {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ArtifactMetadata {
  type Proxied = Self;
  fn as_view(&self) -> ArtifactMetadataView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ArtifactMetadata {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArtifactMetadataMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArtifactMetadata {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__ArtifactMetadata_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X)Pa1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__ArtifactMetadata_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__ArtifactMetadata_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactMetadata {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactMetadata {
  type Msg = ArtifactMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactMetadata {
  type Msg = ArtifactMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactMetadataMut<'_> {
  type Msg = ArtifactMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactMetadataMut<'_> {
  type Msg = ArtifactMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactMetadataView<'_> {
  type Msg = ArtifactMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactMetadata> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactMetadataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__Manifest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Manifest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Manifest>
}

impl ::protobuf::Message for Manifest {}

impl ::std::default::Default for Manifest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Manifest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Manifest` is `Sync` because it does not implement interior mutability.
//    Neither does `ManifestMut`.
unsafe impl Sync for Manifest {}

// SAFETY:
// - `Manifest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Manifest {}

impl ::protobuf::Proxied for Manifest {
  type View<'msg> = ManifestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Manifest {}

impl ::protobuf::MutProxied for Manifest {
  type Mut<'msg> = ManifestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ManifestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Manifest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ManifestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ManifestView<'msg> {
  type Message = Manifest;
}

impl ::std::fmt::Debug for ManifestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ManifestView<'_> {
  fn default() -> ManifestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Manifest>> for ManifestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Manifest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ManifestView<'msg> {

  pub fn to_owned(&self) -> Manifest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // artifact: repeated message org.apache.beam.model.job_management.v1.ArtifactMetadata
  pub fn artifact(self) -> ::protobuf::RepeatedView<'msg, super::ArtifactMetadata> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactMetadata>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ManifestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ManifestView<'_> {}

// SAFETY:
// - `ManifestView` is `Send` because while its alive a `ManifestMut` cannot.
// - `ManifestView` does not use thread-local data.
unsafe impl Send for ManifestView<'_> {}

impl<'msg> ::protobuf::AsView for ManifestView<'msg> {
  type Proxied = Manifest;
  fn as_view(&self) -> ::protobuf::View<'msg, Manifest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ManifestView<'msg> {
  fn into_view<'shorter>(self) -> ManifestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Manifest> for ManifestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Manifest {
    let mut dst = Manifest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Manifest> for ManifestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Manifest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Manifest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ManifestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ManifestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ManifestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Manifest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ManifestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ManifestMut<'msg> {
  type Message = Manifest;
}

impl ::std::fmt::Debug for ManifestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Manifest>> for ManifestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Manifest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ManifestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Manifest> {
    self.inner
  }

  pub fn to_owned(&self) -> Manifest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // artifact: repeated message org.apache.beam.model.job_management.v1.ArtifactMetadata
  pub fn artifact(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactMetadata> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactMetadata>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn artifact_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactMetadata> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_artifact(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactMetadata>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ManifestMut` does not perform any shared mutation.
unsafe impl Send for ManifestMut<'_> {}

// SAFETY:
// - `ManifestMut` does not perform any shared mutation.
unsafe impl Sync for ManifestMut<'_> {}

impl<'msg> ::protobuf::AsView for ManifestMut<'msg> {
  type Proxied = Manifest;
  fn as_view(&self) -> ::protobuf::View<'_, Manifest> {
    ManifestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ManifestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Manifest>
  where
      'msg: 'shorter {
    ManifestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ManifestMut<'msg> {
  type MutProxied = Manifest;
  fn as_mut(&mut self) -> ManifestMut<'msg> {
    ManifestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ManifestMut<'msg> {
  fn into_mut<'shorter>(self) -> ManifestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Manifest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Manifest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ManifestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ManifestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // artifact: repeated message org.apache.beam.model.job_management.v1.ArtifactMetadata
  pub fn artifact(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactMetadata> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactMetadata>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn artifact_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactMetadata> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_artifact(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactMetadata>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Manifest

impl ::std::ops::Drop for Manifest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Manifest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Manifest {
  type Proxied = Self;
  fn as_view(&self) -> ManifestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Manifest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ManifestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Manifest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__Manifest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__Manifest_msg_init.0, &[<super::ArtifactMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__Manifest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Manifest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Manifest {
  type Msg = Manifest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Manifest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Manifest {
  type Msg = Manifest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Manifest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ManifestMut<'_> {
  type Msg = Manifest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Manifest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ManifestMut<'_> {
  type Msg = Manifest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Manifest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ManifestView<'_> {
  type Msg = Manifest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Manifest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ManifestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ProxyManifest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProxyManifest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProxyManifest>
}

impl ::protobuf::Message for ProxyManifest {}

impl ::std::default::Default for ProxyManifest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProxyManifest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProxyManifest` is `Sync` because it does not implement interior mutability.
//    Neither does `ProxyManifestMut`.
unsafe impl Sync for ProxyManifest {}

// SAFETY:
// - `ProxyManifest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ProxyManifest {}

impl ::protobuf::Proxied for ProxyManifest {
  type View<'msg> = ProxyManifestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProxyManifest {}

impl ::protobuf::MutProxied for ProxyManifest {
  type Mut<'msg> = ProxyManifestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProxyManifestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyManifest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyManifestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProxyManifestView<'msg> {
  type Message = ProxyManifest;
}

impl ::std::fmt::Debug for ProxyManifestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProxyManifestView<'_> {
  fn default() -> ProxyManifestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyManifest>> for ProxyManifestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyManifest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyManifestView<'msg> {

  pub fn to_owned(&self) -> ProxyManifest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn manifest_opt(self) -> ::protobuf::Optional<super::ManifestView<'msg>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(self) -> super::ManifestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }

  // location: repeated message org.apache.beam.model.job_management.v1.ProxyManifest.Location
  pub fn location(self) -> ::protobuf::RepeatedView<'msg, super::proxy_manifest::Location> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::proxy_manifest::Location>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ProxyManifestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ProxyManifestView<'_> {}

// SAFETY:
// - `ProxyManifestView` is `Send` because while its alive a `ProxyManifestMut` cannot.
// - `ProxyManifestView` does not use thread-local data.
unsafe impl Send for ProxyManifestView<'_> {}

impl<'msg> ::protobuf::AsView for ProxyManifestView<'msg> {
  type Proxied = ProxyManifest;
  fn as_view(&self) -> ::protobuf::View<'msg, ProxyManifest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyManifestView<'msg> {
  fn into_view<'shorter>(self) -> ProxyManifestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyManifest> for ProxyManifestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyManifest {
    let mut dst = ProxyManifest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyManifest> for ProxyManifestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyManifest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ProxyManifest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ProxyManifestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ProxyManifestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProxyManifestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyManifest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyManifestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProxyManifestMut<'msg> {
  type Message = ProxyManifest;
}

impl ::std::fmt::Debug for ProxyManifestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyManifest>> for ProxyManifestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyManifest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyManifestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyManifest> {
    self.inner
  }

  pub fn to_owned(&self) -> ProxyManifest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_manifest(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn manifest_opt(&self) -> ::protobuf::Optional<super::ManifestView<'_>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(&self) -> super::ManifestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }
  pub fn manifest_mut(&mut self) -> super::ManifestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_manifest(&mut self,
    val: impl ::protobuf::IntoProxied<super::Manifest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // location: repeated message org.apache.beam.model.job_management.v1.ProxyManifest.Location
  pub fn location(&self) -> ::protobuf::RepeatedView<'_, super::proxy_manifest::Location> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::proxy_manifest::Location>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn location_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::proxy_manifest::Location> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_location(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::proxy_manifest::Location>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `ProxyManifestMut` does not perform any shared mutation.
unsafe impl Send for ProxyManifestMut<'_> {}

// SAFETY:
// - `ProxyManifestMut` does not perform any shared mutation.
unsafe impl Sync for ProxyManifestMut<'_> {}

impl<'msg> ::protobuf::AsView for ProxyManifestMut<'msg> {
  type Proxied = ProxyManifest;
  fn as_view(&self) -> ::protobuf::View<'_, ProxyManifest> {
    ProxyManifestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyManifestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProxyManifest>
  where
      'msg: 'shorter {
    ProxyManifestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ProxyManifestMut<'msg> {
  type MutProxied = ProxyManifest;
  fn as_mut(&mut self) -> ProxyManifestMut<'msg> {
    ProxyManifestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProxyManifestMut<'msg> {
  fn into_mut<'shorter>(self) -> ProxyManifestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProxyManifest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProxyManifest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProxyManifestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProxyManifestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_manifest(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn manifest_opt(&self) -> ::protobuf::Optional<super::ManifestView<'_>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(&self) -> super::ManifestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }
  pub fn manifest_mut(&mut self) -> super::ManifestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_manifest(&mut self,
    val: impl ::protobuf::IntoProxied<super::Manifest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // location: repeated message org.apache.beam.model.job_management.v1.ProxyManifest.Location
  pub fn location(&self) -> ::protobuf::RepeatedView<'_, super::proxy_manifest::Location> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::proxy_manifest::Location>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn location_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::proxy_manifest::Location> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_location(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::proxy_manifest::Location>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl ProxyManifest

impl ::std::ops::Drop for ProxyManifest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProxyManifest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProxyManifest {
  type Proxied = Self;
  fn as_view(&self) -> ProxyManifestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProxyManifest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProxyManifestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProxyManifest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__ProxyManifest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__ProxyManifest_msg_init.0, &[<super::Manifest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::proxy_manifest::Location as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__ProxyManifest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyManifest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyManifest {
  type Msg = ProxyManifest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyManifest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyManifest {
  type Msg = ProxyManifest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyManifest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyManifestMut<'_> {
  type Msg = ProxyManifest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyManifest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyManifestMut<'_> {
  type Msg = ProxyManifest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyManifest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyManifestView<'_> {
  type Msg = ProxyManifest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyManifest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyManifestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod proxy_manifest {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ProxyManifest__Location_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Location {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Location>
}

impl ::protobuf::Message for Location {}

impl ::std::default::Default for Location {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Location {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Location` is `Sync` because it does not implement interior mutability.
//    Neither does `LocationMut`.
unsafe impl Sync for Location {}

// SAFETY:
// - `Location` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Location {}

impl ::protobuf::Proxied for Location {
  type View<'msg> = LocationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Location {}

impl ::protobuf::MutProxied for Location {
  type Mut<'msg> = LocationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Location>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocationView<'msg> {
  type Message = Location;
}

impl ::std::fmt::Debug for LocationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocationView<'_> {
  fn default() -> LocationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Location>> for LocationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Location>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocationView<'msg> {

  pub fn to_owned(&self) -> Location {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // uri: optional string
  pub fn uri(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `LocationView` is `Sync` because it does not support mutation.
unsafe impl Sync for LocationView<'_> {}

// SAFETY:
// - `LocationView` is `Send` because while its alive a `LocationMut` cannot.
// - `LocationView` does not use thread-local data.
unsafe impl Send for LocationView<'_> {}

impl<'msg> ::protobuf::AsView for LocationView<'msg> {
  type Proxied = Location;
  fn as_view(&self) -> ::protobuf::View<'msg, Location> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocationView<'msg> {
  fn into_view<'shorter>(self) -> LocationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Location> for LocationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Location {
    let mut dst = Location::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Location> for LocationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Location {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Location {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LocationView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LocationMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Location>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocationMut<'msg> {
  type Message = Location;
}

impl ::std::fmt::Debug for LocationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Location>> for LocationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Location>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Location> {
    self.inner
  }

  pub fn to_owned(&self) -> Location {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // uri: optional string
  pub fn uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `LocationMut` does not perform any shared mutation.
unsafe impl Send for LocationMut<'_> {}

// SAFETY:
// - `LocationMut` does not perform any shared mutation.
unsafe impl Sync for LocationMut<'_> {}

impl<'msg> ::protobuf::AsView for LocationMut<'msg> {
  type Proxied = Location;
  fn as_view(&self) -> ::protobuf::View<'_, Location> {
    LocationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Location>
  where
      'msg: 'shorter {
    LocationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for LocationMut<'msg> {
  type MutProxied = Location;
  fn as_mut(&mut self) -> LocationMut<'msg> {
    LocationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocationMut<'msg> {
  fn into_mut<'shorter>(self) -> LocationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Location {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Location> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // uri: optional string
  pub fn uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Location

impl ::std::ops::Drop for Location {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Location {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Location {
  type Proxied = Self;
  fn as_view(&self) -> LocationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Location {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Location {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::proxy_manifest::org__apache__beam__model__job_0management__v1__ProxyManifest__Location_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::proxy_manifest::org__apache__beam__model__job_0management__v1__ProxyManifest__Location_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::proxy_manifest::org__apache__beam__model__job_0management__v1__ProxyManifest__Location_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Location {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Location {
  type Msg = Location;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Location {
  type Msg = Location;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocationMut<'_> {
  type Msg = Location;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocationMut<'_> {
  type Msg = Location;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocationView<'_> {
  type Msg = Location;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Location> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod proxy_manifest


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetManifestRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetManifestRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetManifestRequest>
}

impl ::protobuf::Message for GetManifestRequest {}

impl ::std::default::Default for GetManifestRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetManifestRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetManifestRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetManifestRequestMut`.
unsafe impl Sync for GetManifestRequest {}

// SAFETY:
// - `GetManifestRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetManifestRequest {}

impl ::protobuf::Proxied for GetManifestRequest {
  type View<'msg> = GetManifestRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetManifestRequest {}

impl ::protobuf::MutProxied for GetManifestRequest {
  type Mut<'msg> = GetManifestRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetManifestRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetManifestRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetManifestRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetManifestRequestView<'msg> {
  type Message = GetManifestRequest;
}

impl ::std::fmt::Debug for GetManifestRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetManifestRequestView<'_> {
  fn default() -> GetManifestRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetManifestRequest>> for GetManifestRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetManifestRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetManifestRequestView<'msg> {

  pub fn to_owned(&self) -> GetManifestRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // retrieval_token: optional string
  pub fn retrieval_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetManifestRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetManifestRequestView<'_> {}

// SAFETY:
// - `GetManifestRequestView` is `Send` because while its alive a `GetManifestRequestMut` cannot.
// - `GetManifestRequestView` does not use thread-local data.
unsafe impl Send for GetManifestRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetManifestRequestView<'msg> {
  type Proxied = GetManifestRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetManifestRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetManifestRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetManifestRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetManifestRequest> for GetManifestRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetManifestRequest {
    let mut dst = GetManifestRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetManifestRequest> for GetManifestRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetManifestRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetManifestRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetManifestRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetManifestRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetManifestRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetManifestRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetManifestRequestMut<'msg> {
  type Message = GetManifestRequest;
}

impl ::std::fmt::Debug for GetManifestRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestRequest>> for GetManifestRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetManifestRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetManifestRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // retrieval_token: optional string
  pub fn retrieval_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_retrieval_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetManifestRequestMut` does not perform any shared mutation.
unsafe impl Send for GetManifestRequestMut<'_> {}

// SAFETY:
// - `GetManifestRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetManifestRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetManifestRequestMut<'msg> {
  type Proxied = GetManifestRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetManifestRequest> {
    GetManifestRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetManifestRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetManifestRequest>
  where
      'msg: 'shorter {
    GetManifestRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetManifestRequestMut<'msg> {
  type MutProxied = GetManifestRequest;
  fn as_mut(&mut self) -> GetManifestRequestMut<'msg> {
    GetManifestRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetManifestRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetManifestRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetManifestRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetManifestRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetManifestRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetManifestRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // retrieval_token: optional string
  pub fn retrieval_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_retrieval_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetManifestRequest

impl ::std::ops::Drop for GetManifestRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetManifestRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetManifestRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetManifestRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetManifestRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetManifestRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetManifestRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetManifestRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetManifestRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetManifestRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetManifestRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetManifestRequest {
  type Msg = GetManifestRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetManifestRequest {
  type Msg = GetManifestRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetManifestRequestMut<'_> {
  type Msg = GetManifestRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetManifestRequestMut<'_> {
  type Msg = GetManifestRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetManifestRequestView<'_> {
  type Msg = GetManifestRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetManifestRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetManifestResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetManifestResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetManifestResponse>
}

impl ::protobuf::Message for GetManifestResponse {}

impl ::std::default::Default for GetManifestResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetManifestResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetManifestResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetManifestResponseMut`.
unsafe impl Sync for GetManifestResponse {}

// SAFETY:
// - `GetManifestResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetManifestResponse {}

impl ::protobuf::Proxied for GetManifestResponse {
  type View<'msg> = GetManifestResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetManifestResponse {}

impl ::protobuf::MutProxied for GetManifestResponse {
  type Mut<'msg> = GetManifestResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetManifestResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetManifestResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetManifestResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetManifestResponseView<'msg> {
  type Message = GetManifestResponse;
}

impl ::std::fmt::Debug for GetManifestResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetManifestResponseView<'_> {
  fn default() -> GetManifestResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetManifestResponse>> for GetManifestResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetManifestResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetManifestResponseView<'msg> {

  pub fn to_owned(&self) -> GetManifestResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn manifest_opt(self) -> ::protobuf::Optional<super::ManifestView<'msg>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(self) -> super::ManifestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }

}

// SAFETY:
// - `GetManifestResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetManifestResponseView<'_> {}

// SAFETY:
// - `GetManifestResponseView` is `Send` because while its alive a `GetManifestResponseMut` cannot.
// - `GetManifestResponseView` does not use thread-local data.
unsafe impl Send for GetManifestResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GetManifestResponseView<'msg> {
  type Proxied = GetManifestResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetManifestResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetManifestResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetManifestResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetManifestResponse> for GetManifestResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetManifestResponse {
    let mut dst = GetManifestResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetManifestResponse> for GetManifestResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetManifestResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetManifestResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetManifestResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetManifestResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetManifestResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetManifestResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetManifestResponseMut<'msg> {
  type Message = GetManifestResponse;
}

impl ::std::fmt::Debug for GetManifestResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestResponse>> for GetManifestResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetManifestResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetManifestResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GetManifestResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_manifest(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn manifest_opt(&self) -> ::protobuf::Optional<super::ManifestView<'_>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(&self) -> super::ManifestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }
  pub fn manifest_mut(&mut self) -> super::ManifestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_manifest(&mut self,
    val: impl ::protobuf::IntoProxied<super::Manifest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `GetManifestResponseMut` does not perform any shared mutation.
unsafe impl Send for GetManifestResponseMut<'_> {}

// SAFETY:
// - `GetManifestResponseMut` does not perform any shared mutation.
unsafe impl Sync for GetManifestResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GetManifestResponseMut<'msg> {
  type Proxied = GetManifestResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetManifestResponse> {
    GetManifestResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetManifestResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetManifestResponse>
  where
      'msg: 'shorter {
    GetManifestResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetManifestResponseMut<'msg> {
  type MutProxied = GetManifestResponse;
  fn as_mut(&mut self) -> GetManifestResponseMut<'msg> {
    GetManifestResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetManifestResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetManifestResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetManifestResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetManifestResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetManifestResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetManifestResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_manifest(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn manifest_opt(&self) -> ::protobuf::Optional<super::ManifestView<'_>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(&self) -> super::ManifestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }
  pub fn manifest_mut(&mut self) -> super::ManifestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_manifest(&mut self,
    val: impl ::protobuf::IntoProxied<super::Manifest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GetManifestResponse

impl ::std::ops::Drop for GetManifestResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetManifestResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetManifestResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetManifestResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetManifestResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetManifestResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetManifestResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetManifestResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetManifestResponse_msg_init.0, &[<super::Manifest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetManifestResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetManifestResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetManifestResponse {
  type Msg = GetManifestResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetManifestResponse {
  type Msg = GetManifestResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetManifestResponseMut<'_> {
  type Msg = GetManifestResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetManifestResponseMut<'_> {
  type Msg = GetManifestResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetManifestResponseView<'_> {
  type Msg = GetManifestResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetManifestResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetManifestResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__LegacyGetArtifactRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LegacyGetArtifactRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LegacyGetArtifactRequest>
}

impl ::protobuf::Message for LegacyGetArtifactRequest {}

impl ::std::default::Default for LegacyGetArtifactRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LegacyGetArtifactRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LegacyGetArtifactRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `LegacyGetArtifactRequestMut`.
unsafe impl Sync for LegacyGetArtifactRequest {}

// SAFETY:
// - `LegacyGetArtifactRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for LegacyGetArtifactRequest {}

impl ::protobuf::Proxied for LegacyGetArtifactRequest {
  type View<'msg> = LegacyGetArtifactRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LegacyGetArtifactRequest {}

impl ::protobuf::MutProxied for LegacyGetArtifactRequest {
  type Mut<'msg> = LegacyGetArtifactRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LegacyGetArtifactRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LegacyGetArtifactRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LegacyGetArtifactRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LegacyGetArtifactRequestView<'msg> {
  type Message = LegacyGetArtifactRequest;
}

impl ::std::fmt::Debug for LegacyGetArtifactRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LegacyGetArtifactRequestView<'_> {
  fn default() -> LegacyGetArtifactRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LegacyGetArtifactRequest>> for LegacyGetArtifactRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LegacyGetArtifactRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LegacyGetArtifactRequestView<'msg> {

  pub fn to_owned(&self) -> LegacyGetArtifactRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // retrieval_token: optional string
  pub fn retrieval_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `LegacyGetArtifactRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for LegacyGetArtifactRequestView<'_> {}

// SAFETY:
// - `LegacyGetArtifactRequestView` is `Send` because while its alive a `LegacyGetArtifactRequestMut` cannot.
// - `LegacyGetArtifactRequestView` does not use thread-local data.
unsafe impl Send for LegacyGetArtifactRequestView<'_> {}

impl<'msg> ::protobuf::AsView for LegacyGetArtifactRequestView<'msg> {
  type Proxied = LegacyGetArtifactRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, LegacyGetArtifactRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LegacyGetArtifactRequestView<'msg> {
  fn into_view<'shorter>(self) -> LegacyGetArtifactRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LegacyGetArtifactRequest> for LegacyGetArtifactRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LegacyGetArtifactRequest {
    let mut dst = LegacyGetArtifactRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LegacyGetArtifactRequest> for LegacyGetArtifactRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LegacyGetArtifactRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for LegacyGetArtifactRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LegacyGetArtifactRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LegacyGetArtifactRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LegacyGetArtifactRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LegacyGetArtifactRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LegacyGetArtifactRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LegacyGetArtifactRequestMut<'msg> {
  type Message = LegacyGetArtifactRequest;
}

impl ::std::fmt::Debug for LegacyGetArtifactRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LegacyGetArtifactRequest>> for LegacyGetArtifactRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LegacyGetArtifactRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LegacyGetArtifactRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LegacyGetArtifactRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> LegacyGetArtifactRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // retrieval_token: optional string
  pub fn retrieval_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_retrieval_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `LegacyGetArtifactRequestMut` does not perform any shared mutation.
unsafe impl Send for LegacyGetArtifactRequestMut<'_> {}

// SAFETY:
// - `LegacyGetArtifactRequestMut` does not perform any shared mutation.
unsafe impl Sync for LegacyGetArtifactRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for LegacyGetArtifactRequestMut<'msg> {
  type Proxied = LegacyGetArtifactRequest;
  fn as_view(&self) -> ::protobuf::View<'_, LegacyGetArtifactRequest> {
    LegacyGetArtifactRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LegacyGetArtifactRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LegacyGetArtifactRequest>
  where
      'msg: 'shorter {
    LegacyGetArtifactRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for LegacyGetArtifactRequestMut<'msg> {
  type MutProxied = LegacyGetArtifactRequest;
  fn as_mut(&mut self) -> LegacyGetArtifactRequestMut<'msg> {
    LegacyGetArtifactRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LegacyGetArtifactRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> LegacyGetArtifactRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LegacyGetArtifactRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LegacyGetArtifactRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LegacyGetArtifactRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LegacyGetArtifactRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // retrieval_token: optional string
  pub fn retrieval_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_retrieval_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl LegacyGetArtifactRequest

impl ::std::ops::Drop for LegacyGetArtifactRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LegacyGetArtifactRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LegacyGetArtifactRequest {
  type Proxied = Self;
  fn as_view(&self) -> LegacyGetArtifactRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LegacyGetArtifactRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LegacyGetArtifactRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LegacyGetArtifactRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__LegacyGetArtifactRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__LegacyGetArtifactRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__LegacyGetArtifactRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LegacyGetArtifactRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LegacyGetArtifactRequest {
  type Msg = LegacyGetArtifactRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LegacyGetArtifactRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LegacyGetArtifactRequest {
  type Msg = LegacyGetArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LegacyGetArtifactRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LegacyGetArtifactRequestMut<'_> {
  type Msg = LegacyGetArtifactRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LegacyGetArtifactRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LegacyGetArtifactRequestMut<'_> {
  type Msg = LegacyGetArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LegacyGetArtifactRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LegacyGetArtifactRequestView<'_> {
  type Msg = LegacyGetArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LegacyGetArtifactRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LegacyGetArtifactRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__ArtifactChunk_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ArtifactChunk {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ArtifactChunk>
}

impl ::protobuf::Message for ArtifactChunk {}

impl ::std::default::Default for ArtifactChunk {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ArtifactChunk {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ArtifactChunk` is `Sync` because it does not implement interior mutability.
//    Neither does `ArtifactChunkMut`.
unsafe impl Sync for ArtifactChunk {}

// SAFETY:
// - `ArtifactChunk` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ArtifactChunk {}

impl ::protobuf::Proxied for ArtifactChunk {
  type View<'msg> = ArtifactChunkView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ArtifactChunk {}

impl ::protobuf::MutProxied for ArtifactChunk {
  type Mut<'msg> = ArtifactChunkMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArtifactChunkView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactChunk>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactChunkView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ArtifactChunkView<'msg> {
  type Message = ArtifactChunk;
}

impl ::std::fmt::Debug for ArtifactChunkView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ArtifactChunkView<'_> {
  fn default() -> ArtifactChunkView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactChunk>> for ArtifactChunkView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArtifactChunk>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactChunkView<'msg> {

  pub fn to_owned(&self) -> ArtifactChunk {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // data: optional bytes
  pub fn data(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `ArtifactChunkView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArtifactChunkView<'_> {}

// SAFETY:
// - `ArtifactChunkView` is `Send` because while its alive a `ArtifactChunkMut` cannot.
// - `ArtifactChunkView` does not use thread-local data.
unsafe impl Send for ArtifactChunkView<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactChunkView<'msg> {
  type Proxied = ArtifactChunk;
  fn as_view(&self) -> ::protobuf::View<'msg, ArtifactChunk> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactChunkView<'msg> {
  fn into_view<'shorter>(self) -> ArtifactChunkView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactChunk> for ArtifactChunkView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactChunk {
    let mut dst = ArtifactChunk::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ArtifactChunk> for ArtifactChunkMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArtifactChunk {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ArtifactChunk {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactChunkView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArtifactChunkMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArtifactChunkMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactChunk>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArtifactChunkMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ArtifactChunkMut<'msg> {
  type Message = ArtifactChunk;
}

impl ::std::fmt::Debug for ArtifactChunkMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactChunk>> for ArtifactChunkMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactChunk>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArtifactChunkMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ArtifactChunk> {
    self.inner
  }

  pub fn to_owned(&self) -> ArtifactChunk {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `ArtifactChunkMut` does not perform any shared mutation.
unsafe impl Send for ArtifactChunkMut<'_> {}

// SAFETY:
// - `ArtifactChunkMut` does not perform any shared mutation.
unsafe impl Sync for ArtifactChunkMut<'_> {}

impl<'msg> ::protobuf::AsView for ArtifactChunkMut<'msg> {
  type Proxied = ArtifactChunk;
  fn as_view(&self) -> ::protobuf::View<'_, ArtifactChunk> {
    ArtifactChunkView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArtifactChunkMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ArtifactChunk>
  where
      'msg: 'shorter {
    ArtifactChunkView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ArtifactChunkMut<'msg> {
  type MutProxied = ArtifactChunk;
  fn as_mut(&mut self) -> ArtifactChunkMut<'msg> {
    ArtifactChunkMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ArtifactChunkMut<'msg> {
  fn into_mut<'shorter>(self) -> ArtifactChunkMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ArtifactChunk {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ArtifactChunk> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ArtifactChunkView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ArtifactChunkMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl ArtifactChunk

impl ::std::ops::Drop for ArtifactChunk {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ArtifactChunk {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ArtifactChunk {
  type Proxied = Self;
  fn as_view(&self) -> ArtifactChunkView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ArtifactChunk {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArtifactChunkMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArtifactChunk {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__ArtifactChunk_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__ArtifactChunk_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__ArtifactChunk_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactChunk {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactChunk {
  type Msg = ArtifactChunk;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactChunk> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactChunk {
  type Msg = ArtifactChunk;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactChunk> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArtifactChunkMut<'_> {
  type Msg = ArtifactChunk;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactChunk> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactChunkMut<'_> {
  type Msg = ArtifactChunk;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactChunk> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArtifactChunkView<'_> {
  type Msg = ArtifactChunk;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArtifactChunk> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArtifactChunkMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__PutArtifactMetadata_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PutArtifactMetadata {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PutArtifactMetadata>
}

impl ::protobuf::Message for PutArtifactMetadata {}

impl ::std::default::Default for PutArtifactMetadata {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PutArtifactMetadata {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PutArtifactMetadata` is `Sync` because it does not implement interior mutability.
//    Neither does `PutArtifactMetadataMut`.
unsafe impl Sync for PutArtifactMetadata {}

// SAFETY:
// - `PutArtifactMetadata` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PutArtifactMetadata {}

impl ::protobuf::Proxied for PutArtifactMetadata {
  type View<'msg> = PutArtifactMetadataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PutArtifactMetadata {}

impl ::protobuf::MutProxied for PutArtifactMetadata {
  type Mut<'msg> = PutArtifactMetadataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PutArtifactMetadataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PutArtifactMetadataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PutArtifactMetadataView<'msg> {
  type Message = PutArtifactMetadata;
}

impl ::std::fmt::Debug for PutArtifactMetadataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PutArtifactMetadataView<'_> {
  fn default() -> PutArtifactMetadataView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactMetadata>> for PutArtifactMetadataView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PutArtifactMetadataView<'msg> {

  pub fn to_owned(&self) -> PutArtifactMetadata {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // staging_session_token: optional string
  pub fn staging_session_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // metadata: optional message org.apache.beam.model.job_management.v1.ArtifactMetadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn metadata_opt(self) -> ::protobuf::Optional<super::ArtifactMetadataView<'msg>> {
        ::protobuf::Optional::new(self.metadata(), self.has_metadata())
  }
  pub fn metadata(self) -> super::ArtifactMetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactMetadataView::default())
  }

}

// SAFETY:
// - `PutArtifactMetadataView` is `Sync` because it does not support mutation.
unsafe impl Sync for PutArtifactMetadataView<'_> {}

// SAFETY:
// - `PutArtifactMetadataView` is `Send` because while its alive a `PutArtifactMetadataMut` cannot.
// - `PutArtifactMetadataView` does not use thread-local data.
unsafe impl Send for PutArtifactMetadataView<'_> {}

impl<'msg> ::protobuf::AsView for PutArtifactMetadataView<'msg> {
  type Proxied = PutArtifactMetadata;
  fn as_view(&self) -> ::protobuf::View<'msg, PutArtifactMetadata> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PutArtifactMetadataView<'msg> {
  fn into_view<'shorter>(self) -> PutArtifactMetadataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PutArtifactMetadata> for PutArtifactMetadataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PutArtifactMetadata {
    let mut dst = PutArtifactMetadata::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PutArtifactMetadata> for PutArtifactMetadataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PutArtifactMetadata {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PutArtifactMetadata {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PutArtifactMetadataView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PutArtifactMetadataMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PutArtifactMetadataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PutArtifactMetadataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PutArtifactMetadataMut<'msg> {
  type Message = PutArtifactMetadata;
}

impl ::std::fmt::Debug for PutArtifactMetadataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactMetadata>> for PutArtifactMetadataMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PutArtifactMetadataMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactMetadata> {
    self.inner
  }

  pub fn to_owned(&self) -> PutArtifactMetadata {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // staging_session_token: optional string
  pub fn staging_session_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_session_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // metadata: optional message org.apache.beam.model.job_management.v1.ArtifactMetadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn metadata_opt(&self) -> ::protobuf::Optional<super::ArtifactMetadataView<'_>> {
        ::protobuf::Optional::new(self.metadata(), self.has_metadata())
  }
  pub fn metadata(&self) -> super::ArtifactMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactMetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> super::ArtifactMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArtifactMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `PutArtifactMetadataMut` does not perform any shared mutation.
unsafe impl Send for PutArtifactMetadataMut<'_> {}

// SAFETY:
// - `PutArtifactMetadataMut` does not perform any shared mutation.
unsafe impl Sync for PutArtifactMetadataMut<'_> {}

impl<'msg> ::protobuf::AsView for PutArtifactMetadataMut<'msg> {
  type Proxied = PutArtifactMetadata;
  fn as_view(&self) -> ::protobuf::View<'_, PutArtifactMetadata> {
    PutArtifactMetadataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PutArtifactMetadataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PutArtifactMetadata>
  where
      'msg: 'shorter {
    PutArtifactMetadataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PutArtifactMetadataMut<'msg> {
  type MutProxied = PutArtifactMetadata;
  fn as_mut(&mut self) -> PutArtifactMetadataMut<'msg> {
    PutArtifactMetadataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PutArtifactMetadataMut<'msg> {
  fn into_mut<'shorter>(self) -> PutArtifactMetadataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PutArtifactMetadata {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PutArtifactMetadata> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PutArtifactMetadataView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PutArtifactMetadataMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // staging_session_token: optional string
  pub fn staging_session_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_session_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // metadata: optional message org.apache.beam.model.job_management.v1.ArtifactMetadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn metadata_opt(&self) -> ::protobuf::Optional<super::ArtifactMetadataView<'_>> {
        ::protobuf::Optional::new(self.metadata(), self.has_metadata())
  }
  pub fn metadata(&self) -> super::ArtifactMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactMetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> super::ArtifactMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArtifactMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl PutArtifactMetadata

impl ::std::ops::Drop for PutArtifactMetadata {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PutArtifactMetadata {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PutArtifactMetadata {
  type Proxied = Self;
  fn as_view(&self) -> PutArtifactMetadataView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PutArtifactMetadata {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PutArtifactMetadataMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PutArtifactMetadata {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__PutArtifactMetadata_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__PutArtifactMetadata_msg_init.0, &[<super::ArtifactMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__PutArtifactMetadata_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PutArtifactMetadata {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PutArtifactMetadata {
  type Msg = PutArtifactMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactMetadata {
  type Msg = PutArtifactMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PutArtifactMetadataMut<'_> {
  type Msg = PutArtifactMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactMetadataMut<'_> {
  type Msg = PutArtifactMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactMetadataView<'_> {
  type Msg = PutArtifactMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactMetadata> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PutArtifactMetadataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__PutArtifactRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PutArtifactRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PutArtifactRequest>
}

impl ::protobuf::Message for PutArtifactRequest {}

impl ::std::default::Default for PutArtifactRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PutArtifactRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PutArtifactRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PutArtifactRequestMut`.
unsafe impl Sync for PutArtifactRequest {}

// SAFETY:
// - `PutArtifactRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PutArtifactRequest {}

impl ::protobuf::Proxied for PutArtifactRequest {
  type View<'msg> = PutArtifactRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PutArtifactRequest {}

impl ::protobuf::MutProxied for PutArtifactRequest {
  type Mut<'msg> = PutArtifactRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PutArtifactRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PutArtifactRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PutArtifactRequestView<'msg> {
  type Message = PutArtifactRequest;
}

impl ::std::fmt::Debug for PutArtifactRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PutArtifactRequestView<'_> {
  fn default() -> PutArtifactRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactRequest>> for PutArtifactRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PutArtifactRequestView<'msg> {

  pub fn to_owned(&self) -> PutArtifactRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // metadata: optional message org.apache.beam.model.job_management.v1.PutArtifactMetadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn metadata_opt(self) -> ::protobuf::Optional<super::PutArtifactMetadataView<'msg>> {
        ::protobuf::Optional::new(self.metadata(), self.has_metadata())
  }
  pub fn metadata(self) -> super::PutArtifactMetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PutArtifactMetadataView::default())
  }

  // data: optional message org.apache.beam.model.job_management.v1.ArtifactChunk
  pub fn has_data(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn data_opt(self) -> ::protobuf::Optional<super::ArtifactChunkView<'msg>> {
        ::protobuf::Optional::new(self.data(), self.has_data())
  }
  pub fn data(self) -> super::ArtifactChunkView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactChunkView::default())
  }

  pub fn content(self) -> super::put_artifact_request::ContentOneof<'msg> {
    match self.content_case() {
      super::put_artifact_request::ContentCase::Metadata =>
          super::put_artifact_request::ContentOneof::Metadata(self.metadata()),
      super::put_artifact_request::ContentCase::Data =>
          super::put_artifact_request::ContentOneof::Data(self.data()),
      _ => super::put_artifact_request::ContentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn content_case(self) -> super::put_artifact_request::ContentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::put_artifact_request::ContentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PutArtifactRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PutArtifactRequestView<'_> {}

// SAFETY:
// - `PutArtifactRequestView` is `Send` because while its alive a `PutArtifactRequestMut` cannot.
// - `PutArtifactRequestView` does not use thread-local data.
unsafe impl Send for PutArtifactRequestView<'_> {}

impl<'msg> ::protobuf::AsView for PutArtifactRequestView<'msg> {
  type Proxied = PutArtifactRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PutArtifactRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PutArtifactRequestView<'msg> {
  fn into_view<'shorter>(self) -> PutArtifactRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PutArtifactRequest> for PutArtifactRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PutArtifactRequest {
    let mut dst = PutArtifactRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PutArtifactRequest> for PutArtifactRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PutArtifactRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PutArtifactRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PutArtifactRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PutArtifactRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PutArtifactRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PutArtifactRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PutArtifactRequestMut<'msg> {
  type Message = PutArtifactRequest;
}

impl ::std::fmt::Debug for PutArtifactRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactRequest>> for PutArtifactRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PutArtifactRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PutArtifactRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // metadata: optional message org.apache.beam.model.job_management.v1.PutArtifactMetadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn metadata_opt(&self) -> ::protobuf::Optional<super::PutArtifactMetadataView<'_>> {
        ::protobuf::Optional::new(self.metadata(), self.has_metadata())
  }
  pub fn metadata(&self) -> super::PutArtifactMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PutArtifactMetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> super::PutArtifactMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::PutArtifactMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // data: optional message org.apache.beam.model.job_management.v1.ArtifactChunk
  pub fn has_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn data_opt(&self) -> ::protobuf::Optional<super::ArtifactChunkView<'_>> {
        ::protobuf::Optional::new(self.data(), self.has_data())
  }
  pub fn data(&self) -> super::ArtifactChunkView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactChunkView::default())
  }
  pub fn data_mut(&mut self) -> super::ArtifactChunkMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArtifactChunk>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn content(&self) -> super::put_artifact_request::ContentOneof<'_> {
    match &self.content_case() {
      super::put_artifact_request::ContentCase::Metadata =>
          super::put_artifact_request::ContentOneof::Metadata(self.metadata()),
      super::put_artifact_request::ContentCase::Data =>
          super::put_artifact_request::ContentOneof::Data(self.data()),
      _ => super::put_artifact_request::ContentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn content_case(&self) -> super::put_artifact_request::ContentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::put_artifact_request::ContentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PutArtifactRequestMut` does not perform any shared mutation.
unsafe impl Send for PutArtifactRequestMut<'_> {}

// SAFETY:
// - `PutArtifactRequestMut` does not perform any shared mutation.
unsafe impl Sync for PutArtifactRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for PutArtifactRequestMut<'msg> {
  type Proxied = PutArtifactRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PutArtifactRequest> {
    PutArtifactRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PutArtifactRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PutArtifactRequest>
  where
      'msg: 'shorter {
    PutArtifactRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PutArtifactRequestMut<'msg> {
  type MutProxied = PutArtifactRequest;
  fn as_mut(&mut self) -> PutArtifactRequestMut<'msg> {
    PutArtifactRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PutArtifactRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PutArtifactRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PutArtifactRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PutArtifactRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PutArtifactRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PutArtifactRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // metadata: optional message org.apache.beam.model.job_management.v1.PutArtifactMetadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn metadata_opt(&self) -> ::protobuf::Optional<super::PutArtifactMetadataView<'_>> {
        ::protobuf::Optional::new(self.metadata(), self.has_metadata())
  }
  pub fn metadata(&self) -> super::PutArtifactMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PutArtifactMetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> super::PutArtifactMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<super::PutArtifactMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // data: optional message org.apache.beam.model.job_management.v1.ArtifactChunk
  pub fn has_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn data_opt(&self) -> ::protobuf::Optional<super::ArtifactChunkView<'_>> {
        ::protobuf::Optional::new(self.data(), self.has_data())
  }
  pub fn data(&self) -> super::ArtifactChunkView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArtifactChunkView::default())
  }
  pub fn data_mut(&mut self) -> super::ArtifactChunkMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArtifactChunk>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn content(&self) -> super::put_artifact_request::ContentOneof<'_> {
    match &self.content_case() {
      super::put_artifact_request::ContentCase::Metadata =>
          super::put_artifact_request::ContentOneof::Metadata(self.metadata()),
      super::put_artifact_request::ContentCase::Data =>
          super::put_artifact_request::ContentOneof::Data(self.data()),
      _ => super::put_artifact_request::ContentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn content_case(&self) -> super::put_artifact_request::ContentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::put_artifact_request::ContentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl PutArtifactRequest

impl ::std::ops::Drop for PutArtifactRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PutArtifactRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PutArtifactRequest {
  type Proxied = Self;
  fn as_view(&self) -> PutArtifactRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PutArtifactRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PutArtifactRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PutArtifactRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__PutArtifactRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__PutArtifactRequest_msg_init.0, &[<super::PutArtifactMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ArtifactChunk as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__PutArtifactRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PutArtifactRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PutArtifactRequest {
  type Msg = PutArtifactRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactRequest {
  type Msg = PutArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PutArtifactRequestMut<'_> {
  type Msg = PutArtifactRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactRequestMut<'_> {
  type Msg = PutArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactRequestView<'_> {
  type Msg = PutArtifactRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PutArtifactRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod put_artifact_request {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ContentOneof<'msg> {
  Metadata(::protobuf::View<'msg, super::super::PutArtifactMetadata>) = 1,
  Data(::protobuf::View<'msg, super::super::ArtifactChunk>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ContentCase {
  Metadata = 1,
  Data = 2,

  not_set = 0
}

impl ContentCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ContentCase> {
    match v {
      0 => Some(ContentCase::not_set),
      1 => Some(ContentCase::Metadata),
      2 => Some(ContentCase::Data),
      _ => None
    }
  }
}
}  // pub mod put_artifact_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__PutArtifactResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PutArtifactResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PutArtifactResponse>
}

impl ::protobuf::Message for PutArtifactResponse {}

impl ::std::default::Default for PutArtifactResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PutArtifactResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PutArtifactResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PutArtifactResponseMut`.
unsafe impl Sync for PutArtifactResponse {}

// SAFETY:
// - `PutArtifactResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PutArtifactResponse {}

impl ::protobuf::Proxied for PutArtifactResponse {
  type View<'msg> = PutArtifactResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PutArtifactResponse {}

impl ::protobuf::MutProxied for PutArtifactResponse {
  type Mut<'msg> = PutArtifactResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PutArtifactResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PutArtifactResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PutArtifactResponseView<'msg> {
  type Message = PutArtifactResponse;
}

impl ::std::fmt::Debug for PutArtifactResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PutArtifactResponseView<'_> {
  fn default() -> PutArtifactResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactResponse>> for PutArtifactResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PutArtifactResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PutArtifactResponseView<'msg> {

  pub fn to_owned(&self) -> PutArtifactResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `PutArtifactResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PutArtifactResponseView<'_> {}

// SAFETY:
// - `PutArtifactResponseView` is `Send` because while its alive a `PutArtifactResponseMut` cannot.
// - `PutArtifactResponseView` does not use thread-local data.
unsafe impl Send for PutArtifactResponseView<'_> {}

impl<'msg> ::protobuf::AsView for PutArtifactResponseView<'msg> {
  type Proxied = PutArtifactResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PutArtifactResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PutArtifactResponseView<'msg> {
  fn into_view<'shorter>(self) -> PutArtifactResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PutArtifactResponse> for PutArtifactResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PutArtifactResponse {
    let mut dst = PutArtifactResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PutArtifactResponse> for PutArtifactResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PutArtifactResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PutArtifactResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PutArtifactResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PutArtifactResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PutArtifactResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PutArtifactResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PutArtifactResponseMut<'msg> {
  type Message = PutArtifactResponse;
}

impl ::std::fmt::Debug for PutArtifactResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactResponse>> for PutArtifactResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PutArtifactResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PutArtifactResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PutArtifactResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `PutArtifactResponseMut` does not perform any shared mutation.
unsafe impl Send for PutArtifactResponseMut<'_> {}

// SAFETY:
// - `PutArtifactResponseMut` does not perform any shared mutation.
unsafe impl Sync for PutArtifactResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for PutArtifactResponseMut<'msg> {
  type Proxied = PutArtifactResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PutArtifactResponse> {
    PutArtifactResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PutArtifactResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PutArtifactResponse>
  where
      'msg: 'shorter {
    PutArtifactResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PutArtifactResponseMut<'msg> {
  type MutProxied = PutArtifactResponse;
  fn as_mut(&mut self) -> PutArtifactResponseMut<'msg> {
    PutArtifactResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PutArtifactResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PutArtifactResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PutArtifactResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PutArtifactResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PutArtifactResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PutArtifactResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl PutArtifactResponse

impl ::std::ops::Drop for PutArtifactResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PutArtifactResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PutArtifactResponse {
  type Proxied = Self;
  fn as_view(&self) -> PutArtifactResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PutArtifactResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PutArtifactResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PutArtifactResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__PutArtifactResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__PutArtifactResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__PutArtifactResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PutArtifactResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PutArtifactResponse {
  type Msg = PutArtifactResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactResponse {
  type Msg = PutArtifactResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PutArtifactResponseMut<'_> {
  type Msg = PutArtifactResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactResponseMut<'_> {
  type Msg = PutArtifactResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PutArtifactResponseView<'_> {
  type Msg = PutArtifactResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PutArtifactResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PutArtifactResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__CommitManifestRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CommitManifestRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CommitManifestRequest>
}

impl ::protobuf::Message for CommitManifestRequest {}

impl ::std::default::Default for CommitManifestRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CommitManifestRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CommitManifestRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CommitManifestRequestMut`.
unsafe impl Sync for CommitManifestRequest {}

// SAFETY:
// - `CommitManifestRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CommitManifestRequest {}

impl ::protobuf::Proxied for CommitManifestRequest {
  type View<'msg> = CommitManifestRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CommitManifestRequest {}

impl ::protobuf::MutProxied for CommitManifestRequest {
  type Mut<'msg> = CommitManifestRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CommitManifestRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommitManifestRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommitManifestRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CommitManifestRequestView<'msg> {
  type Message = CommitManifestRequest;
}

impl ::std::fmt::Debug for CommitManifestRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CommitManifestRequestView<'_> {
  fn default() -> CommitManifestRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CommitManifestRequest>> for CommitManifestRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommitManifestRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommitManifestRequestView<'msg> {

  pub fn to_owned(&self) -> CommitManifestRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn manifest_opt(self) -> ::protobuf::Optional<super::ManifestView<'msg>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(self) -> super::ManifestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }

  // staging_session_token: optional string
  pub fn staging_session_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `CommitManifestRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for CommitManifestRequestView<'_> {}

// SAFETY:
// - `CommitManifestRequestView` is `Send` because while its alive a `CommitManifestRequestMut` cannot.
// - `CommitManifestRequestView` does not use thread-local data.
unsafe impl Send for CommitManifestRequestView<'_> {}

impl<'msg> ::protobuf::AsView for CommitManifestRequestView<'msg> {
  type Proxied = CommitManifestRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, CommitManifestRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommitManifestRequestView<'msg> {
  fn into_view<'shorter>(self) -> CommitManifestRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CommitManifestRequest> for CommitManifestRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommitManifestRequest {
    let mut dst = CommitManifestRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CommitManifestRequest> for CommitManifestRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommitManifestRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for CommitManifestRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CommitManifestRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CommitManifestRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CommitManifestRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommitManifestRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CommitManifestRequestMut<'msg> {
  type Message = CommitManifestRequest;
}

impl ::std::fmt::Debug for CommitManifestRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestRequest>> for CommitManifestRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommitManifestRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> CommitManifestRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_manifest(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn manifest_opt(&self) -> ::protobuf::Optional<super::ManifestView<'_>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(&self) -> super::ManifestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }
  pub fn manifest_mut(&mut self) -> super::ManifestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_manifest(&mut self,
    val: impl ::protobuf::IntoProxied<super::Manifest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // staging_session_token: optional string
  pub fn staging_session_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_session_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `CommitManifestRequestMut` does not perform any shared mutation.
unsafe impl Send for CommitManifestRequestMut<'_> {}

// SAFETY:
// - `CommitManifestRequestMut` does not perform any shared mutation.
unsafe impl Sync for CommitManifestRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for CommitManifestRequestMut<'msg> {
  type Proxied = CommitManifestRequest;
  fn as_view(&self) -> ::protobuf::View<'_, CommitManifestRequest> {
    CommitManifestRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommitManifestRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CommitManifestRequest>
  where
      'msg: 'shorter {
    CommitManifestRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CommitManifestRequestMut<'msg> {
  type MutProxied = CommitManifestRequest;
  fn as_mut(&mut self) -> CommitManifestRequestMut<'msg> {
    CommitManifestRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CommitManifestRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> CommitManifestRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CommitManifestRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CommitManifestRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CommitManifestRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CommitManifestRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // manifest: optional message org.apache.beam.model.job_management.v1.Manifest
  pub fn has_manifest(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_manifest(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn manifest_opt(&self) -> ::protobuf::Optional<super::ManifestView<'_>> {
        ::protobuf::Optional::new(self.manifest(), self.has_manifest())
  }
  pub fn manifest(&self) -> super::ManifestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ManifestView::default())
  }
  pub fn manifest_mut(&mut self) -> super::ManifestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_manifest(&mut self,
    val: impl ::protobuf::IntoProxied<super::Manifest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // staging_session_token: optional string
  pub fn staging_session_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_session_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl CommitManifestRequest

impl ::std::ops::Drop for CommitManifestRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CommitManifestRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CommitManifestRequest {
  type Proxied = Self;
  fn as_view(&self) -> CommitManifestRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CommitManifestRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CommitManifestRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CommitManifestRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__CommitManifestRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__CommitManifestRequest_msg_init.0, &[<super::Manifest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__CommitManifestRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommitManifestRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommitManifestRequest {
  type Msg = CommitManifestRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommitManifestRequest {
  type Msg = CommitManifestRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommitManifestRequestMut<'_> {
  type Msg = CommitManifestRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommitManifestRequestMut<'_> {
  type Msg = CommitManifestRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommitManifestRequestView<'_> {
  type Msg = CommitManifestRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommitManifestRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__CommitManifestResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CommitManifestResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CommitManifestResponse>
}

impl ::protobuf::Message for CommitManifestResponse {}

impl ::std::default::Default for CommitManifestResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CommitManifestResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CommitManifestResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CommitManifestResponseMut`.
unsafe impl Sync for CommitManifestResponse {}

// SAFETY:
// - `CommitManifestResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CommitManifestResponse {}

impl ::protobuf::Proxied for CommitManifestResponse {
  type View<'msg> = CommitManifestResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CommitManifestResponse {}

impl ::protobuf::MutProxied for CommitManifestResponse {
  type Mut<'msg> = CommitManifestResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CommitManifestResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommitManifestResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommitManifestResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CommitManifestResponseView<'msg> {
  type Message = CommitManifestResponse;
}

impl ::std::fmt::Debug for CommitManifestResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CommitManifestResponseView<'_> {
  fn default() -> CommitManifestResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CommitManifestResponse>> for CommitManifestResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommitManifestResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommitManifestResponseView<'msg> {

  pub fn to_owned(&self) -> CommitManifestResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // retrieval_token: optional string
  pub fn retrieval_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `CommitManifestResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for CommitManifestResponseView<'_> {}

// SAFETY:
// - `CommitManifestResponseView` is `Send` because while its alive a `CommitManifestResponseMut` cannot.
// - `CommitManifestResponseView` does not use thread-local data.
unsafe impl Send for CommitManifestResponseView<'_> {}

impl<'msg> ::protobuf::AsView for CommitManifestResponseView<'msg> {
  type Proxied = CommitManifestResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, CommitManifestResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommitManifestResponseView<'msg> {
  fn into_view<'shorter>(self) -> CommitManifestResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CommitManifestResponse> for CommitManifestResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommitManifestResponse {
    let mut dst = CommitManifestResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CommitManifestResponse> for CommitManifestResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommitManifestResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for CommitManifestResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CommitManifestResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CommitManifestResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CommitManifestResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommitManifestResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CommitManifestResponseMut<'msg> {
  type Message = CommitManifestResponse;
}

impl ::std::fmt::Debug for CommitManifestResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestResponse>> for CommitManifestResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommitManifestResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CommitManifestResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> CommitManifestResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // retrieval_token: optional string
  pub fn retrieval_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_retrieval_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `CommitManifestResponseMut` does not perform any shared mutation.
unsafe impl Send for CommitManifestResponseMut<'_> {}

// SAFETY:
// - `CommitManifestResponseMut` does not perform any shared mutation.
unsafe impl Sync for CommitManifestResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for CommitManifestResponseMut<'msg> {
  type Proxied = CommitManifestResponse;
  fn as_view(&self) -> ::protobuf::View<'_, CommitManifestResponse> {
    CommitManifestResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommitManifestResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CommitManifestResponse>
  where
      'msg: 'shorter {
    CommitManifestResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CommitManifestResponseMut<'msg> {
  type MutProxied = CommitManifestResponse;
  fn as_mut(&mut self) -> CommitManifestResponseMut<'msg> {
    CommitManifestResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CommitManifestResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> CommitManifestResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CommitManifestResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CommitManifestResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CommitManifestResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CommitManifestResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // retrieval_token: optional string
  pub fn retrieval_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_retrieval_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl CommitManifestResponse

impl ::std::ops::Drop for CommitManifestResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CommitManifestResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CommitManifestResponse {
  type Proxied = Self;
  fn as_view(&self) -> CommitManifestResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CommitManifestResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CommitManifestResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CommitManifestResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__CommitManifestResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__CommitManifestResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__CommitManifestResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommitManifestResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommitManifestResponse {
  type Msg = CommitManifestResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommitManifestResponse {
  type Msg = CommitManifestResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommitManifestResponseMut<'_> {
  type Msg = CommitManifestResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommitManifestResponseMut<'_> {
  type Msg = CommitManifestResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommitManifestResponseView<'_> {
  type Msg = CommitManifestResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommitManifestResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommitManifestResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod commit_manifest_response {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Constants(i32);

#[allow(non_upper_case_globals)]
impl Constants {
  pub const NoArtifactsStagedToken: Constants = Constants(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "NoArtifactsStagedToken",
      _ => return None
    })
  }
}

impl ::std::convert::From<Constants> for i32 {
  fn from(val: Constants) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Constants {
  fn from(val: i32) -> Constants {
    Self(val)
  }
}

impl ::std::default::Default for Constants {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Constants {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Constants::{}", constant_name)
    } else {
      write!(f, "Constants::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Constants {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Constants {}

impl ::protobuf::Proxied for Constants {
  type View<'a> = Constants;
}

impl ::protobuf::AsView for Constants {
  type Proxied = Constants;

  fn as_view(&self) -> Constants {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Constants {
  fn into_view<'shorter>(self) -> Constants where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Constants {
  const NAME: &'static str = "Constants";

  fn is_known(value: i32) -> bool {
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Constants {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod commit_manifest_response


