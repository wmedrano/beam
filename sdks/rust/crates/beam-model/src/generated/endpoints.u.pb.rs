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
pub(crate) static mut org__apache__beam__model__pipeline__v1__ApiServiceDescriptor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ApiServiceDescriptor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ApiServiceDescriptor>
}

impl ::protobuf::Message for ApiServiceDescriptor {}

impl ::std::default::Default for ApiServiceDescriptor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ApiServiceDescriptor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ApiServiceDescriptor` is `Sync` because it does not implement interior mutability.
//    Neither does `ApiServiceDescriptorMut`.
unsafe impl Sync for ApiServiceDescriptor {}

// SAFETY:
// - `ApiServiceDescriptor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ApiServiceDescriptor {}

impl ::protobuf::Proxied for ApiServiceDescriptor {
  type View<'msg> = ApiServiceDescriptorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ApiServiceDescriptor {}

impl ::protobuf::MutProxied for ApiServiceDescriptor {
  type Mut<'msg> = ApiServiceDescriptorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ApiServiceDescriptorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiServiceDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiServiceDescriptorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ApiServiceDescriptorView<'msg> {
  type Message = ApiServiceDescriptor;
}

impl ::std::fmt::Debug for ApiServiceDescriptorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ApiServiceDescriptorView<'_> {
  fn default() -> ApiServiceDescriptorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ApiServiceDescriptor>> for ApiServiceDescriptorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiServiceDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiServiceDescriptorView<'msg> {

  pub fn to_owned(&self) -> ApiServiceDescriptor {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // url: optional string
  pub fn url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // authentication: optional message org.apache.beam.model.pipeline.v1.AuthenticationSpec
  pub fn has_authentication(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn authentication_opt(self) -> ::protobuf::Optional<super::AuthenticationSpecView<'msg>> {
        ::protobuf::Optional::new(self.authentication(), self.has_authentication())
  }
  pub fn authentication(self) -> super::AuthenticationSpecView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthenticationSpecView::default())
  }

}

// SAFETY:
// - `ApiServiceDescriptorView` is `Sync` because it does not support mutation.
unsafe impl Sync for ApiServiceDescriptorView<'_> {}

// SAFETY:
// - `ApiServiceDescriptorView` is `Send` because while its alive a `ApiServiceDescriptorMut` cannot.
// - `ApiServiceDescriptorView` does not use thread-local data.
unsafe impl Send for ApiServiceDescriptorView<'_> {}

impl<'msg> ::protobuf::AsView for ApiServiceDescriptorView<'msg> {
  type Proxied = ApiServiceDescriptor;
  fn as_view(&self) -> ::protobuf::View<'msg, ApiServiceDescriptor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiServiceDescriptorView<'msg> {
  fn into_view<'shorter>(self) -> ApiServiceDescriptorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiServiceDescriptor> for ApiServiceDescriptorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiServiceDescriptor {
    let mut dst = ApiServiceDescriptor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiServiceDescriptor> for ApiServiceDescriptorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiServiceDescriptor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ApiServiceDescriptor {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ApiServiceDescriptorView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ApiServiceDescriptorMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ApiServiceDescriptorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiServiceDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiServiceDescriptorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ApiServiceDescriptorMut<'msg> {
  type Message = ApiServiceDescriptor;
}

impl ::std::fmt::Debug for ApiServiceDescriptorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ApiServiceDescriptor>> for ApiServiceDescriptorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiServiceDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiServiceDescriptorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiServiceDescriptor> {
    self.inner
  }

  pub fn to_owned(&self) -> ApiServiceDescriptor {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // url: optional string
  pub fn url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // authentication: optional message org.apache.beam.model.pipeline.v1.AuthenticationSpec
  pub fn has_authentication(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_authentication(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn authentication_opt(&self) -> ::protobuf::Optional<super::AuthenticationSpecView<'_>> {
        ::protobuf::Optional::new(self.authentication(), self.has_authentication())
  }
  pub fn authentication(&self) -> super::AuthenticationSpecView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthenticationSpecView::default())
  }
  pub fn authentication_mut(&mut self) -> super::AuthenticationSpecMut<'_> {
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
  pub fn set_authentication(&mut self,
    val: impl ::protobuf::IntoProxied<super::AuthenticationSpec>) {

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
// - `ApiServiceDescriptorMut` does not perform any shared mutation.
unsafe impl Send for ApiServiceDescriptorMut<'_> {}

// SAFETY:
// - `ApiServiceDescriptorMut` does not perform any shared mutation.
unsafe impl Sync for ApiServiceDescriptorMut<'_> {}

impl<'msg> ::protobuf::AsView for ApiServiceDescriptorMut<'msg> {
  type Proxied = ApiServiceDescriptor;
  fn as_view(&self) -> ::protobuf::View<'_, ApiServiceDescriptor> {
    ApiServiceDescriptorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiServiceDescriptorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ApiServiceDescriptor>
  where
      'msg: 'shorter {
    ApiServiceDescriptorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ApiServiceDescriptorMut<'msg> {
  type MutProxied = ApiServiceDescriptor;
  fn as_mut(&mut self) -> ApiServiceDescriptorMut<'msg> {
    ApiServiceDescriptorMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ApiServiceDescriptorMut<'msg> {
  fn into_mut<'shorter>(self) -> ApiServiceDescriptorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ApiServiceDescriptor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ApiServiceDescriptor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ApiServiceDescriptorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ApiServiceDescriptorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // url: optional string
  pub fn url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // authentication: optional message org.apache.beam.model.pipeline.v1.AuthenticationSpec
  pub fn has_authentication(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_authentication(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn authentication_opt(&self) -> ::protobuf::Optional<super::AuthenticationSpecView<'_>> {
        ::protobuf::Optional::new(self.authentication(), self.has_authentication())
  }
  pub fn authentication(&self) -> super::AuthenticationSpecView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthenticationSpecView::default())
  }
  pub fn authentication_mut(&mut self) -> super::AuthenticationSpecMut<'_> {
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
  pub fn set_authentication(&mut self,
    val: impl ::protobuf::IntoProxied<super::AuthenticationSpec>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ApiServiceDescriptor

impl ::std::ops::Drop for ApiServiceDescriptor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ApiServiceDescriptor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ApiServiceDescriptor {
  type Proxied = Self;
  fn as_view(&self) -> ApiServiceDescriptorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ApiServiceDescriptor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ApiServiceDescriptorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ApiServiceDescriptor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__ApiServiceDescriptor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__ApiServiceDescriptor_msg_init.0, &[<super::AuthenticationSpec as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__ApiServiceDescriptor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiServiceDescriptor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiServiceDescriptor {
  type Msg = ApiServiceDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiServiceDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiServiceDescriptor {
  type Msg = ApiServiceDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiServiceDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiServiceDescriptorMut<'_> {
  type Msg = ApiServiceDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiServiceDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiServiceDescriptorMut<'_> {
  type Msg = ApiServiceDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiServiceDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiServiceDescriptorView<'_> {
  type Msg = ApiServiceDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiServiceDescriptor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiServiceDescriptorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__AuthenticationSpec_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AuthenticationSpec {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AuthenticationSpec>
}

impl ::protobuf::Message for AuthenticationSpec {}

impl ::std::default::Default for AuthenticationSpec {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AuthenticationSpec {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AuthenticationSpec` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthenticationSpecMut`.
unsafe impl Sync for AuthenticationSpec {}

// SAFETY:
// - `AuthenticationSpec` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AuthenticationSpec {}

impl ::protobuf::Proxied for AuthenticationSpec {
  type View<'msg> = AuthenticationSpecView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AuthenticationSpec {}

impl ::protobuf::MutProxied for AuthenticationSpec {
  type Mut<'msg> = AuthenticationSpecMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthenticationSpecView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuthenticationSpec>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthenticationSpecView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AuthenticationSpecView<'msg> {
  type Message = AuthenticationSpec;
}

impl ::std::fmt::Debug for AuthenticationSpecView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AuthenticationSpecView<'_> {
  fn default() -> AuthenticationSpecView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AuthenticationSpec>> for AuthenticationSpecView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuthenticationSpec>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticationSpecView<'msg> {

  pub fn to_owned(&self) -> AuthenticationSpec {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // urn: optional string
  pub fn urn(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // payload: optional bytes
  pub fn payload(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `AuthenticationSpecView` is `Sync` because it does not support mutation.
unsafe impl Sync for AuthenticationSpecView<'_> {}

// SAFETY:
// - `AuthenticationSpecView` is `Send` because while its alive a `AuthenticationSpecMut` cannot.
// - `AuthenticationSpecView` does not use thread-local data.
unsafe impl Send for AuthenticationSpecView<'_> {}

impl<'msg> ::protobuf::AsView for AuthenticationSpecView<'msg> {
  type Proxied = AuthenticationSpec;
  fn as_view(&self) -> ::protobuf::View<'msg, AuthenticationSpec> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthenticationSpecView<'msg> {
  fn into_view<'shorter>(self) -> AuthenticationSpecView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AuthenticationSpec> for AuthenticationSpecView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuthenticationSpec {
    let mut dst = AuthenticationSpec::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AuthenticationSpec> for AuthenticationSpecMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuthenticationSpec {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AuthenticationSpec {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AuthenticationSpecView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AuthenticationSpecMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthenticationSpecMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthenticationSpec>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthenticationSpecMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AuthenticationSpecMut<'msg> {
  type Message = AuthenticationSpec;
}

impl ::std::fmt::Debug for AuthenticationSpecMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AuthenticationSpec>> for AuthenticationSpecMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthenticationSpec>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticationSpecMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthenticationSpec> {
    self.inner
  }

  pub fn to_owned(&self) -> AuthenticationSpec {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // urn: optional string
  pub fn urn(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_urn(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // payload: optional bytes
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `AuthenticationSpecMut` does not perform any shared mutation.
unsafe impl Send for AuthenticationSpecMut<'_> {}

// SAFETY:
// - `AuthenticationSpecMut` does not perform any shared mutation.
unsafe impl Sync for AuthenticationSpecMut<'_> {}

impl<'msg> ::protobuf::AsView for AuthenticationSpecMut<'msg> {
  type Proxied = AuthenticationSpec;
  fn as_view(&self) -> ::protobuf::View<'_, AuthenticationSpec> {
    AuthenticationSpecView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthenticationSpecMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AuthenticationSpec>
  where
      'msg: 'shorter {
    AuthenticationSpecView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AuthenticationSpecMut<'msg> {
  type MutProxied = AuthenticationSpec;
  fn as_mut(&mut self) -> AuthenticationSpecMut<'msg> {
    AuthenticationSpecMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AuthenticationSpecMut<'msg> {
  fn into_mut<'shorter>(self) -> AuthenticationSpecMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AuthenticationSpec {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AuthenticationSpec> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AuthenticationSpecView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AuthenticationSpecMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // urn: optional string
  pub fn urn(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_urn(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // payload: optional bytes
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl AuthenticationSpec

impl ::std::ops::Drop for AuthenticationSpec {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AuthenticationSpec {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AuthenticationSpec {
  type Proxied = Self;
  fn as_view(&self) -> AuthenticationSpecView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AuthenticationSpec {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AuthenticationSpecMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AuthenticationSpec {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__AuthenticationSpec_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__AuthenticationSpec_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__AuthenticationSpec_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthenticationSpec {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthenticationSpec {
  type Msg = AuthenticationSpec;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthenticationSpec> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthenticationSpec {
  type Msg = AuthenticationSpec;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthenticationSpec> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthenticationSpecMut<'_> {
  type Msg = AuthenticationSpec;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthenticationSpec> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthenticationSpecMut<'_> {
  type Msg = AuthenticationSpec;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthenticationSpec> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthenticationSpecView<'_> {
  type Msg = AuthenticationSpec;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthenticationSpec> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthenticationSpecMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



