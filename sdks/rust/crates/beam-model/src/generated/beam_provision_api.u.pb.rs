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
pub(crate) static mut org__apache__beam__model__fn_0execution__v1__GetProvisionInfoRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetProvisionInfoRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetProvisionInfoRequest>
}

impl ::protobuf::Message for GetProvisionInfoRequest {}

impl ::std::default::Default for GetProvisionInfoRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetProvisionInfoRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetProvisionInfoRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetProvisionInfoRequestMut`.
unsafe impl Sync for GetProvisionInfoRequest {}

// SAFETY:
// - `GetProvisionInfoRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetProvisionInfoRequest {}

impl ::protobuf::Proxied for GetProvisionInfoRequest {
  type View<'msg> = GetProvisionInfoRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetProvisionInfoRequest {}

impl ::protobuf::MutProxied for GetProvisionInfoRequest {
  type Mut<'msg> = GetProvisionInfoRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetProvisionInfoRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetProvisionInfoRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetProvisionInfoRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetProvisionInfoRequestView<'msg> {
  type Message = GetProvisionInfoRequest;
}

impl ::std::fmt::Debug for GetProvisionInfoRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetProvisionInfoRequestView<'_> {
  fn default() -> GetProvisionInfoRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetProvisionInfoRequest>> for GetProvisionInfoRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetProvisionInfoRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetProvisionInfoRequestView<'msg> {

  pub fn to_owned(&self) -> GetProvisionInfoRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetProvisionInfoRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetProvisionInfoRequestView<'_> {}

// SAFETY:
// - `GetProvisionInfoRequestView` is `Send` because while its alive a `GetProvisionInfoRequestMut` cannot.
// - `GetProvisionInfoRequestView` does not use thread-local data.
unsafe impl Send for GetProvisionInfoRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetProvisionInfoRequestView<'msg> {
  type Proxied = GetProvisionInfoRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetProvisionInfoRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetProvisionInfoRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetProvisionInfoRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetProvisionInfoRequest> for GetProvisionInfoRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetProvisionInfoRequest {
    let mut dst = GetProvisionInfoRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetProvisionInfoRequest> for GetProvisionInfoRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetProvisionInfoRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetProvisionInfoRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetProvisionInfoRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetProvisionInfoRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetProvisionInfoRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetProvisionInfoRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetProvisionInfoRequestMut<'msg> {
  type Message = GetProvisionInfoRequest;
}

impl ::std::fmt::Debug for GetProvisionInfoRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoRequest>> for GetProvisionInfoRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetProvisionInfoRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetProvisionInfoRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `GetProvisionInfoRequestMut` does not perform any shared mutation.
unsafe impl Send for GetProvisionInfoRequestMut<'_> {}

// SAFETY:
// - `GetProvisionInfoRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetProvisionInfoRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetProvisionInfoRequestMut<'msg> {
  type Proxied = GetProvisionInfoRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetProvisionInfoRequest> {
    GetProvisionInfoRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetProvisionInfoRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetProvisionInfoRequest>
  where
      'msg: 'shorter {
    GetProvisionInfoRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetProvisionInfoRequestMut<'msg> {
  type MutProxied = GetProvisionInfoRequest;
  fn as_mut(&mut self) -> GetProvisionInfoRequestMut<'msg> {
    GetProvisionInfoRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetProvisionInfoRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetProvisionInfoRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetProvisionInfoRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetProvisionInfoRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetProvisionInfoRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetProvisionInfoRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl GetProvisionInfoRequest

impl ::std::ops::Drop for GetProvisionInfoRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetProvisionInfoRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetProvisionInfoRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetProvisionInfoRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetProvisionInfoRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetProvisionInfoRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetProvisionInfoRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__fn_0execution__v1__GetProvisionInfoRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__fn_0execution__v1__GetProvisionInfoRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__fn_0execution__v1__GetProvisionInfoRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetProvisionInfoRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetProvisionInfoRequest {
  type Msg = GetProvisionInfoRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetProvisionInfoRequest {
  type Msg = GetProvisionInfoRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetProvisionInfoRequestMut<'_> {
  type Msg = GetProvisionInfoRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetProvisionInfoRequestMut<'_> {
  type Msg = GetProvisionInfoRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetProvisionInfoRequestView<'_> {
  type Msg = GetProvisionInfoRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetProvisionInfoRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__fn_0execution__v1__GetProvisionInfoResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetProvisionInfoResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetProvisionInfoResponse>
}

impl ::protobuf::Message for GetProvisionInfoResponse {}

impl ::std::default::Default for GetProvisionInfoResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetProvisionInfoResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetProvisionInfoResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetProvisionInfoResponseMut`.
unsafe impl Sync for GetProvisionInfoResponse {}

// SAFETY:
// - `GetProvisionInfoResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetProvisionInfoResponse {}

impl ::protobuf::Proxied for GetProvisionInfoResponse {
  type View<'msg> = GetProvisionInfoResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetProvisionInfoResponse {}

impl ::protobuf::MutProxied for GetProvisionInfoResponse {
  type Mut<'msg> = GetProvisionInfoResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetProvisionInfoResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetProvisionInfoResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetProvisionInfoResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetProvisionInfoResponseView<'msg> {
  type Message = GetProvisionInfoResponse;
}

impl ::std::fmt::Debug for GetProvisionInfoResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetProvisionInfoResponseView<'_> {
  fn default() -> GetProvisionInfoResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetProvisionInfoResponse>> for GetProvisionInfoResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetProvisionInfoResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetProvisionInfoResponseView<'msg> {

  pub fn to_owned(&self) -> GetProvisionInfoResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // info: optional message org.apache.beam.model.fn_execution.v1.ProvisionInfo
  pub fn has_info(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn info_opt(self) -> ::protobuf::Optional<super::ProvisionInfoView<'msg>> {
        ::protobuf::Optional::new(self.info(), self.has_info())
  }
  pub fn info(self) -> super::ProvisionInfoView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProvisionInfoView::default())
  }

}

// SAFETY:
// - `GetProvisionInfoResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetProvisionInfoResponseView<'_> {}

// SAFETY:
// - `GetProvisionInfoResponseView` is `Send` because while its alive a `GetProvisionInfoResponseMut` cannot.
// - `GetProvisionInfoResponseView` does not use thread-local data.
unsafe impl Send for GetProvisionInfoResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GetProvisionInfoResponseView<'msg> {
  type Proxied = GetProvisionInfoResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetProvisionInfoResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetProvisionInfoResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetProvisionInfoResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetProvisionInfoResponse> for GetProvisionInfoResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetProvisionInfoResponse {
    let mut dst = GetProvisionInfoResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetProvisionInfoResponse> for GetProvisionInfoResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetProvisionInfoResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetProvisionInfoResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetProvisionInfoResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetProvisionInfoResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetProvisionInfoResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetProvisionInfoResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetProvisionInfoResponseMut<'msg> {
  type Message = GetProvisionInfoResponse;
}

impl ::std::fmt::Debug for GetProvisionInfoResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoResponse>> for GetProvisionInfoResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetProvisionInfoResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetProvisionInfoResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GetProvisionInfoResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // info: optional message org.apache.beam.model.fn_execution.v1.ProvisionInfo
  pub fn has_info(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_info(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn info_opt(&self) -> ::protobuf::Optional<super::ProvisionInfoView<'_>> {
        ::protobuf::Optional::new(self.info(), self.has_info())
  }
  pub fn info(&self) -> super::ProvisionInfoView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProvisionInfoView::default())
  }
  pub fn info_mut(&mut self) -> super::ProvisionInfoMut<'_> {
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
  pub fn set_info(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProvisionInfo>) {

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
// - `GetProvisionInfoResponseMut` does not perform any shared mutation.
unsafe impl Send for GetProvisionInfoResponseMut<'_> {}

// SAFETY:
// - `GetProvisionInfoResponseMut` does not perform any shared mutation.
unsafe impl Sync for GetProvisionInfoResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GetProvisionInfoResponseMut<'msg> {
  type Proxied = GetProvisionInfoResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetProvisionInfoResponse> {
    GetProvisionInfoResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetProvisionInfoResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetProvisionInfoResponse>
  where
      'msg: 'shorter {
    GetProvisionInfoResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetProvisionInfoResponseMut<'msg> {
  type MutProxied = GetProvisionInfoResponse;
  fn as_mut(&mut self) -> GetProvisionInfoResponseMut<'msg> {
    GetProvisionInfoResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetProvisionInfoResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetProvisionInfoResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetProvisionInfoResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetProvisionInfoResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetProvisionInfoResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetProvisionInfoResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // info: optional message org.apache.beam.model.fn_execution.v1.ProvisionInfo
  pub fn has_info(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_info(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn info_opt(&self) -> ::protobuf::Optional<super::ProvisionInfoView<'_>> {
        ::protobuf::Optional::new(self.info(), self.has_info())
  }
  pub fn info(&self) -> super::ProvisionInfoView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProvisionInfoView::default())
  }
  pub fn info_mut(&mut self) -> super::ProvisionInfoMut<'_> {
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
  pub fn set_info(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProvisionInfo>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GetProvisionInfoResponse

impl ::std::ops::Drop for GetProvisionInfoResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetProvisionInfoResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetProvisionInfoResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetProvisionInfoResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetProvisionInfoResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetProvisionInfoResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetProvisionInfoResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__fn_0execution__v1__GetProvisionInfoResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__fn_0execution__v1__GetProvisionInfoResponse_msg_init.0, &[<super::ProvisionInfo as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__fn_0execution__v1__GetProvisionInfoResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetProvisionInfoResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetProvisionInfoResponse {
  type Msg = GetProvisionInfoResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetProvisionInfoResponse {
  type Msg = GetProvisionInfoResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetProvisionInfoResponseMut<'_> {
  type Msg = GetProvisionInfoResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetProvisionInfoResponseMut<'_> {
  type Msg = GetProvisionInfoResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetProvisionInfoResponseView<'_> {
  type Msg = GetProvisionInfoResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetProvisionInfoResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetProvisionInfoResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__fn_0execution__v1__ProvisionInfo_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProvisionInfo {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProvisionInfo>
}

impl ::protobuf::Message for ProvisionInfo {}

impl ::std::default::Default for ProvisionInfo {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProvisionInfo {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProvisionInfo` is `Sync` because it does not implement interior mutability.
//    Neither does `ProvisionInfoMut`.
unsafe impl Sync for ProvisionInfo {}

// SAFETY:
// - `ProvisionInfo` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ProvisionInfo {}

impl ::protobuf::Proxied for ProvisionInfo {
  type View<'msg> = ProvisionInfoView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProvisionInfo {}

impl ::protobuf::MutProxied for ProvisionInfo {
  type Mut<'msg> = ProvisionInfoMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProvisionInfoView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProvisionInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProvisionInfoView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProvisionInfoView<'msg> {
  type Message = ProvisionInfo;
}

impl ::std::fmt::Debug for ProvisionInfoView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProvisionInfoView<'_> {
  fn default() -> ProvisionInfoView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProvisionInfo>> for ProvisionInfoView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProvisionInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProvisionInfoView<'msg> {

  pub fn to_owned(&self) -> ProvisionInfo {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn pipeline_options_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'msg>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
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

  // status_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_status_endpoint(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn status_endpoint_opt(self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'msg>> {
        ::protobuf::Optional::new(self.status_endpoint(), self.has_status_endpoint())
  }
  pub fn status_endpoint(self) -> super::ApiServiceDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }

  // logging_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_logging_endpoint(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn logging_endpoint_opt(self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'msg>> {
        ::protobuf::Optional::new(self.logging_endpoint(), self.has_logging_endpoint())
  }
  pub fn logging_endpoint(self) -> super::ApiServiceDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }

  // artifact_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_artifact_endpoint(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn artifact_endpoint_opt(self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'msg>> {
        ::protobuf::Optional::new(self.artifact_endpoint(), self.has_artifact_endpoint())
  }
  pub fn artifact_endpoint(self) -> super::ApiServiceDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }

  // control_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_control_endpoint(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn control_endpoint_opt(self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'msg>> {
        ::protobuf::Optional::new(self.control_endpoint(), self.has_control_endpoint())
  }
  pub fn control_endpoint(self) -> super::ApiServiceDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }

  // dependencies: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn dependencies(self) -> ::protobuf::RepeatedView<'msg, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // runner_capabilities: repeated string
  pub fn runner_capabilities(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // metadata: repeated message org.apache.beam.model.fn_execution.v1.ProvisionInfo.MetadataEntry
  pub fn metadata(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(8)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // sibling_worker_ids: repeated string
  pub fn sibling_worker_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
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
// - `ProvisionInfoView` is `Sync` because it does not support mutation.
unsafe impl Sync for ProvisionInfoView<'_> {}

// SAFETY:
// - `ProvisionInfoView` is `Send` because while its alive a `ProvisionInfoMut` cannot.
// - `ProvisionInfoView` does not use thread-local data.
unsafe impl Send for ProvisionInfoView<'_> {}

impl<'msg> ::protobuf::AsView for ProvisionInfoView<'msg> {
  type Proxied = ProvisionInfo;
  fn as_view(&self) -> ::protobuf::View<'msg, ProvisionInfo> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProvisionInfoView<'msg> {
  fn into_view<'shorter>(self) -> ProvisionInfoView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProvisionInfo> for ProvisionInfoView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProvisionInfo {
    let mut dst = ProvisionInfo::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProvisionInfo> for ProvisionInfoMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProvisionInfo {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ProvisionInfo {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ProvisionInfoView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ProvisionInfoMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProvisionInfoMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProvisionInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProvisionInfoMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProvisionInfoMut<'msg> {
  type Message = ProvisionInfo;
}

impl ::std::fmt::Debug for ProvisionInfoMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProvisionInfo>> for ProvisionInfoMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProvisionInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProvisionInfoMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProvisionInfo> {
    self.inner
  }

  pub fn to_owned(&self) -> ProvisionInfo {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
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

  // status_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_status_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_status_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn status_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.status_endpoint(), self.has_status_endpoint())
  }
  pub fn status_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn status_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
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
  pub fn set_status_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // logging_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_logging_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_logging_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn logging_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.logging_endpoint(), self.has_logging_endpoint())
  }
  pub fn logging_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn logging_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
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
  pub fn set_logging_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // artifact_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_artifact_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_artifact_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn artifact_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.artifact_endpoint(), self.has_artifact_endpoint())
  }
  pub fn artifact_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn artifact_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_artifact_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // control_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_control_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_control_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn control_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.control_endpoint(), self.has_control_endpoint())
  }
  pub fn control_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn control_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_control_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // dependencies: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn dependencies(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dependencies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactInformation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_dependencies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactInformation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // runner_capabilities: repeated string
  pub fn runner_capabilities(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn runner_capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_runner_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // metadata: repeated message org.apache.beam.model.fn_execution.v1.ProvisionInfo.MetadataEntry
  pub fn metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(8)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          8, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // sibling_worker_ids: repeated string
  pub fn sibling_worker_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn sibling_worker_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_sibling_worker_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

}

// SAFETY:
// - `ProvisionInfoMut` does not perform any shared mutation.
unsafe impl Send for ProvisionInfoMut<'_> {}

// SAFETY:
// - `ProvisionInfoMut` does not perform any shared mutation.
unsafe impl Sync for ProvisionInfoMut<'_> {}

impl<'msg> ::protobuf::AsView for ProvisionInfoMut<'msg> {
  type Proxied = ProvisionInfo;
  fn as_view(&self) -> ::protobuf::View<'_, ProvisionInfo> {
    ProvisionInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProvisionInfoMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProvisionInfo>
  where
      'msg: 'shorter {
    ProvisionInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ProvisionInfoMut<'msg> {
  type MutProxied = ProvisionInfo;
  fn as_mut(&mut self) -> ProvisionInfoMut<'msg> {
    ProvisionInfoMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProvisionInfoMut<'msg> {
  fn into_mut<'shorter>(self) -> ProvisionInfoMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProvisionInfo {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProvisionInfo> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProvisionInfoView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProvisionInfoMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
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

  // status_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_status_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_status_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn status_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.status_endpoint(), self.has_status_endpoint())
  }
  pub fn status_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn status_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
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
  pub fn set_status_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // logging_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_logging_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_logging_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn logging_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.logging_endpoint(), self.has_logging_endpoint())
  }
  pub fn logging_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn logging_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
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
  pub fn set_logging_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // artifact_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_artifact_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_artifact_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn artifact_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.artifact_endpoint(), self.has_artifact_endpoint())
  }
  pub fn artifact_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn artifact_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_artifact_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // control_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_control_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_control_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn control_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.control_endpoint(), self.has_control_endpoint())
  }
  pub fn control_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn control_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_control_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // dependencies: repeated message org.apache.beam.model.pipeline.v1.ArtifactInformation
  pub fn dependencies(&self) -> ::protobuf::RepeatedView<'_, super::ArtifactInformation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ArtifactInformation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dependencies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ArtifactInformation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_dependencies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ArtifactInformation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // runner_capabilities: repeated string
  pub fn runner_capabilities(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn runner_capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_runner_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // metadata: repeated message org.apache.beam.model.fn_execution.v1.ProvisionInfo.MetadataEntry
  pub fn metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(8)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          8, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // sibling_worker_ids: repeated string
  pub fn sibling_worker_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn sibling_worker_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_sibling_worker_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

}  // impl ProvisionInfo

impl ::std::ops::Drop for ProvisionInfo {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProvisionInfo {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProvisionInfo {
  type Proxied = Self;
  fn as_view(&self) -> ProvisionInfoView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProvisionInfo {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProvisionInfoMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProvisionInfo {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__fn_0execution__v1__ProvisionInfo_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$b3b1X3333GETGET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__fn_0execution__v1__ProvisionInfo_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ApiServiceDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ApiServiceDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ApiServiceDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ApiServiceDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ArtifactInformation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::provision_info::MetadataEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__fn_0execution__v1__ProvisionInfo_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProvisionInfo {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProvisionInfo {
  type Msg = ProvisionInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProvisionInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProvisionInfo {
  type Msg = ProvisionInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProvisionInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProvisionInfoMut<'_> {
  type Msg = ProvisionInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProvisionInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProvisionInfoMut<'_> {
  type Msg = ProvisionInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProvisionInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProvisionInfoView<'_> {
  type Msg = ProvisionInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProvisionInfo> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProvisionInfoMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod provision_info {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__fn_0execution__v1__ProvisionInfo__MetadataEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct MetadataEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::provision_info::org__apache__beam__model__fn_0execution__v1__ProvisionInfo__MetadataEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::provision_info::org__apache__beam__model__fn_0execution__v1__ProvisionInfo__MetadataEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::provision_info::org__apache__beam__model__fn_0execution__v1__ProvisionInfo__MetadataEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod provision_info


