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
pub(crate) static mut org__apache__beam__model__pipeline__v1__GlobalWindowsPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GlobalWindowsPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GlobalWindowsPayload>
}

impl ::protobuf::Message for GlobalWindowsPayload {}

impl ::std::default::Default for GlobalWindowsPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GlobalWindowsPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GlobalWindowsPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `GlobalWindowsPayloadMut`.
unsafe impl Sync for GlobalWindowsPayload {}

// SAFETY:
// - `GlobalWindowsPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GlobalWindowsPayload {}

impl ::protobuf::Proxied for GlobalWindowsPayload {
  type View<'msg> = GlobalWindowsPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GlobalWindowsPayload {}

impl ::protobuf::MutProxied for GlobalWindowsPayload {
  type Mut<'msg> = GlobalWindowsPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GlobalWindowsPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GlobalWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GlobalWindowsPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GlobalWindowsPayloadView<'msg> {
  type Message = GlobalWindowsPayload;
}

impl ::std::fmt::Debug for GlobalWindowsPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GlobalWindowsPayloadView<'_> {
  fn default() -> GlobalWindowsPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GlobalWindowsPayload>> for GlobalWindowsPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GlobalWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GlobalWindowsPayloadView<'msg> {

  pub fn to_owned(&self) -> GlobalWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GlobalWindowsPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for GlobalWindowsPayloadView<'_> {}

// SAFETY:
// - `GlobalWindowsPayloadView` is `Send` because while its alive a `GlobalWindowsPayloadMut` cannot.
// - `GlobalWindowsPayloadView` does not use thread-local data.
unsafe impl Send for GlobalWindowsPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for GlobalWindowsPayloadView<'msg> {
  type Proxied = GlobalWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, GlobalWindowsPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GlobalWindowsPayloadView<'msg> {
  fn into_view<'shorter>(self) -> GlobalWindowsPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GlobalWindowsPayload> for GlobalWindowsPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GlobalWindowsPayload {
    let mut dst = GlobalWindowsPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GlobalWindowsPayload> for GlobalWindowsPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GlobalWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GlobalWindowsPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GlobalWindowsPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GlobalWindowsPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GlobalWindowsPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GlobalWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GlobalWindowsPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GlobalWindowsPayloadMut<'msg> {
  type Message = GlobalWindowsPayload;
}

impl ::std::fmt::Debug for GlobalWindowsPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GlobalWindowsPayload>> for GlobalWindowsPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GlobalWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GlobalWindowsPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GlobalWindowsPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> GlobalWindowsPayload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `GlobalWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Send for GlobalWindowsPayloadMut<'_> {}

// SAFETY:
// - `GlobalWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Sync for GlobalWindowsPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for GlobalWindowsPayloadMut<'msg> {
  type Proxied = GlobalWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'_, GlobalWindowsPayload> {
    GlobalWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GlobalWindowsPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GlobalWindowsPayload>
  where
      'msg: 'shorter {
    GlobalWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GlobalWindowsPayloadMut<'msg> {
  type MutProxied = GlobalWindowsPayload;
  fn as_mut(&mut self) -> GlobalWindowsPayloadMut<'msg> {
    GlobalWindowsPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GlobalWindowsPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> GlobalWindowsPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GlobalWindowsPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GlobalWindowsPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GlobalWindowsPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GlobalWindowsPayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl GlobalWindowsPayload

impl ::std::ops::Drop for GlobalWindowsPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GlobalWindowsPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GlobalWindowsPayload {
  type Proxied = Self;
  fn as_view(&self) -> GlobalWindowsPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GlobalWindowsPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GlobalWindowsPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GlobalWindowsPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__GlobalWindowsPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__GlobalWindowsPayload_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__GlobalWindowsPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GlobalWindowsPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GlobalWindowsPayload {
  type Msg = GlobalWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GlobalWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GlobalWindowsPayload {
  type Msg = GlobalWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GlobalWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GlobalWindowsPayloadMut<'_> {
  type Msg = GlobalWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GlobalWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GlobalWindowsPayloadMut<'_> {
  type Msg = GlobalWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GlobalWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GlobalWindowsPayloadView<'_> {
  type Msg = GlobalWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GlobalWindowsPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GlobalWindowsPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod global_windows_payload {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const Properties: Enum = Enum(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Properties",
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
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod global_windows_payload


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__FixedWindowsPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FixedWindowsPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FixedWindowsPayload>
}

impl ::protobuf::Message for FixedWindowsPayload {}

impl ::std::default::Default for FixedWindowsPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FixedWindowsPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FixedWindowsPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `FixedWindowsPayloadMut`.
unsafe impl Sync for FixedWindowsPayload {}

// SAFETY:
// - `FixedWindowsPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for FixedWindowsPayload {}

impl ::protobuf::Proxied for FixedWindowsPayload {
  type View<'msg> = FixedWindowsPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FixedWindowsPayload {}

impl ::protobuf::MutProxied for FixedWindowsPayload {
  type Mut<'msg> = FixedWindowsPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FixedWindowsPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FixedWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FixedWindowsPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FixedWindowsPayloadView<'msg> {
  type Message = FixedWindowsPayload;
}

impl ::std::fmt::Debug for FixedWindowsPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FixedWindowsPayloadView<'_> {
  fn default() -> FixedWindowsPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FixedWindowsPayload>> for FixedWindowsPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FixedWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FixedWindowsPayloadView<'msg> {

  pub fn to_owned(&self) -> FixedWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // size: optional message google.protobuf.Duration
  pub fn has_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn size_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.size(), self.has_size())
  }
  pub fn size(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // offset: optional message google.protobuf.Timestamp
  pub fn has_offset(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn offset_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.offset(), self.has_offset())
  }
  pub fn offset(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `FixedWindowsPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for FixedWindowsPayloadView<'_> {}

// SAFETY:
// - `FixedWindowsPayloadView` is `Send` because while its alive a `FixedWindowsPayloadMut` cannot.
// - `FixedWindowsPayloadView` does not use thread-local data.
unsafe impl Send for FixedWindowsPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for FixedWindowsPayloadView<'msg> {
  type Proxied = FixedWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, FixedWindowsPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FixedWindowsPayloadView<'msg> {
  fn into_view<'shorter>(self) -> FixedWindowsPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FixedWindowsPayload> for FixedWindowsPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FixedWindowsPayload {
    let mut dst = FixedWindowsPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FixedWindowsPayload> for FixedWindowsPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FixedWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for FixedWindowsPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FixedWindowsPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FixedWindowsPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FixedWindowsPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FixedWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FixedWindowsPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FixedWindowsPayloadMut<'msg> {
  type Message = FixedWindowsPayload;
}

impl ::std::fmt::Debug for FixedWindowsPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FixedWindowsPayload>> for FixedWindowsPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FixedWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FixedWindowsPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FixedWindowsPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> FixedWindowsPayload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // size: optional message google.protobuf.Duration
  pub fn has_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn size_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.size(), self.has_size())
  }
  pub fn size(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn size_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // offset: optional message google.protobuf.Timestamp
  pub fn has_offset(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_offset(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn offset_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.offset(), self.has_offset())
  }
  pub fn offset(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn offset_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_offset(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `FixedWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Send for FixedWindowsPayloadMut<'_> {}

// SAFETY:
// - `FixedWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Sync for FixedWindowsPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for FixedWindowsPayloadMut<'msg> {
  type Proxied = FixedWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'_, FixedWindowsPayload> {
    FixedWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FixedWindowsPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FixedWindowsPayload>
  where
      'msg: 'shorter {
    FixedWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for FixedWindowsPayloadMut<'msg> {
  type MutProxied = FixedWindowsPayload;
  fn as_mut(&mut self) -> FixedWindowsPayloadMut<'msg> {
    FixedWindowsPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FixedWindowsPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> FixedWindowsPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FixedWindowsPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FixedWindowsPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FixedWindowsPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FixedWindowsPayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // size: optional message google.protobuf.Duration
  pub fn has_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn size_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.size(), self.has_size())
  }
  pub fn size(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn size_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // offset: optional message google.protobuf.Timestamp
  pub fn has_offset(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_offset(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn offset_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.offset(), self.has_offset())
  }
  pub fn offset(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn offset_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_offset(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl FixedWindowsPayload

impl ::std::ops::Drop for FixedWindowsPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FixedWindowsPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FixedWindowsPayload {
  type Proxied = Self;
  fn as_view(&self) -> FixedWindowsPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FixedWindowsPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FixedWindowsPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FixedWindowsPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__FixedWindowsPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__FixedWindowsPayload_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__FixedWindowsPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FixedWindowsPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FixedWindowsPayload {
  type Msg = FixedWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FixedWindowsPayload {
  type Msg = FixedWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FixedWindowsPayloadMut<'_> {
  type Msg = FixedWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FixedWindowsPayloadMut<'_> {
  type Msg = FixedWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FixedWindowsPayloadView<'_> {
  type Msg = FixedWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FixedWindowsPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FixedWindowsPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod fixed_windows_payload {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const Properties: Enum = Enum(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Properties",
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
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod fixed_windows_payload


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__SlidingWindowsPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SlidingWindowsPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SlidingWindowsPayload>
}

impl ::protobuf::Message for SlidingWindowsPayload {}

impl ::std::default::Default for SlidingWindowsPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SlidingWindowsPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SlidingWindowsPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `SlidingWindowsPayloadMut`.
unsafe impl Sync for SlidingWindowsPayload {}

// SAFETY:
// - `SlidingWindowsPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SlidingWindowsPayload {}

impl ::protobuf::Proxied for SlidingWindowsPayload {
  type View<'msg> = SlidingWindowsPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SlidingWindowsPayload {}

impl ::protobuf::MutProxied for SlidingWindowsPayload {
  type Mut<'msg> = SlidingWindowsPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SlidingWindowsPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SlidingWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SlidingWindowsPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SlidingWindowsPayloadView<'msg> {
  type Message = SlidingWindowsPayload;
}

impl ::std::fmt::Debug for SlidingWindowsPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SlidingWindowsPayloadView<'_> {
  fn default() -> SlidingWindowsPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SlidingWindowsPayload>> for SlidingWindowsPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SlidingWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SlidingWindowsPayloadView<'msg> {

  pub fn to_owned(&self) -> SlidingWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // size: optional message google.protobuf.Duration
  pub fn has_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn size_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.size(), self.has_size())
  }
  pub fn size(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // offset: optional message google.protobuf.Timestamp
  pub fn has_offset(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn offset_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.offset(), self.has_offset())
  }
  pub fn offset(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // period: optional message google.protobuf.Duration
  pub fn has_period(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn period_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.period(), self.has_period())
  }
  pub fn period(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `SlidingWindowsPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for SlidingWindowsPayloadView<'_> {}

// SAFETY:
// - `SlidingWindowsPayloadView` is `Send` because while its alive a `SlidingWindowsPayloadMut` cannot.
// - `SlidingWindowsPayloadView` does not use thread-local data.
unsafe impl Send for SlidingWindowsPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for SlidingWindowsPayloadView<'msg> {
  type Proxied = SlidingWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, SlidingWindowsPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SlidingWindowsPayloadView<'msg> {
  fn into_view<'shorter>(self) -> SlidingWindowsPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SlidingWindowsPayload> for SlidingWindowsPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SlidingWindowsPayload {
    let mut dst = SlidingWindowsPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SlidingWindowsPayload> for SlidingWindowsPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SlidingWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SlidingWindowsPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SlidingWindowsPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SlidingWindowsPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SlidingWindowsPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SlidingWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SlidingWindowsPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SlidingWindowsPayloadMut<'msg> {
  type Message = SlidingWindowsPayload;
}

impl ::std::fmt::Debug for SlidingWindowsPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SlidingWindowsPayload>> for SlidingWindowsPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SlidingWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SlidingWindowsPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SlidingWindowsPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> SlidingWindowsPayload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // size: optional message google.protobuf.Duration
  pub fn has_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn size_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.size(), self.has_size())
  }
  pub fn size(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn size_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // offset: optional message google.protobuf.Timestamp
  pub fn has_offset(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_offset(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn offset_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.offset(), self.has_offset())
  }
  pub fn offset(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn offset_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_offset(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // period: optional message google.protobuf.Duration
  pub fn has_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn period_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.period(), self.has_period())
  }
  pub fn period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}

// SAFETY:
// - `SlidingWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Send for SlidingWindowsPayloadMut<'_> {}

// SAFETY:
// - `SlidingWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Sync for SlidingWindowsPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for SlidingWindowsPayloadMut<'msg> {
  type Proxied = SlidingWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'_, SlidingWindowsPayload> {
    SlidingWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SlidingWindowsPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SlidingWindowsPayload>
  where
      'msg: 'shorter {
    SlidingWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SlidingWindowsPayloadMut<'msg> {
  type MutProxied = SlidingWindowsPayload;
  fn as_mut(&mut self) -> SlidingWindowsPayloadMut<'msg> {
    SlidingWindowsPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SlidingWindowsPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> SlidingWindowsPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SlidingWindowsPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SlidingWindowsPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SlidingWindowsPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SlidingWindowsPayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // size: optional message google.protobuf.Duration
  pub fn has_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn size_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.size(), self.has_size())
  }
  pub fn size(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn size_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // offset: optional message google.protobuf.Timestamp
  pub fn has_offset(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_offset(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn offset_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.offset(), self.has_offset())
  }
  pub fn offset(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn offset_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_offset(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // period: optional message google.protobuf.Duration
  pub fn has_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn period_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.period(), self.has_period())
  }
  pub fn period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl SlidingWindowsPayload

impl ::std::ops::Drop for SlidingWindowsPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SlidingWindowsPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SlidingWindowsPayload {
  type Proxied = Self;
  fn as_view(&self) -> SlidingWindowsPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SlidingWindowsPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SlidingWindowsPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SlidingWindowsPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__SlidingWindowsPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__SlidingWindowsPayload_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__SlidingWindowsPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SlidingWindowsPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SlidingWindowsPayload {
  type Msg = SlidingWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlidingWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlidingWindowsPayload {
  type Msg = SlidingWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlidingWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SlidingWindowsPayloadMut<'_> {
  type Msg = SlidingWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlidingWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlidingWindowsPayloadMut<'_> {
  type Msg = SlidingWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlidingWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlidingWindowsPayloadView<'_> {
  type Msg = SlidingWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlidingWindowsPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SlidingWindowsPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod sliding_windows_payload {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const Properties: Enum = Enum(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Properties",
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
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod sliding_windows_payload


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__SessionWindowsPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SessionWindowsPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SessionWindowsPayload>
}

impl ::protobuf::Message for SessionWindowsPayload {}

impl ::std::default::Default for SessionWindowsPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SessionWindowsPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SessionWindowsPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `SessionWindowsPayloadMut`.
unsafe impl Sync for SessionWindowsPayload {}

// SAFETY:
// - `SessionWindowsPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SessionWindowsPayload {}

impl ::protobuf::Proxied for SessionWindowsPayload {
  type View<'msg> = SessionWindowsPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SessionWindowsPayload {}

impl ::protobuf::MutProxied for SessionWindowsPayload {
  type Mut<'msg> = SessionWindowsPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SessionWindowsPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SessionWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SessionWindowsPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SessionWindowsPayloadView<'msg> {
  type Message = SessionWindowsPayload;
}

impl ::std::fmt::Debug for SessionWindowsPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SessionWindowsPayloadView<'_> {
  fn default() -> SessionWindowsPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SessionWindowsPayload>> for SessionWindowsPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SessionWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SessionWindowsPayloadView<'msg> {

  pub fn to_owned(&self) -> SessionWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // gap_size: optional message google.protobuf.Duration
  pub fn has_gap_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn gap_size_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.gap_size(), self.has_gap_size())
  }
  pub fn gap_size(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `SessionWindowsPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for SessionWindowsPayloadView<'_> {}

// SAFETY:
// - `SessionWindowsPayloadView` is `Send` because while its alive a `SessionWindowsPayloadMut` cannot.
// - `SessionWindowsPayloadView` does not use thread-local data.
unsafe impl Send for SessionWindowsPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for SessionWindowsPayloadView<'msg> {
  type Proxied = SessionWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, SessionWindowsPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SessionWindowsPayloadView<'msg> {
  fn into_view<'shorter>(self) -> SessionWindowsPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SessionWindowsPayload> for SessionWindowsPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SessionWindowsPayload {
    let mut dst = SessionWindowsPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SessionWindowsPayload> for SessionWindowsPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SessionWindowsPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SessionWindowsPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SessionWindowsPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SessionWindowsPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SessionWindowsPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SessionWindowsPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SessionWindowsPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SessionWindowsPayloadMut<'msg> {
  type Message = SessionWindowsPayload;
}

impl ::std::fmt::Debug for SessionWindowsPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SessionWindowsPayload>> for SessionWindowsPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SessionWindowsPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SessionWindowsPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SessionWindowsPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> SessionWindowsPayload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // gap_size: optional message google.protobuf.Duration
  pub fn has_gap_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_gap_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn gap_size_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.gap_size(), self.has_gap_size())
  }
  pub fn gap_size(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn gap_size_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_gap_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `SessionWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Send for SessionWindowsPayloadMut<'_> {}

// SAFETY:
// - `SessionWindowsPayloadMut` does not perform any shared mutation.
unsafe impl Sync for SessionWindowsPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for SessionWindowsPayloadMut<'msg> {
  type Proxied = SessionWindowsPayload;
  fn as_view(&self) -> ::protobuf::View<'_, SessionWindowsPayload> {
    SessionWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SessionWindowsPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SessionWindowsPayload>
  where
      'msg: 'shorter {
    SessionWindowsPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SessionWindowsPayloadMut<'msg> {
  type MutProxied = SessionWindowsPayload;
  fn as_mut(&mut self) -> SessionWindowsPayloadMut<'msg> {
    SessionWindowsPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SessionWindowsPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> SessionWindowsPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SessionWindowsPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SessionWindowsPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SessionWindowsPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SessionWindowsPayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // gap_size: optional message google.protobuf.Duration
  pub fn has_gap_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_gap_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn gap_size_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.gap_size(), self.has_gap_size())
  }
  pub fn gap_size(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn gap_size_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_gap_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl SessionWindowsPayload

impl ::std::ops::Drop for SessionWindowsPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SessionWindowsPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SessionWindowsPayload {
  type Proxied = Self;
  fn as_view(&self) -> SessionWindowsPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SessionWindowsPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SessionWindowsPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SessionWindowsPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__SessionWindowsPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__SessionWindowsPayload_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__SessionWindowsPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SessionWindowsPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SessionWindowsPayload {
  type Msg = SessionWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SessionWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SessionWindowsPayload {
  type Msg = SessionWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SessionWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SessionWindowsPayloadMut<'_> {
  type Msg = SessionWindowsPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SessionWindowsPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SessionWindowsPayloadMut<'_> {
  type Msg = SessionWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SessionWindowsPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SessionWindowsPayloadView<'_> {
  type Msg = SessionWindowsPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SessionWindowsPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SessionWindowsPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod session_windows_payload {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const Properties: Enum = Enum(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Properties",
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
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod session_windows_payload


