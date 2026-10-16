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
pub(crate) static mut org__apache__beam__model__pipeline__v1__ExternalConfigurationPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExternalConfigurationPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExternalConfigurationPayload>
}

impl ::protobuf::Message for ExternalConfigurationPayload {}

impl ::std::default::Default for ExternalConfigurationPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExternalConfigurationPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExternalConfigurationPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `ExternalConfigurationPayloadMut`.
unsafe impl Sync for ExternalConfigurationPayload {}

// SAFETY:
// - `ExternalConfigurationPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ExternalConfigurationPayload {}

impl ::protobuf::Proxied for ExternalConfigurationPayload {
  type View<'msg> = ExternalConfigurationPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExternalConfigurationPayload {}

impl ::protobuf::MutProxied for ExternalConfigurationPayload {
  type Mut<'msg> = ExternalConfigurationPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExternalConfigurationPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExternalConfigurationPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExternalConfigurationPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExternalConfigurationPayloadView<'msg> {
  type Message = ExternalConfigurationPayload;
}

impl ::std::fmt::Debug for ExternalConfigurationPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExternalConfigurationPayloadView<'_> {
  fn default() -> ExternalConfigurationPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExternalConfigurationPayload>> for ExternalConfigurationPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExternalConfigurationPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExternalConfigurationPayloadView<'msg> {

  pub fn to_owned(&self) -> ExternalConfigurationPayload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
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
// - `ExternalConfigurationPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for ExternalConfigurationPayloadView<'_> {}

// SAFETY:
// - `ExternalConfigurationPayloadView` is `Send` because while its alive a `ExternalConfigurationPayloadMut` cannot.
// - `ExternalConfigurationPayloadView` does not use thread-local data.
unsafe impl Send for ExternalConfigurationPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for ExternalConfigurationPayloadView<'msg> {
  type Proxied = ExternalConfigurationPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, ExternalConfigurationPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExternalConfigurationPayloadView<'msg> {
  fn into_view<'shorter>(self) -> ExternalConfigurationPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExternalConfigurationPayload> for ExternalConfigurationPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExternalConfigurationPayload {
    let mut dst = ExternalConfigurationPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExternalConfigurationPayload> for ExternalConfigurationPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExternalConfigurationPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ExternalConfigurationPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExternalConfigurationPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExternalConfigurationPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExternalConfigurationPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalConfigurationPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExternalConfigurationPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExternalConfigurationPayloadMut<'msg> {
  type Message = ExternalConfigurationPayload;
}

impl ::std::fmt::Debug for ExternalConfigurationPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalConfigurationPayload>> for ExternalConfigurationPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalConfigurationPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExternalConfigurationPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalConfigurationPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> ExternalConfigurationPayload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
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
// - `ExternalConfigurationPayloadMut` does not perform any shared mutation.
unsafe impl Send for ExternalConfigurationPayloadMut<'_> {}

// SAFETY:
// - `ExternalConfigurationPayloadMut` does not perform any shared mutation.
unsafe impl Sync for ExternalConfigurationPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for ExternalConfigurationPayloadMut<'msg> {
  type Proxied = ExternalConfigurationPayload;
  fn as_view(&self) -> ::protobuf::View<'_, ExternalConfigurationPayload> {
    ExternalConfigurationPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExternalConfigurationPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExternalConfigurationPayload>
  where
      'msg: 'shorter {
    ExternalConfigurationPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ExternalConfigurationPayloadMut<'msg> {
  type MutProxied = ExternalConfigurationPayload;
  fn as_mut(&mut self) -> ExternalConfigurationPayloadMut<'msg> {
    ExternalConfigurationPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExternalConfigurationPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> ExternalConfigurationPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExternalConfigurationPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExternalConfigurationPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExternalConfigurationPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExternalConfigurationPayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
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

}  // impl ExternalConfigurationPayload

impl ::std::ops::Drop for ExternalConfigurationPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExternalConfigurationPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExternalConfigurationPayload {
  type Proxied = Self;
  fn as_view(&self) -> ExternalConfigurationPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExternalConfigurationPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExternalConfigurationPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExternalConfigurationPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__ExternalConfigurationPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$30P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__ExternalConfigurationPayload_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__ExternalConfigurationPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExternalConfigurationPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExternalConfigurationPayload {
  type Msg = ExternalConfigurationPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalConfigurationPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExternalConfigurationPayload {
  type Msg = ExternalConfigurationPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalConfigurationPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExternalConfigurationPayloadMut<'_> {
  type Msg = ExternalConfigurationPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalConfigurationPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExternalConfigurationPayloadMut<'_> {
  type Msg = ExternalConfigurationPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalConfigurationPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExternalConfigurationPayloadView<'_> {
  type Msg = ExternalConfigurationPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalConfigurationPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExternalConfigurationPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__ExpansionMethods_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExpansionMethods {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExpansionMethods>
}

impl ::protobuf::Message for ExpansionMethods {}

impl ::std::default::Default for ExpansionMethods {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExpansionMethods {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExpansionMethods` is `Sync` because it does not implement interior mutability.
//    Neither does `ExpansionMethodsMut`.
unsafe impl Sync for ExpansionMethods {}

// SAFETY:
// - `ExpansionMethods` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ExpansionMethods {}

impl ::protobuf::Proxied for ExpansionMethods {
  type View<'msg> = ExpansionMethodsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExpansionMethods {}

impl ::protobuf::MutProxied for ExpansionMethods {
  type Mut<'msg> = ExpansionMethodsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExpansionMethodsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionMethods>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpansionMethodsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExpansionMethodsView<'msg> {
  type Message = ExpansionMethods;
}

impl ::std::fmt::Debug for ExpansionMethodsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExpansionMethodsView<'_> {
  fn default() -> ExpansionMethodsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionMethods>> for ExpansionMethodsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionMethods>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpansionMethodsView<'msg> {

  pub fn to_owned(&self) -> ExpansionMethods {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ExpansionMethodsView` is `Sync` because it does not support mutation.
unsafe impl Sync for ExpansionMethodsView<'_> {}

// SAFETY:
// - `ExpansionMethodsView` is `Send` because while its alive a `ExpansionMethodsMut` cannot.
// - `ExpansionMethodsView` does not use thread-local data.
unsafe impl Send for ExpansionMethodsView<'_> {}

impl<'msg> ::protobuf::AsView for ExpansionMethodsView<'msg> {
  type Proxied = ExpansionMethods;
  fn as_view(&self) -> ::protobuf::View<'msg, ExpansionMethods> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpansionMethodsView<'msg> {
  fn into_view<'shorter>(self) -> ExpansionMethodsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpansionMethods> for ExpansionMethodsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpansionMethods {
    let mut dst = ExpansionMethods::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpansionMethods> for ExpansionMethodsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpansionMethods {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ExpansionMethods {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpansionMethodsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpansionMethodsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExpansionMethodsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionMethods>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpansionMethodsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExpansionMethodsMut<'msg> {
  type Message = ExpansionMethods;
}

impl ::std::fmt::Debug for ExpansionMethodsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionMethods>> for ExpansionMethodsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionMethods>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpansionMethodsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionMethods> {
    self.inner
  }

  pub fn to_owned(&self) -> ExpansionMethods {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ExpansionMethodsMut` does not perform any shared mutation.
unsafe impl Send for ExpansionMethodsMut<'_> {}

// SAFETY:
// - `ExpansionMethodsMut` does not perform any shared mutation.
unsafe impl Sync for ExpansionMethodsMut<'_> {}

impl<'msg> ::protobuf::AsView for ExpansionMethodsMut<'msg> {
  type Proxied = ExpansionMethods;
  fn as_view(&self) -> ::protobuf::View<'_, ExpansionMethods> {
    ExpansionMethodsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpansionMethodsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExpansionMethods>
  where
      'msg: 'shorter {
    ExpansionMethodsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ExpansionMethodsMut<'msg> {
  type MutProxied = ExpansionMethods;
  fn as_mut(&mut self) -> ExpansionMethodsMut<'msg> {
    ExpansionMethodsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExpansionMethodsMut<'msg> {
  fn into_mut<'shorter>(self) -> ExpansionMethodsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExpansionMethods {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExpansionMethods> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExpansionMethodsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExpansionMethodsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ExpansionMethods

impl ::std::ops::Drop for ExpansionMethods {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExpansionMethods {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExpansionMethods {
  type Proxied = Self;
  fn as_view(&self) -> ExpansionMethodsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExpansionMethods {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExpansionMethodsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExpansionMethods {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__ExpansionMethods_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__ExpansionMethods_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__ExpansionMethods_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpansionMethods {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpansionMethods {
  type Msg = ExpansionMethods;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionMethods> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionMethods {
  type Msg = ExpansionMethods;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionMethods> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpansionMethodsMut<'_> {
  type Msg = ExpansionMethods;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionMethods> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionMethodsMut<'_> {
  type Msg = ExpansionMethods;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionMethods> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionMethodsView<'_> {
  type Msg = ExpansionMethods;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionMethods> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpansionMethodsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod expansion_methods {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const JavaClassLookup: Enum = Enum(0);
  pub const SchemaTransform: Enum = Enum(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "JavaClassLookup",
      1 => "SchemaTransform",
      _ => return None
    })
  }
}

impl ::std::convert::From<Enum> for i32 {
  fn from(val: Enum) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Enum {
  fn from(val: i32) -> Enum {
    Self(val)
  }
}

impl ::std::default::Default for Enum {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Enum {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Enum::{}", constant_name)
    } else {
      write!(f, "Enum::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Enum {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Enum {}

impl ::protobuf::Proxied for Enum {
  type View<'a> = Enum;
}

impl ::protobuf::AsView for Enum {
  type Proxied = Enum;

  fn as_view(&self) -> Enum {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Enum {
  fn into_view<'shorter>(self) -> Enum where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Enum {
  const NAME: &'static str = "Enum";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod expansion_methods


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__ManagedTransforms_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ManagedTransforms {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ManagedTransforms>
}

impl ::protobuf::Message for ManagedTransforms {}

impl ::std::default::Default for ManagedTransforms {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ManagedTransforms {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ManagedTransforms` is `Sync` because it does not implement interior mutability.
//    Neither does `ManagedTransformsMut`.
unsafe impl Sync for ManagedTransforms {}

// SAFETY:
// - `ManagedTransforms` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ManagedTransforms {}

impl ::protobuf::Proxied for ManagedTransforms {
  type View<'msg> = ManagedTransformsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ManagedTransforms {}

impl ::protobuf::MutProxied for ManagedTransforms {
  type Mut<'msg> = ManagedTransformsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ManagedTransformsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ManagedTransforms>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ManagedTransformsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ManagedTransformsView<'msg> {
  type Message = ManagedTransforms;
}

impl ::std::fmt::Debug for ManagedTransformsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ManagedTransformsView<'_> {
  fn default() -> ManagedTransformsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ManagedTransforms>> for ManagedTransformsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ManagedTransforms>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ManagedTransformsView<'msg> {

  pub fn to_owned(&self) -> ManagedTransforms {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ManagedTransformsView` is `Sync` because it does not support mutation.
unsafe impl Sync for ManagedTransformsView<'_> {}

// SAFETY:
// - `ManagedTransformsView` is `Send` because while its alive a `ManagedTransformsMut` cannot.
// - `ManagedTransformsView` does not use thread-local data.
unsafe impl Send for ManagedTransformsView<'_> {}

impl<'msg> ::protobuf::AsView for ManagedTransformsView<'msg> {
  type Proxied = ManagedTransforms;
  fn as_view(&self) -> ::protobuf::View<'msg, ManagedTransforms> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ManagedTransformsView<'msg> {
  fn into_view<'shorter>(self) -> ManagedTransformsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ManagedTransforms> for ManagedTransformsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ManagedTransforms {
    let mut dst = ManagedTransforms::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ManagedTransforms> for ManagedTransformsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ManagedTransforms {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ManagedTransforms {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ManagedTransformsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ManagedTransformsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ManagedTransformsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ManagedTransforms>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ManagedTransformsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ManagedTransformsMut<'msg> {
  type Message = ManagedTransforms;
}

impl ::std::fmt::Debug for ManagedTransformsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ManagedTransforms>> for ManagedTransformsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ManagedTransforms>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ManagedTransformsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ManagedTransforms> {
    self.inner
  }

  pub fn to_owned(&self) -> ManagedTransforms {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ManagedTransformsMut` does not perform any shared mutation.
unsafe impl Send for ManagedTransformsMut<'_> {}

// SAFETY:
// - `ManagedTransformsMut` does not perform any shared mutation.
unsafe impl Sync for ManagedTransformsMut<'_> {}

impl<'msg> ::protobuf::AsView for ManagedTransformsMut<'msg> {
  type Proxied = ManagedTransforms;
  fn as_view(&self) -> ::protobuf::View<'_, ManagedTransforms> {
    ManagedTransformsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ManagedTransformsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ManagedTransforms>
  where
      'msg: 'shorter {
    ManagedTransformsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ManagedTransformsMut<'msg> {
  type MutProxied = ManagedTransforms;
  fn as_mut(&mut self) -> ManagedTransformsMut<'msg> {
    ManagedTransformsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ManagedTransformsMut<'msg> {
  fn into_mut<'shorter>(self) -> ManagedTransformsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ManagedTransforms {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ManagedTransforms> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ManagedTransformsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ManagedTransformsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ManagedTransforms

impl ::std::ops::Drop for ManagedTransforms {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ManagedTransforms {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ManagedTransforms {
  type Proxied = Self;
  fn as_view(&self) -> ManagedTransformsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ManagedTransforms {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ManagedTransformsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ManagedTransforms {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__ManagedTransforms_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__ManagedTransforms_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__ManagedTransforms_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ManagedTransforms {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ManagedTransforms {
  type Msg = ManagedTransforms;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ManagedTransforms> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ManagedTransforms {
  type Msg = ManagedTransforms;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ManagedTransforms> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ManagedTransformsMut<'_> {
  type Msg = ManagedTransforms;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ManagedTransforms> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ManagedTransformsMut<'_> {
  type Msg = ManagedTransforms;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ManagedTransforms> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ManagedTransformsView<'_> {
  type Msg = ManagedTransforms;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ManagedTransforms> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ManagedTransformsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod managed_transforms {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Urns(i32);

#[allow(non_upper_case_globals)]
impl Urns {
  pub const IcebergRead: Urns = Urns(0);
  pub const IcebergWrite: Urns = Urns(1);
  pub const KafkaRead: Urns = Urns(2);
  pub const KafkaWrite: Urns = Urns(3);
  pub const BigqueryRead: Urns = Urns(4);
  pub const BigqueryWrite: Urns = Urns(5);
  pub const IcebergCdcRead: Urns = Urns(6);
  pub const PostgresRead: Urns = Urns(7);
  pub const PostgresWrite: Urns = Urns(8);
  pub const MysqlRead: Urns = Urns(9);
  pub const MysqlWrite: Urns = Urns(10);
  pub const SqlServerRead: Urns = Urns(11);
  pub const SqlServerWrite: Urns = Urns(12);
  pub const DeltaLakeRead: Urns = Urns(13);
  pub const DeltaLakeCdcRead: Urns = Urns(14);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "IcebergRead",
      1 => "IcebergWrite",
      2 => "KafkaRead",
      3 => "KafkaWrite",
      4 => "BigqueryRead",
      5 => "BigqueryWrite",
      6 => "IcebergCdcRead",
      7 => "PostgresRead",
      8 => "PostgresWrite",
      9 => "MysqlRead",
      10 => "MysqlWrite",
      11 => "SqlServerRead",
      12 => "SqlServerWrite",
      13 => "DeltaLakeRead",
      14 => "DeltaLakeCdcRead",
      _ => return None
    })
  }
}

impl ::std::convert::From<Urns> for i32 {
  fn from(val: Urns) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Urns {
  fn from(val: i32) -> Urns {
    Self(val)
  }
}

impl ::std::default::Default for Urns {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Urns {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Urns::{}", constant_name)
    } else {
      write!(f, "Urns::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Urns {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Urns {}

impl ::protobuf::Proxied for Urns {
  type View<'a> = Urns;
}

impl ::protobuf::AsView for Urns {
  type Proxied = Urns;

  fn as_view(&self) -> Urns {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Urns {
  fn into_view<'shorter>(self) -> Urns where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Urns {
  const NAME: &'static str = "Urns";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11|12|13|14)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Urns {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod managed_transforms


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__JavaClassLookupPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JavaClassLookupPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JavaClassLookupPayload>
}

impl ::protobuf::Message for JavaClassLookupPayload {}

impl ::std::default::Default for JavaClassLookupPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JavaClassLookupPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JavaClassLookupPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `JavaClassLookupPayloadMut`.
unsafe impl Sync for JavaClassLookupPayload {}

// SAFETY:
// - `JavaClassLookupPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JavaClassLookupPayload {}

impl ::protobuf::Proxied for JavaClassLookupPayload {
  type View<'msg> = JavaClassLookupPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JavaClassLookupPayload {}

impl ::protobuf::MutProxied for JavaClassLookupPayload {
  type Mut<'msg> = JavaClassLookupPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JavaClassLookupPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JavaClassLookupPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JavaClassLookupPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JavaClassLookupPayloadView<'msg> {
  type Message = JavaClassLookupPayload;
}

impl ::std::fmt::Debug for JavaClassLookupPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JavaClassLookupPayloadView<'_> {
  fn default() -> JavaClassLookupPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JavaClassLookupPayload>> for JavaClassLookupPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JavaClassLookupPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JavaClassLookupPayloadView<'msg> {

  pub fn to_owned(&self) -> JavaClassLookupPayload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // class_name: optional string
  pub fn class_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // constructor_method: optional string
  pub fn constructor_method(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // constructor_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_constructor_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn constructor_schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.constructor_schema(), self.has_constructor_schema())
  }
  pub fn constructor_schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }

  // constructor_payload: optional bytes
  pub fn constructor_payload(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // builder_methods: repeated message org.apache.beam.model.pipeline.v1.BuilderMethod
  pub fn builder_methods(self) -> ::protobuf::RepeatedView<'msg, super::BuilderMethod> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::BuilderMethod>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `JavaClassLookupPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for JavaClassLookupPayloadView<'_> {}

// SAFETY:
// - `JavaClassLookupPayloadView` is `Send` because while its alive a `JavaClassLookupPayloadMut` cannot.
// - `JavaClassLookupPayloadView` does not use thread-local data.
unsafe impl Send for JavaClassLookupPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for JavaClassLookupPayloadView<'msg> {
  type Proxied = JavaClassLookupPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, JavaClassLookupPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JavaClassLookupPayloadView<'msg> {
  fn into_view<'shorter>(self) -> JavaClassLookupPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JavaClassLookupPayload> for JavaClassLookupPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JavaClassLookupPayload {
    let mut dst = JavaClassLookupPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JavaClassLookupPayload> for JavaClassLookupPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JavaClassLookupPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JavaClassLookupPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JavaClassLookupPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JavaClassLookupPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JavaClassLookupPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JavaClassLookupPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JavaClassLookupPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JavaClassLookupPayloadMut<'msg> {
  type Message = JavaClassLookupPayload;
}

impl ::std::fmt::Debug for JavaClassLookupPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JavaClassLookupPayload>> for JavaClassLookupPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JavaClassLookupPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JavaClassLookupPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JavaClassLookupPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> JavaClassLookupPayload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // class_name: optional string
  pub fn class_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_class_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // constructor_method: optional string
  pub fn constructor_method(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_constructor_method(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // constructor_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_constructor_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_constructor_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn constructor_schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.constructor_schema(), self.has_constructor_schema())
  }
  pub fn constructor_schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn constructor_schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_constructor_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // constructor_payload: optional bytes
  pub fn constructor_payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_constructor_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // builder_methods: repeated message org.apache.beam.model.pipeline.v1.BuilderMethod
  pub fn builder_methods(&self) -> ::protobuf::RepeatedView<'_, super::BuilderMethod> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::BuilderMethod>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn builder_methods_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::BuilderMethod> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_builder_methods(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::BuilderMethod>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}

// SAFETY:
// - `JavaClassLookupPayloadMut` does not perform any shared mutation.
unsafe impl Send for JavaClassLookupPayloadMut<'_> {}

// SAFETY:
// - `JavaClassLookupPayloadMut` does not perform any shared mutation.
unsafe impl Sync for JavaClassLookupPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for JavaClassLookupPayloadMut<'msg> {
  type Proxied = JavaClassLookupPayload;
  fn as_view(&self) -> ::protobuf::View<'_, JavaClassLookupPayload> {
    JavaClassLookupPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JavaClassLookupPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JavaClassLookupPayload>
  where
      'msg: 'shorter {
    JavaClassLookupPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JavaClassLookupPayloadMut<'msg> {
  type MutProxied = JavaClassLookupPayload;
  fn as_mut(&mut self) -> JavaClassLookupPayloadMut<'msg> {
    JavaClassLookupPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JavaClassLookupPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> JavaClassLookupPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JavaClassLookupPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JavaClassLookupPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JavaClassLookupPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JavaClassLookupPayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // class_name: optional string
  pub fn class_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_class_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // constructor_method: optional string
  pub fn constructor_method(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_constructor_method(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // constructor_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_constructor_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_constructor_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn constructor_schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.constructor_schema(), self.has_constructor_schema())
  }
  pub fn constructor_schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn constructor_schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_constructor_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // constructor_payload: optional bytes
  pub fn constructor_payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_constructor_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // builder_methods: repeated message org.apache.beam.model.pipeline.v1.BuilderMethod
  pub fn builder_methods(&self) -> ::protobuf::RepeatedView<'_, super::BuilderMethod> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::BuilderMethod>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn builder_methods_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::BuilderMethod> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_builder_methods(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::BuilderMethod>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}  // impl JavaClassLookupPayload

impl ::std::ops::Drop for JavaClassLookupPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JavaClassLookupPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JavaClassLookupPayload {
  type Proxied = Self;
  fn as_view(&self) -> JavaClassLookupPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JavaClassLookupPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JavaClassLookupPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JavaClassLookupPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__JavaClassLookupPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X30PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__JavaClassLookupPayload_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BuilderMethod as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__JavaClassLookupPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JavaClassLookupPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JavaClassLookupPayload {
  type Msg = JavaClassLookupPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JavaClassLookupPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JavaClassLookupPayload {
  type Msg = JavaClassLookupPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JavaClassLookupPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JavaClassLookupPayloadMut<'_> {
  type Msg = JavaClassLookupPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JavaClassLookupPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JavaClassLookupPayloadMut<'_> {
  type Msg = JavaClassLookupPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JavaClassLookupPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JavaClassLookupPayloadView<'_> {
  type Msg = JavaClassLookupPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JavaClassLookupPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JavaClassLookupPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__BuilderMethod_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BuilderMethod {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BuilderMethod>
}

impl ::protobuf::Message for BuilderMethod {}

impl ::std::default::Default for BuilderMethod {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BuilderMethod {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BuilderMethod` is `Sync` because it does not implement interior mutability.
//    Neither does `BuilderMethodMut`.
unsafe impl Sync for BuilderMethod {}

// SAFETY:
// - `BuilderMethod` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for BuilderMethod {}

impl ::protobuf::Proxied for BuilderMethod {
  type View<'msg> = BuilderMethodView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BuilderMethod {}

impl ::protobuf::MutProxied for BuilderMethod {
  type Mut<'msg> = BuilderMethodMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BuilderMethodView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BuilderMethod>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BuilderMethodView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BuilderMethodView<'msg> {
  type Message = BuilderMethod;
}

impl ::std::fmt::Debug for BuilderMethodView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BuilderMethodView<'_> {
  fn default() -> BuilderMethodView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BuilderMethod>> for BuilderMethodView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BuilderMethod>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BuilderMethodView<'msg> {

  pub fn to_owned(&self) -> BuilderMethod {
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

  // schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }

  // payload: optional bytes
  pub fn payload(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `BuilderMethodView` is `Sync` because it does not support mutation.
unsafe impl Sync for BuilderMethodView<'_> {}

// SAFETY:
// - `BuilderMethodView` is `Send` because while its alive a `BuilderMethodMut` cannot.
// - `BuilderMethodView` does not use thread-local data.
unsafe impl Send for BuilderMethodView<'_> {}

impl<'msg> ::protobuf::AsView for BuilderMethodView<'msg> {
  type Proxied = BuilderMethod;
  fn as_view(&self) -> ::protobuf::View<'msg, BuilderMethod> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BuilderMethodView<'msg> {
  fn into_view<'shorter>(self) -> BuilderMethodView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BuilderMethod> for BuilderMethodView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BuilderMethod {
    let mut dst = BuilderMethod::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BuilderMethod> for BuilderMethodMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BuilderMethod {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for BuilderMethod {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BuilderMethodView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BuilderMethodMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BuilderMethodMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BuilderMethod>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BuilderMethodMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BuilderMethodMut<'msg> {
  type Message = BuilderMethod;
}

impl ::std::fmt::Debug for BuilderMethodMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BuilderMethod>> for BuilderMethodMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BuilderMethod>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BuilderMethodMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BuilderMethod> {
    self.inner
  }

  pub fn to_owned(&self) -> BuilderMethod {
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

  // schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // payload: optional bytes
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `BuilderMethodMut` does not perform any shared mutation.
unsafe impl Send for BuilderMethodMut<'_> {}

// SAFETY:
// - `BuilderMethodMut` does not perform any shared mutation.
unsafe impl Sync for BuilderMethodMut<'_> {}

impl<'msg> ::protobuf::AsView for BuilderMethodMut<'msg> {
  type Proxied = BuilderMethod;
  fn as_view(&self) -> ::protobuf::View<'_, BuilderMethod> {
    BuilderMethodView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BuilderMethodMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BuilderMethod>
  where
      'msg: 'shorter {
    BuilderMethodView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for BuilderMethodMut<'msg> {
  type MutProxied = BuilderMethod;
  fn as_mut(&mut self) -> BuilderMethodMut<'msg> {
    BuilderMethodMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BuilderMethodMut<'msg> {
  fn into_mut<'shorter>(self) -> BuilderMethodMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BuilderMethod {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BuilderMethod> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BuilderMethodView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BuilderMethodMut<'_> {
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

  // schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.schema(), self.has_schema())
  }
  pub fn schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // payload: optional bytes
  pub fn payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl BuilderMethod

impl ::std::ops::Drop for BuilderMethod {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BuilderMethod {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BuilderMethod {
  type Proxied = Self;
  fn as_view(&self) -> BuilderMethodView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BuilderMethod {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BuilderMethodMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BuilderMethod {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__BuilderMethod_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X30P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__BuilderMethod_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__BuilderMethod_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BuilderMethod {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BuilderMethod {
  type Msg = BuilderMethod;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuilderMethod> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BuilderMethod {
  type Msg = BuilderMethod;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuilderMethod> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BuilderMethodMut<'_> {
  type Msg = BuilderMethod;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuilderMethod> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BuilderMethodMut<'_> {
  type Msg = BuilderMethod;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuilderMethod> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BuilderMethodView<'_> {
  type Msg = BuilderMethod;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuilderMethod> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BuilderMethodMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__Annotations_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Annotations {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Annotations>
}

impl ::protobuf::Message for Annotations {}

impl ::std::default::Default for Annotations {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Annotations {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Annotations` is `Sync` because it does not implement interior mutability.
//    Neither does `AnnotationsMut`.
unsafe impl Sync for Annotations {}

// SAFETY:
// - `Annotations` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Annotations {}

impl ::protobuf::Proxied for Annotations {
  type View<'msg> = AnnotationsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Annotations {}

impl ::protobuf::MutProxied for Annotations {
  type Mut<'msg> = AnnotationsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AnnotationsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Annotations>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AnnotationsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AnnotationsView<'msg> {
  type Message = Annotations;
}

impl ::std::fmt::Debug for AnnotationsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AnnotationsView<'_> {
  fn default() -> AnnotationsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Annotations>> for AnnotationsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Annotations>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AnnotationsView<'msg> {

  pub fn to_owned(&self) -> Annotations {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `AnnotationsView` is `Sync` because it does not support mutation.
unsafe impl Sync for AnnotationsView<'_> {}

// SAFETY:
// - `AnnotationsView` is `Send` because while its alive a `AnnotationsMut` cannot.
// - `AnnotationsView` does not use thread-local data.
unsafe impl Send for AnnotationsView<'_> {}

impl<'msg> ::protobuf::AsView for AnnotationsView<'msg> {
  type Proxied = Annotations;
  fn as_view(&self) -> ::protobuf::View<'msg, Annotations> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AnnotationsView<'msg> {
  fn into_view<'shorter>(self) -> AnnotationsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Annotations> for AnnotationsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Annotations {
    let mut dst = Annotations::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Annotations> for AnnotationsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Annotations {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Annotations {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AnnotationsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AnnotationsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AnnotationsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Annotations>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AnnotationsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AnnotationsMut<'msg> {
  type Message = Annotations;
}

impl ::std::fmt::Debug for AnnotationsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Annotations>> for AnnotationsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Annotations>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AnnotationsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Annotations> {
    self.inner
  }

  pub fn to_owned(&self) -> Annotations {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `AnnotationsMut` does not perform any shared mutation.
unsafe impl Send for AnnotationsMut<'_> {}

// SAFETY:
// - `AnnotationsMut` does not perform any shared mutation.
unsafe impl Sync for AnnotationsMut<'_> {}

impl<'msg> ::protobuf::AsView for AnnotationsMut<'msg> {
  type Proxied = Annotations;
  fn as_view(&self) -> ::protobuf::View<'_, Annotations> {
    AnnotationsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AnnotationsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Annotations>
  where
      'msg: 'shorter {
    AnnotationsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AnnotationsMut<'msg> {
  type MutProxied = Annotations;
  fn as_mut(&mut self) -> AnnotationsMut<'msg> {
    AnnotationsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AnnotationsMut<'msg> {
  fn into_mut<'shorter>(self) -> AnnotationsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Annotations {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Annotations> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AnnotationsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AnnotationsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Annotations

impl ::std::ops::Drop for Annotations {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Annotations {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Annotations {
  type Proxied = Self;
  fn as_view(&self) -> AnnotationsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Annotations {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AnnotationsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Annotations {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__Annotations_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__Annotations_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__Annotations_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Annotations {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Annotations {
  type Msg = Annotations;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotations> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Annotations {
  type Msg = Annotations;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotations> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AnnotationsMut<'_> {
  type Msg = Annotations;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotations> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AnnotationsMut<'_> {
  type Msg = Annotations;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotations> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AnnotationsView<'_> {
  type Msg = Annotations;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotations> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AnnotationsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod annotations {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const ConfigRowKey: Enum = Enum(0);
  pub const ConfigRowSchemaKey: Enum = Enum(1);
  pub const SchematransformUrnKey: Enum = Enum(2);
  pub const ManagedUnderlyingTransformUrnKey: Enum = Enum(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "ConfigRowKey",
      1 => "ConfigRowSchemaKey",
      2 => "SchematransformUrnKey",
      3 => "ManagedUnderlyingTransformUrnKey",
      _ => return None
    })
  }
}

impl ::std::convert::From<Enum> for i32 {
  fn from(val: Enum) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Enum {
  fn from(val: i32) -> Enum {
    Self(val)
  }
}

impl ::std::default::Default for Enum {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Enum {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Enum::{}", constant_name)
    } else {
      write!(f, "Enum::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Enum {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Enum {}

impl ::protobuf::Proxied for Enum {
  type View<'a> = Enum;
}

impl ::protobuf::AsView for Enum {
  type Proxied = Enum;

  fn as_view(&self) -> Enum {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Enum {
  fn into_view<'shorter>(self) -> Enum where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Enum {
  const NAME: &'static str = "Enum";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod annotations


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__SchemaTransformPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SchemaTransformPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SchemaTransformPayload>
}

impl ::protobuf::Message for SchemaTransformPayload {}

impl ::std::default::Default for SchemaTransformPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SchemaTransformPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SchemaTransformPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `SchemaTransformPayloadMut`.
unsafe impl Sync for SchemaTransformPayload {}

// SAFETY:
// - `SchemaTransformPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SchemaTransformPayload {}

impl ::protobuf::Proxied for SchemaTransformPayload {
  type View<'msg> = SchemaTransformPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SchemaTransformPayload {}

impl ::protobuf::MutProxied for SchemaTransformPayload {
  type Mut<'msg> = SchemaTransformPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SchemaTransformPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaTransformPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaTransformPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SchemaTransformPayloadView<'msg> {
  type Message = SchemaTransformPayload;
}

impl ::std::fmt::Debug for SchemaTransformPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SchemaTransformPayloadView<'_> {
  fn default() -> SchemaTransformPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaTransformPayload>> for SchemaTransformPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaTransformPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaTransformPayloadView<'msg> {

  pub fn to_owned(&self) -> SchemaTransformPayload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // identifier: optional string
  pub fn identifier(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // configuration_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_configuration_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn configuration_schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.configuration_schema(), self.has_configuration_schema())
  }
  pub fn configuration_schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }

  // configuration_row: optional bytes
  pub fn configuration_row(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `SchemaTransformPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for SchemaTransformPayloadView<'_> {}

// SAFETY:
// - `SchemaTransformPayloadView` is `Send` because while its alive a `SchemaTransformPayloadMut` cannot.
// - `SchemaTransformPayloadView` does not use thread-local data.
unsafe impl Send for SchemaTransformPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for SchemaTransformPayloadView<'msg> {
  type Proxied = SchemaTransformPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, SchemaTransformPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaTransformPayloadView<'msg> {
  fn into_view<'shorter>(self) -> SchemaTransformPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaTransformPayload> for SchemaTransformPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaTransformPayload {
    let mut dst = SchemaTransformPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaTransformPayload> for SchemaTransformPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaTransformPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SchemaTransformPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaTransformPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaTransformPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SchemaTransformPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaTransformPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SchemaTransformPayloadMut<'msg> {
  type Message = SchemaTransformPayload;
}

impl ::std::fmt::Debug for SchemaTransformPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformPayload>> for SchemaTransformPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaTransformPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> SchemaTransformPayload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // identifier: optional string
  pub fn identifier(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_identifier(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // configuration_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_configuration_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_configuration_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn configuration_schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.configuration_schema(), self.has_configuration_schema())
  }
  pub fn configuration_schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn configuration_schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_configuration_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // configuration_row: optional bytes
  pub fn configuration_row(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_configuration_row(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `SchemaTransformPayloadMut` does not perform any shared mutation.
unsafe impl Send for SchemaTransformPayloadMut<'_> {}

// SAFETY:
// - `SchemaTransformPayloadMut` does not perform any shared mutation.
unsafe impl Sync for SchemaTransformPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for SchemaTransformPayloadMut<'msg> {
  type Proxied = SchemaTransformPayload;
  fn as_view(&self) -> ::protobuf::View<'_, SchemaTransformPayload> {
    SchemaTransformPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaTransformPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SchemaTransformPayload>
  where
      'msg: 'shorter {
    SchemaTransformPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SchemaTransformPayloadMut<'msg> {
  type MutProxied = SchemaTransformPayload;
  fn as_mut(&mut self) -> SchemaTransformPayloadMut<'msg> {
    SchemaTransformPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SchemaTransformPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> SchemaTransformPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SchemaTransformPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SchemaTransformPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SchemaTransformPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SchemaTransformPayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // identifier: optional string
  pub fn identifier(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_identifier(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // configuration_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_configuration_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_configuration_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn configuration_schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.configuration_schema(), self.has_configuration_schema())
  }
  pub fn configuration_schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn configuration_schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_configuration_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // configuration_row: optional bytes
  pub fn configuration_row(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_configuration_row(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl SchemaTransformPayload

impl ::std::ops::Drop for SchemaTransformPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SchemaTransformPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SchemaTransformPayload {
  type Proxied = Self;
  fn as_view(&self) -> SchemaTransformPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SchemaTransformPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SchemaTransformPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SchemaTransformPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__SchemaTransformPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X30P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__SchemaTransformPayload_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__SchemaTransformPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaTransformPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaTransformPayload {
  type Msg = SchemaTransformPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaTransformPayload {
  type Msg = SchemaTransformPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaTransformPayloadMut<'_> {
  type Msg = SchemaTransformPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaTransformPayloadMut<'_> {
  type Msg = SchemaTransformPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaTransformPayloadView<'_> {
  type Msg = SchemaTransformPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaTransformPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



