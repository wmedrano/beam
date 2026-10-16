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
pub(crate) static mut org__apache__beam__model__pipeline__v1__MonitoringInfoSpec_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MonitoringInfoSpec {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MonitoringInfoSpec>
}

impl ::protobuf::Message for MonitoringInfoSpec {}

impl ::std::default::Default for MonitoringInfoSpec {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MonitoringInfoSpec {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MonitoringInfoSpec` is `Sync` because it does not implement interior mutability.
//    Neither does `MonitoringInfoSpecMut`.
unsafe impl Sync for MonitoringInfoSpec {}

// SAFETY:
// - `MonitoringInfoSpec` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MonitoringInfoSpec {}

impl ::protobuf::Proxied for MonitoringInfoSpec {
  type View<'msg> = MonitoringInfoSpecView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MonitoringInfoSpec {}

impl ::protobuf::MutProxied for MonitoringInfoSpec {
  type Mut<'msg> = MonitoringInfoSpecMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MonitoringInfoSpecView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoSpec>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoSpecView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MonitoringInfoSpecView<'msg> {
  type Message = MonitoringInfoSpec;
}

impl ::std::fmt::Debug for MonitoringInfoSpecView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MonitoringInfoSpecView<'_> {
  fn default() -> MonitoringInfoSpecView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoSpec>> for MonitoringInfoSpecView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoSpec>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoSpecView<'msg> {

  pub fn to_owned(&self) -> MonitoringInfoSpec {
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

  // type: optional string
  pub fn r#type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // required_labels: repeated string
  pub fn required_labels(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // annotations: repeated message org.apache.beam.model.pipeline.v1.Annotation
  pub fn annotations(self) -> ::protobuf::RepeatedView<'msg, super::Annotation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Annotation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MonitoringInfoSpecView` is `Sync` because it does not support mutation.
unsafe impl Sync for MonitoringInfoSpecView<'_> {}

// SAFETY:
// - `MonitoringInfoSpecView` is `Send` because while its alive a `MonitoringInfoSpecMut` cannot.
// - `MonitoringInfoSpecView` does not use thread-local data.
unsafe impl Send for MonitoringInfoSpecView<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoSpecView<'msg> {
  type Proxied = MonitoringInfoSpec;
  fn as_view(&self) -> ::protobuf::View<'msg, MonitoringInfoSpec> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoSpecView<'msg> {
  fn into_view<'shorter>(self) -> MonitoringInfoSpecView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoSpec> for MonitoringInfoSpecView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoSpec {
    let mut dst = MonitoringInfoSpec::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoSpec> for MonitoringInfoSpecMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoSpec {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MonitoringInfoSpec {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoSpecView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoSpecMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MonitoringInfoSpecMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpec>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoSpecMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MonitoringInfoSpecMut<'msg> {
  type Message = MonitoringInfoSpec;
}

impl ::std::fmt::Debug for MonitoringInfoSpecMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpec>> for MonitoringInfoSpecMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpec>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoSpecMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpec> {
    self.inner
  }

  pub fn to_owned(&self) -> MonitoringInfoSpec {
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

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // required_labels: repeated string
  pub fn required_labels(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn required_labels_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_required_labels(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // annotations: repeated message org.apache.beam.model.pipeline.v1.Annotation
  pub fn annotations(&self) -> ::protobuf::RepeatedView<'_, super::Annotation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Annotation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn annotations_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Annotation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_annotations(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Annotation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `MonitoringInfoSpecMut` does not perform any shared mutation.
unsafe impl Send for MonitoringInfoSpecMut<'_> {}

// SAFETY:
// - `MonitoringInfoSpecMut` does not perform any shared mutation.
unsafe impl Sync for MonitoringInfoSpecMut<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoSpecMut<'msg> {
  type Proxied = MonitoringInfoSpec;
  fn as_view(&self) -> ::protobuf::View<'_, MonitoringInfoSpec> {
    MonitoringInfoSpecView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoSpecMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MonitoringInfoSpec>
  where
      'msg: 'shorter {
    MonitoringInfoSpecView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MonitoringInfoSpecMut<'msg> {
  type MutProxied = MonitoringInfoSpec;
  fn as_mut(&mut self) -> MonitoringInfoSpecMut<'msg> {
    MonitoringInfoSpecMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MonitoringInfoSpecMut<'msg> {
  fn into_mut<'shorter>(self) -> MonitoringInfoSpecMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MonitoringInfoSpec {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MonitoringInfoSpec> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MonitoringInfoSpecView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MonitoringInfoSpecMut<'_> {
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

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // required_labels: repeated string
  pub fn required_labels(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn required_labels_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_required_labels(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // annotations: repeated message org.apache.beam.model.pipeline.v1.Annotation
  pub fn annotations(&self) -> ::protobuf::RepeatedView<'_, super::Annotation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Annotation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn annotations_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Annotation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_annotations(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Annotation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl MonitoringInfoSpec

impl ::std::ops::Drop for MonitoringInfoSpec {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MonitoringInfoSpec {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MonitoringInfoSpec {
  type Proxied = Self;
  fn as_view(&self) -> MonitoringInfoSpecView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MonitoringInfoSpec {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MonitoringInfoSpecMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MonitoringInfoSpec {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__MonitoringInfoSpec_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1XETG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MonitoringInfoSpec_msg_init.0, &[<super::Annotation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MonitoringInfoSpec_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoSpec {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoSpec {
  type Msg = MonitoringInfoSpec;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpec> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoSpec {
  type Msg = MonitoringInfoSpec;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpec> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoSpecMut<'_> {
  type Msg = MonitoringInfoSpec;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpec> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoSpecMut<'_> {
  type Msg = MonitoringInfoSpec;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpec> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoSpecView<'_> {
  type Msg = MonitoringInfoSpec;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpec> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoSpecMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__Annotation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Annotation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Annotation>
}

impl ::protobuf::Message for Annotation {}

impl ::std::default::Default for Annotation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Annotation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Annotation` is `Sync` because it does not implement interior mutability.
//    Neither does `AnnotationMut`.
unsafe impl Sync for Annotation {}

// SAFETY:
// - `Annotation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Annotation {}

impl ::protobuf::Proxied for Annotation {
  type View<'msg> = AnnotationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Annotation {}

impl ::protobuf::MutProxied for Annotation {
  type Mut<'msg> = AnnotationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AnnotationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Annotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AnnotationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AnnotationView<'msg> {
  type Message = Annotation;
}

impl ::std::fmt::Debug for AnnotationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AnnotationView<'_> {
  fn default() -> AnnotationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Annotation>> for AnnotationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Annotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AnnotationView<'msg> {

  pub fn to_owned(&self) -> Annotation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // value: optional string
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `AnnotationView` is `Sync` because it does not support mutation.
unsafe impl Sync for AnnotationView<'_> {}

// SAFETY:
// - `AnnotationView` is `Send` because while its alive a `AnnotationMut` cannot.
// - `AnnotationView` does not use thread-local data.
unsafe impl Send for AnnotationView<'_> {}

impl<'msg> ::protobuf::AsView for AnnotationView<'msg> {
  type Proxied = Annotation;
  fn as_view(&self) -> ::protobuf::View<'msg, Annotation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AnnotationView<'msg> {
  fn into_view<'shorter>(self) -> AnnotationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Annotation> for AnnotationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Annotation {
    let mut dst = Annotation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Annotation> for AnnotationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Annotation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Annotation {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AnnotationView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AnnotationMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AnnotationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Annotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AnnotationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AnnotationMut<'msg> {
  type Message = Annotation;
}

impl ::std::fmt::Debug for AnnotationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Annotation>> for AnnotationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Annotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AnnotationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Annotation> {
    self.inner
  }

  pub fn to_owned(&self) -> Annotation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `AnnotationMut` does not perform any shared mutation.
unsafe impl Send for AnnotationMut<'_> {}

// SAFETY:
// - `AnnotationMut` does not perform any shared mutation.
unsafe impl Sync for AnnotationMut<'_> {}

impl<'msg> ::protobuf::AsView for AnnotationMut<'msg> {
  type Proxied = Annotation;
  fn as_view(&self) -> ::protobuf::View<'_, Annotation> {
    AnnotationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AnnotationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Annotation>
  where
      'msg: 'shorter {
    AnnotationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AnnotationMut<'msg> {
  type MutProxied = Annotation;
  fn as_mut(&mut self) -> AnnotationMut<'msg> {
    AnnotationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AnnotationMut<'msg> {
  fn into_mut<'shorter>(self) -> AnnotationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Annotation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Annotation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AnnotationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AnnotationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Annotation

impl ::std::ops::Drop for Annotation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Annotation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Annotation {
  type Proxied = Self;
  fn as_view(&self) -> AnnotationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Annotation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AnnotationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Annotation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__Annotation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__Annotation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__Annotation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Annotation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Annotation {
  type Msg = Annotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Annotation {
  type Msg = Annotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AnnotationMut<'_> {
  type Msg = Annotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AnnotationMut<'_> {
  type Msg = Annotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AnnotationView<'_> {
  type Msg = Annotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Annotation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AnnotationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MonitoringInfoSpecs_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MonitoringInfoSpecs {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MonitoringInfoSpecs>
}

impl ::protobuf::Message for MonitoringInfoSpecs {}

impl ::std::default::Default for MonitoringInfoSpecs {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MonitoringInfoSpecs {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MonitoringInfoSpecs` is `Sync` because it does not implement interior mutability.
//    Neither does `MonitoringInfoSpecsMut`.
unsafe impl Sync for MonitoringInfoSpecs {}

// SAFETY:
// - `MonitoringInfoSpecs` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MonitoringInfoSpecs {}

impl ::protobuf::Proxied for MonitoringInfoSpecs {
  type View<'msg> = MonitoringInfoSpecsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MonitoringInfoSpecs {}

impl ::protobuf::MutProxied for MonitoringInfoSpecs {
  type Mut<'msg> = MonitoringInfoSpecsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MonitoringInfoSpecsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoSpecs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoSpecsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MonitoringInfoSpecsView<'msg> {
  type Message = MonitoringInfoSpecs;
}

impl ::std::fmt::Debug for MonitoringInfoSpecsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MonitoringInfoSpecsView<'_> {
  fn default() -> MonitoringInfoSpecsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoSpecs>> for MonitoringInfoSpecsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoSpecs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoSpecsView<'msg> {

  pub fn to_owned(&self) -> MonitoringInfoSpecs {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `MonitoringInfoSpecsView` is `Sync` because it does not support mutation.
unsafe impl Sync for MonitoringInfoSpecsView<'_> {}

// SAFETY:
// - `MonitoringInfoSpecsView` is `Send` because while its alive a `MonitoringInfoSpecsMut` cannot.
// - `MonitoringInfoSpecsView` does not use thread-local data.
unsafe impl Send for MonitoringInfoSpecsView<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoSpecsView<'msg> {
  type Proxied = MonitoringInfoSpecs;
  fn as_view(&self) -> ::protobuf::View<'msg, MonitoringInfoSpecs> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoSpecsView<'msg> {
  fn into_view<'shorter>(self) -> MonitoringInfoSpecsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoSpecs> for MonitoringInfoSpecsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoSpecs {
    let mut dst = MonitoringInfoSpecs::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoSpecs> for MonitoringInfoSpecsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoSpecs {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MonitoringInfoSpecs {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoSpecsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoSpecsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MonitoringInfoSpecsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpecs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoSpecsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MonitoringInfoSpecsMut<'msg> {
  type Message = MonitoringInfoSpecs;
}

impl ::std::fmt::Debug for MonitoringInfoSpecsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpecs>> for MonitoringInfoSpecsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpecs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoSpecsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoSpecs> {
    self.inner
  }

  pub fn to_owned(&self) -> MonitoringInfoSpecs {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `MonitoringInfoSpecsMut` does not perform any shared mutation.
unsafe impl Send for MonitoringInfoSpecsMut<'_> {}

// SAFETY:
// - `MonitoringInfoSpecsMut` does not perform any shared mutation.
unsafe impl Sync for MonitoringInfoSpecsMut<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoSpecsMut<'msg> {
  type Proxied = MonitoringInfoSpecs;
  fn as_view(&self) -> ::protobuf::View<'_, MonitoringInfoSpecs> {
    MonitoringInfoSpecsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoSpecsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MonitoringInfoSpecs>
  where
      'msg: 'shorter {
    MonitoringInfoSpecsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MonitoringInfoSpecsMut<'msg> {
  type MutProxied = MonitoringInfoSpecs;
  fn as_mut(&mut self) -> MonitoringInfoSpecsMut<'msg> {
    MonitoringInfoSpecsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MonitoringInfoSpecsMut<'msg> {
  fn into_mut<'shorter>(self) -> MonitoringInfoSpecsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MonitoringInfoSpecs {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MonitoringInfoSpecs> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MonitoringInfoSpecsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MonitoringInfoSpecsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl MonitoringInfoSpecs

impl ::std::ops::Drop for MonitoringInfoSpecs {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MonitoringInfoSpecs {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MonitoringInfoSpecs {
  type Proxied = Self;
  fn as_view(&self) -> MonitoringInfoSpecsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MonitoringInfoSpecs {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MonitoringInfoSpecsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MonitoringInfoSpecs {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__MonitoringInfoSpecs_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MonitoringInfoSpecs_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MonitoringInfoSpecs_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoSpecs {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoSpecs {
  type Msg = MonitoringInfoSpecs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpecs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoSpecs {
  type Msg = MonitoringInfoSpecs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpecs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoSpecsMut<'_> {
  type Msg = MonitoringInfoSpecs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpecs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoSpecsMut<'_> {
  type Msg = MonitoringInfoSpecs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpecs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoSpecsView<'_> {
  type Msg = MonitoringInfoSpecs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoSpecs> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoSpecsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod monitoring_info_specs {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const UserSumInt64: Enum = Enum(0);
  pub const UserSumDouble: Enum = Enum(1);
  pub const UserDistributionInt64: Enum = Enum(2);
  pub const UserDistributionDouble: Enum = Enum(3);
  pub const UserLatestInt64: Enum = Enum(4);
  pub const UserLatestDouble: Enum = Enum(5);
  pub const UserTopNInt64: Enum = Enum(6);
  pub const UserTopNDouble: Enum = Enum(7);
  pub const UserBottomNInt64: Enum = Enum(8);
  pub const UserBottomNDouble: Enum = Enum(9);
  pub const UserSetString: Enum = Enum(21);
  pub const UserBoundedTrie: Enum = Enum(22);
  pub const ElementCount: Enum = Enum(10);
  pub const SampledByteSize: Enum = Enum(11);
  pub const StartBundleMsecs: Enum = Enum(12);
  pub const ProcessBundleMsecs: Enum = Enum(13);
  pub const FinishBundleMsecs: Enum = Enum(14);
  pub const TotalMsecs: Enum = Enum(15);
  pub const WorkRemaining: Enum = Enum(16);
  pub const WorkCompleted: Enum = Enum(17);
  pub const DataChannelReadIndex: Enum = Enum(18);
  pub const ApiRequestCount: Enum = Enum(19);
  pub const ApiRequestLatencies: Enum = Enum(20);
  pub const UserHistogram: Enum = Enum(23);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "UserSumInt64",
      1 => "UserSumDouble",
      2 => "UserDistributionInt64",
      3 => "UserDistributionDouble",
      4 => "UserLatestInt64",
      5 => "UserLatestDouble",
      6 => "UserTopNInt64",
      7 => "UserTopNDouble",
      8 => "UserBottomNInt64",
      9 => "UserBottomNDouble",
      21 => "UserSetString",
      22 => "UserBoundedTrie",
      10 => "ElementCount",
      11 => "SampledByteSize",
      12 => "StartBundleMsecs",
      13 => "ProcessBundleMsecs",
      14 => "FinishBundleMsecs",
      15 => "TotalMsecs",
      16 => "WorkRemaining",
      17 => "WorkCompleted",
      18 => "DataChannelReadIndex",
      19 => "ApiRequestCount",
      20 => "ApiRequestLatencies",
      23 => "UserHistogram",
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
    matches!(value, 0|1|2|3|4|5|6|7|8|9|21|22|10|11|12|13|14|15|16|17|18|19|20|23)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod monitoring_info_specs


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MonitoringInfoLabelProps_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MonitoringInfoLabelProps {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MonitoringInfoLabelProps>
}

impl ::protobuf::Message for MonitoringInfoLabelProps {}

impl ::std::default::Default for MonitoringInfoLabelProps {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MonitoringInfoLabelProps {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MonitoringInfoLabelProps` is `Sync` because it does not implement interior mutability.
//    Neither does `MonitoringInfoLabelPropsMut`.
unsafe impl Sync for MonitoringInfoLabelProps {}

// SAFETY:
// - `MonitoringInfoLabelProps` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MonitoringInfoLabelProps {}

impl ::protobuf::Proxied for MonitoringInfoLabelProps {
  type View<'msg> = MonitoringInfoLabelPropsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MonitoringInfoLabelProps {}

impl ::protobuf::MutProxied for MonitoringInfoLabelProps {
  type Mut<'msg> = MonitoringInfoLabelPropsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MonitoringInfoLabelPropsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoLabelProps>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoLabelPropsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MonitoringInfoLabelPropsView<'msg> {
  type Message = MonitoringInfoLabelProps;
}

impl ::std::fmt::Debug for MonitoringInfoLabelPropsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MonitoringInfoLabelPropsView<'_> {
  fn default() -> MonitoringInfoLabelPropsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoLabelProps>> for MonitoringInfoLabelPropsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoLabelProps>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoLabelPropsView<'msg> {

  pub fn to_owned(&self) -> MonitoringInfoLabelProps {
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

}

// SAFETY:
// - `MonitoringInfoLabelPropsView` is `Sync` because it does not support mutation.
unsafe impl Sync for MonitoringInfoLabelPropsView<'_> {}

// SAFETY:
// - `MonitoringInfoLabelPropsView` is `Send` because while its alive a `MonitoringInfoLabelPropsMut` cannot.
// - `MonitoringInfoLabelPropsView` does not use thread-local data.
unsafe impl Send for MonitoringInfoLabelPropsView<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoLabelPropsView<'msg> {
  type Proxied = MonitoringInfoLabelProps;
  fn as_view(&self) -> ::protobuf::View<'msg, MonitoringInfoLabelProps> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoLabelPropsView<'msg> {
  fn into_view<'shorter>(self) -> MonitoringInfoLabelPropsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoLabelProps> for MonitoringInfoLabelPropsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoLabelProps {
    let mut dst = MonitoringInfoLabelProps::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoLabelProps> for MonitoringInfoLabelPropsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoLabelProps {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MonitoringInfoLabelProps {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoLabelPropsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoLabelPropsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MonitoringInfoLabelPropsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoLabelProps>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoLabelPropsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MonitoringInfoLabelPropsMut<'msg> {
  type Message = MonitoringInfoLabelProps;
}

impl ::std::fmt::Debug for MonitoringInfoLabelPropsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoLabelProps>> for MonitoringInfoLabelPropsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoLabelProps>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoLabelPropsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoLabelProps> {
    self.inner
  }

  pub fn to_owned(&self) -> MonitoringInfoLabelProps {
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

}

// SAFETY:
// - `MonitoringInfoLabelPropsMut` does not perform any shared mutation.
unsafe impl Send for MonitoringInfoLabelPropsMut<'_> {}

// SAFETY:
// - `MonitoringInfoLabelPropsMut` does not perform any shared mutation.
unsafe impl Sync for MonitoringInfoLabelPropsMut<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoLabelPropsMut<'msg> {
  type Proxied = MonitoringInfoLabelProps;
  fn as_view(&self) -> ::protobuf::View<'_, MonitoringInfoLabelProps> {
    MonitoringInfoLabelPropsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoLabelPropsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MonitoringInfoLabelProps>
  where
      'msg: 'shorter {
    MonitoringInfoLabelPropsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MonitoringInfoLabelPropsMut<'msg> {
  type MutProxied = MonitoringInfoLabelProps;
  fn as_mut(&mut self) -> MonitoringInfoLabelPropsMut<'msg> {
    MonitoringInfoLabelPropsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MonitoringInfoLabelPropsMut<'msg> {
  fn into_mut<'shorter>(self) -> MonitoringInfoLabelPropsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MonitoringInfoLabelProps {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MonitoringInfoLabelProps> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MonitoringInfoLabelPropsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MonitoringInfoLabelPropsMut<'_> {
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

}  // impl MonitoringInfoLabelProps

impl ::std::ops::Drop for MonitoringInfoLabelProps {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MonitoringInfoLabelProps {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MonitoringInfoLabelProps {
  type Proxied = Self;
  fn as_view(&self) -> MonitoringInfoLabelPropsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MonitoringInfoLabelProps {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MonitoringInfoLabelPropsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MonitoringInfoLabelProps {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__MonitoringInfoLabelProps_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MonitoringInfoLabelProps_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MonitoringInfoLabelProps_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoLabelProps {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoLabelProps {
  type Msg = MonitoringInfoLabelProps;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoLabelProps> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoLabelProps {
  type Msg = MonitoringInfoLabelProps;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoLabelProps> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoLabelPropsMut<'_> {
  type Msg = MonitoringInfoLabelProps;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoLabelProps> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoLabelPropsMut<'_> {
  type Msg = MonitoringInfoLabelProps;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoLabelProps> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoLabelPropsView<'_> {
  type Msg = MonitoringInfoLabelProps;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoLabelProps> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoLabelPropsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MonitoringInfo_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MonitoringInfo {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MonitoringInfo>
}

impl ::protobuf::Message for MonitoringInfo {}

impl ::std::default::Default for MonitoringInfo {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MonitoringInfo {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MonitoringInfo` is `Sync` because it does not implement interior mutability.
//    Neither does `MonitoringInfoMut`.
unsafe impl Sync for MonitoringInfo {}

// SAFETY:
// - `MonitoringInfo` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MonitoringInfo {}

impl ::protobuf::Proxied for MonitoringInfo {
  type View<'msg> = MonitoringInfoView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MonitoringInfo {}

impl ::protobuf::MutProxied for MonitoringInfo {
  type Mut<'msg> = MonitoringInfoMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MonitoringInfoView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MonitoringInfoView<'msg> {
  type Message = MonitoringInfo;
}

impl ::std::fmt::Debug for MonitoringInfoView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MonitoringInfoView<'_> {
  fn default() -> MonitoringInfoView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfo>> for MonitoringInfoView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoView<'msg> {

  pub fn to_owned(&self) -> MonitoringInfo {
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

  // type: optional string
  pub fn r#type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
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

  // labels: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo.LabelsEntry
  pub fn labels(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // start_time: optional message google.protobuf.Timestamp
  pub fn has_start_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn start_time_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.start_time(), self.has_start_time())
  }
  pub fn start_time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `MonitoringInfoView` is `Sync` because it does not support mutation.
unsafe impl Sync for MonitoringInfoView<'_> {}

// SAFETY:
// - `MonitoringInfoView` is `Send` because while its alive a `MonitoringInfoMut` cannot.
// - `MonitoringInfoView` does not use thread-local data.
unsafe impl Send for MonitoringInfoView<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoView<'msg> {
  type Proxied = MonitoringInfo;
  fn as_view(&self) -> ::protobuf::View<'msg, MonitoringInfo> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoView<'msg> {
  fn into_view<'shorter>(self) -> MonitoringInfoView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfo> for MonitoringInfoView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfo {
    let mut dst = MonitoringInfo::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfo> for MonitoringInfoMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfo {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MonitoringInfo {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MonitoringInfoMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MonitoringInfoMut<'msg> {
  type Message = MonitoringInfo;
}

impl ::std::fmt::Debug for MonitoringInfoMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfo>> for MonitoringInfoMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfo> {
    self.inner
  }

  pub fn to_owned(&self) -> MonitoringInfo {
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

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
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

  // labels: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // start_time: optional message google.protobuf.Timestamp
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn start_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.start_time(), self.has_start_time())
  }
  pub fn start_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn start_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_start_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `MonitoringInfoMut` does not perform any shared mutation.
unsafe impl Send for MonitoringInfoMut<'_> {}

// SAFETY:
// - `MonitoringInfoMut` does not perform any shared mutation.
unsafe impl Sync for MonitoringInfoMut<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoMut<'msg> {
  type Proxied = MonitoringInfo;
  fn as_view(&self) -> ::protobuf::View<'_, MonitoringInfo> {
    MonitoringInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MonitoringInfo>
  where
      'msg: 'shorter {
    MonitoringInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MonitoringInfoMut<'msg> {
  type MutProxied = MonitoringInfo;
  fn as_mut(&mut self) -> MonitoringInfoMut<'msg> {
    MonitoringInfoMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MonitoringInfoMut<'msg> {
  fn into_mut<'shorter>(self) -> MonitoringInfoMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MonitoringInfo {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MonitoringInfo> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MonitoringInfoView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MonitoringInfoMut<'_> {
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

  // type: optional string
  pub fn r#type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
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

  // labels: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // start_time: optional message google.protobuf.Timestamp
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn start_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.start_time(), self.has_start_time())
  }
  pub fn start_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn start_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_start_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl MonitoringInfo

impl ::std::ops::Drop for MonitoringInfo {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MonitoringInfo {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MonitoringInfo {
  type Proxied = Self;
  fn as_view(&self) -> MonitoringInfoView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MonitoringInfo {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MonitoringInfoMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MonitoringInfo {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__MonitoringInfo_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X0PG3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MonitoringInfo_msg_init.0, &[<super::monitoring_info::LabelsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MonitoringInfo_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfo {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfo {
  type Msg = MonitoringInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfo {
  type Msg = MonitoringInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoMut<'_> {
  type Msg = MonitoringInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoMut<'_> {
  type Msg = MonitoringInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoView<'_> {
  type Msg = MonitoringInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfo> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod monitoring_info {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MonitoringInfo__LabelsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct LabelsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LabelsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::monitoring_info::org__apache__beam__model__pipeline__v1__MonitoringInfo__LabelsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::monitoring_info::org__apache__beam__model__pipeline__v1__MonitoringInfo__LabelsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::monitoring_info::org__apache__beam__model__pipeline__v1__MonitoringInfo__LabelsEntry_msg_init.0)
      }).0
    }
  }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonitoringInfoLabels(i32);

#[allow(non_upper_case_globals)]
impl MonitoringInfoLabels {
  pub const Transform: MonitoringInfoLabels = MonitoringInfoLabels(0);
  pub const Pcollection: MonitoringInfoLabels = MonitoringInfoLabels(1);
  pub const WindowingStrategy: MonitoringInfoLabels = MonitoringInfoLabels(2);
  pub const Coder: MonitoringInfoLabels = MonitoringInfoLabels(3);
  pub const Environment: MonitoringInfoLabels = MonitoringInfoLabels(4);
  pub const Namespace: MonitoringInfoLabels = MonitoringInfoLabels(5);
  pub const Name: MonitoringInfoLabels = MonitoringInfoLabels(6);
  pub const Service: MonitoringInfoLabels = MonitoringInfoLabels(7);
  pub const Method: MonitoringInfoLabels = MonitoringInfoLabels(8);
  pub const Resource: MonitoringInfoLabels = MonitoringInfoLabels(9);
  pub const Status: MonitoringInfoLabels = MonitoringInfoLabels(10);
  pub const BigqueryProjectId: MonitoringInfoLabels = MonitoringInfoLabels(11);
  pub const BigqueryDataset: MonitoringInfoLabels = MonitoringInfoLabels(12);
  pub const BigqueryTable: MonitoringInfoLabels = MonitoringInfoLabels(13);
  pub const BigqueryView: MonitoringInfoLabels = MonitoringInfoLabels(14);
  pub const BigqueryQueryName: MonitoringInfoLabels = MonitoringInfoLabels(15);
  pub const GcsBucket: MonitoringInfoLabels = MonitoringInfoLabels(16);
  pub const GcsProjectId: MonitoringInfoLabels = MonitoringInfoLabels(17);
  pub const DatastoreProject: MonitoringInfoLabels = MonitoringInfoLabels(18);
  pub const DatastoreNamespace: MonitoringInfoLabels = MonitoringInfoLabels(19);
  pub const BigtableProjectId: MonitoringInfoLabels = MonitoringInfoLabels(20);
  pub const InstanceId: MonitoringInfoLabels = MonitoringInfoLabels(21);
  pub const TableId: MonitoringInfoLabels = MonitoringInfoLabels(22);
  pub const SpannerProjectId: MonitoringInfoLabels = MonitoringInfoLabels(23);
  pub const SpannerDatabaseId: MonitoringInfoLabels = MonitoringInfoLabels(24);
  pub const SpannerTableId: MonitoringInfoLabels = MonitoringInfoLabels(25);
  pub const SpannerInstanceId: MonitoringInfoLabels = MonitoringInfoLabels(26);
  pub const SpannerQueryName: MonitoringInfoLabels = MonitoringInfoLabels(27);
  pub const PerWorkerMetric: MonitoringInfoLabels = MonitoringInfoLabels(28);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Transform",
      1 => "Pcollection",
      2 => "WindowingStrategy",
      3 => "Coder",
      4 => "Environment",
      5 => "Namespace",
      6 => "Name",
      7 => "Service",
      8 => "Method",
      9 => "Resource",
      10 => "Status",
      11 => "BigqueryProjectId",
      12 => "BigqueryDataset",
      13 => "BigqueryTable",
      14 => "BigqueryView",
      15 => "BigqueryQueryName",
      16 => "GcsBucket",
      17 => "GcsProjectId",
      18 => "DatastoreProject",
      19 => "DatastoreNamespace",
      20 => "BigtableProjectId",
      21 => "InstanceId",
      22 => "TableId",
      23 => "SpannerProjectId",
      24 => "SpannerDatabaseId",
      25 => "SpannerTableId",
      26 => "SpannerInstanceId",
      27 => "SpannerQueryName",
      28 => "PerWorkerMetric",
      _ => return None
    })
  }
}

impl ::std::convert::From<MonitoringInfoLabels> for i32 {
  fn from(val: MonitoringInfoLabels) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for MonitoringInfoLabels {
  fn from(val: i32) -> MonitoringInfoLabels {
    Self(val)
  }
}

impl ::std::default::Default for MonitoringInfoLabels {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for MonitoringInfoLabels {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "MonitoringInfoLabels::{}", constant_name)
    } else {
      write!(f, "MonitoringInfoLabels::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for MonitoringInfoLabels {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for MonitoringInfoLabels {}

impl ::protobuf::Proxied for MonitoringInfoLabels {
  type View<'a> = MonitoringInfoLabels;
}

impl ::protobuf::AsView for MonitoringInfoLabels {
  type Proxied = MonitoringInfoLabels;

  fn as_view(&self) -> MonitoringInfoLabels {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoLabels {
  fn into_view<'shorter>(self) -> MonitoringInfoLabels where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for MonitoringInfoLabels {
  const NAME: &'static str = "MonitoringInfoLabels";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|21|22|23|24|25|26|27|28)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MonitoringInfoLabels {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod monitoring_info


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MonitoringInfoTypeUrns_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MonitoringInfoTypeUrns {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MonitoringInfoTypeUrns>
}

impl ::protobuf::Message for MonitoringInfoTypeUrns {}

impl ::std::default::Default for MonitoringInfoTypeUrns {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MonitoringInfoTypeUrns {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MonitoringInfoTypeUrns` is `Sync` because it does not implement interior mutability.
//    Neither does `MonitoringInfoTypeUrnsMut`.
unsafe impl Sync for MonitoringInfoTypeUrns {}

// SAFETY:
// - `MonitoringInfoTypeUrns` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MonitoringInfoTypeUrns {}

impl ::protobuf::Proxied for MonitoringInfoTypeUrns {
  type View<'msg> = MonitoringInfoTypeUrnsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MonitoringInfoTypeUrns {}

impl ::protobuf::MutProxied for MonitoringInfoTypeUrns {
  type Mut<'msg> = MonitoringInfoTypeUrnsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MonitoringInfoTypeUrnsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoTypeUrns>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoTypeUrnsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MonitoringInfoTypeUrnsView<'msg> {
  type Message = MonitoringInfoTypeUrns;
}

impl ::std::fmt::Debug for MonitoringInfoTypeUrnsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MonitoringInfoTypeUrnsView<'_> {
  fn default() -> MonitoringInfoTypeUrnsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoTypeUrns>> for MonitoringInfoTypeUrnsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MonitoringInfoTypeUrns>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoTypeUrnsView<'msg> {

  pub fn to_owned(&self) -> MonitoringInfoTypeUrns {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `MonitoringInfoTypeUrnsView` is `Sync` because it does not support mutation.
unsafe impl Sync for MonitoringInfoTypeUrnsView<'_> {}

// SAFETY:
// - `MonitoringInfoTypeUrnsView` is `Send` because while its alive a `MonitoringInfoTypeUrnsMut` cannot.
// - `MonitoringInfoTypeUrnsView` does not use thread-local data.
unsafe impl Send for MonitoringInfoTypeUrnsView<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoTypeUrnsView<'msg> {
  type Proxied = MonitoringInfoTypeUrns;
  fn as_view(&self) -> ::protobuf::View<'msg, MonitoringInfoTypeUrns> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoTypeUrnsView<'msg> {
  fn into_view<'shorter>(self) -> MonitoringInfoTypeUrnsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoTypeUrns> for MonitoringInfoTypeUrnsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoTypeUrns {
    let mut dst = MonitoringInfoTypeUrns::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MonitoringInfoTypeUrns> for MonitoringInfoTypeUrnsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MonitoringInfoTypeUrns {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MonitoringInfoTypeUrns {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoTypeUrnsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MonitoringInfoTypeUrnsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MonitoringInfoTypeUrnsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoTypeUrns>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MonitoringInfoTypeUrnsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MonitoringInfoTypeUrnsMut<'msg> {
  type Message = MonitoringInfoTypeUrns;
}

impl ::std::fmt::Debug for MonitoringInfoTypeUrnsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoTypeUrns>> for MonitoringInfoTypeUrnsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoTypeUrns>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MonitoringInfoTypeUrnsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MonitoringInfoTypeUrns> {
    self.inner
  }

  pub fn to_owned(&self) -> MonitoringInfoTypeUrns {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `MonitoringInfoTypeUrnsMut` does not perform any shared mutation.
unsafe impl Send for MonitoringInfoTypeUrnsMut<'_> {}

// SAFETY:
// - `MonitoringInfoTypeUrnsMut` does not perform any shared mutation.
unsafe impl Sync for MonitoringInfoTypeUrnsMut<'_> {}

impl<'msg> ::protobuf::AsView for MonitoringInfoTypeUrnsMut<'msg> {
  type Proxied = MonitoringInfoTypeUrns;
  fn as_view(&self) -> ::protobuf::View<'_, MonitoringInfoTypeUrns> {
    MonitoringInfoTypeUrnsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MonitoringInfoTypeUrnsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MonitoringInfoTypeUrns>
  where
      'msg: 'shorter {
    MonitoringInfoTypeUrnsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MonitoringInfoTypeUrnsMut<'msg> {
  type MutProxied = MonitoringInfoTypeUrns;
  fn as_mut(&mut self) -> MonitoringInfoTypeUrnsMut<'msg> {
    MonitoringInfoTypeUrnsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MonitoringInfoTypeUrnsMut<'msg> {
  fn into_mut<'shorter>(self) -> MonitoringInfoTypeUrnsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MonitoringInfoTypeUrns {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MonitoringInfoTypeUrns> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MonitoringInfoTypeUrnsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MonitoringInfoTypeUrnsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl MonitoringInfoTypeUrns

impl ::std::ops::Drop for MonitoringInfoTypeUrns {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MonitoringInfoTypeUrns {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MonitoringInfoTypeUrns {
  type Proxied = Self;
  fn as_view(&self) -> MonitoringInfoTypeUrnsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MonitoringInfoTypeUrns {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MonitoringInfoTypeUrnsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MonitoringInfoTypeUrns {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__MonitoringInfoTypeUrns_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MonitoringInfoTypeUrns_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MonitoringInfoTypeUrns_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoTypeUrns {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoTypeUrns {
  type Msg = MonitoringInfoTypeUrns;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoTypeUrns> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoTypeUrns {
  type Msg = MonitoringInfoTypeUrns;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoTypeUrns> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MonitoringInfoTypeUrnsMut<'_> {
  type Msg = MonitoringInfoTypeUrns;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoTypeUrns> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoTypeUrnsMut<'_> {
  type Msg = MonitoringInfoTypeUrns;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoTypeUrns> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MonitoringInfoTypeUrnsView<'_> {
  type Msg = MonitoringInfoTypeUrns;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MonitoringInfoTypeUrns> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MonitoringInfoTypeUrnsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod monitoring_info_type_urns {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const SumInt64Type: Enum = Enum(0);
  pub const SumDoubleType: Enum = Enum(1);
  pub const DistributionInt64Type: Enum = Enum(2);
  pub const DistributionDoubleType: Enum = Enum(3);
  pub const LatestInt64Type: Enum = Enum(4);
  pub const LatestDoubleType: Enum = Enum(5);
  pub const TopNInt64Type: Enum = Enum(6);
  pub const TopNDoubleType: Enum = Enum(7);
  pub const BottomNInt64Type: Enum = Enum(8);
  pub const BottomNDoubleType: Enum = Enum(9);
  pub const ProgressType: Enum = Enum(10);
  pub const SetStringType: Enum = Enum(11);
  pub const BoundedTrieType: Enum = Enum(12);
  pub const Histogram: Enum = Enum(13);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "SumInt64Type",
      1 => "SumDoubleType",
      2 => "DistributionInt64Type",
      3 => "DistributionDoubleType",
      4 => "LatestInt64Type",
      5 => "LatestDoubleType",
      6 => "TopNInt64Type",
      7 => "TopNDoubleType",
      8 => "BottomNInt64Type",
      9 => "BottomNDoubleType",
      10 => "ProgressType",
      11 => "SetStringType",
      12 => "BoundedTrieType",
      13 => "Histogram",
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
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11|12|13)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod monitoring_info_type_urns


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__BoundedTrieNode_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BoundedTrieNode {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BoundedTrieNode>
}

impl ::protobuf::Message for BoundedTrieNode {}

impl ::std::default::Default for BoundedTrieNode {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BoundedTrieNode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BoundedTrieNode` is `Sync` because it does not implement interior mutability.
//    Neither does `BoundedTrieNodeMut`.
unsafe impl Sync for BoundedTrieNode {}

// SAFETY:
// - `BoundedTrieNode` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for BoundedTrieNode {}

impl ::protobuf::Proxied for BoundedTrieNode {
  type View<'msg> = BoundedTrieNodeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BoundedTrieNode {}

impl ::protobuf::MutProxied for BoundedTrieNode {
  type Mut<'msg> = BoundedTrieNodeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BoundedTrieNodeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoundedTrieNode>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoundedTrieNodeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BoundedTrieNodeView<'msg> {
  type Message = BoundedTrieNode;
}

impl ::std::fmt::Debug for BoundedTrieNodeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BoundedTrieNodeView<'_> {
  fn default() -> BoundedTrieNodeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BoundedTrieNode>> for BoundedTrieNodeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoundedTrieNode>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoundedTrieNodeView<'msg> {

  pub fn to_owned(&self) -> BoundedTrieNode {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // truncated: optional bool
  pub fn truncated(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // children: repeated message org.apache.beam.model.pipeline.v1.BoundedTrieNode.ChildrenEntry
  pub fn children(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, super::BoundedTrieNode> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::BoundedTrieNode>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `BoundedTrieNodeView` is `Sync` because it does not support mutation.
unsafe impl Sync for BoundedTrieNodeView<'_> {}

// SAFETY:
// - `BoundedTrieNodeView` is `Send` because while its alive a `BoundedTrieNodeMut` cannot.
// - `BoundedTrieNodeView` does not use thread-local data.
unsafe impl Send for BoundedTrieNodeView<'_> {}

impl<'msg> ::protobuf::AsView for BoundedTrieNodeView<'msg> {
  type Proxied = BoundedTrieNode;
  fn as_view(&self) -> ::protobuf::View<'msg, BoundedTrieNode> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoundedTrieNodeView<'msg> {
  fn into_view<'shorter>(self) -> BoundedTrieNodeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BoundedTrieNode> for BoundedTrieNodeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoundedTrieNode {
    let mut dst = BoundedTrieNode::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BoundedTrieNode> for BoundedTrieNodeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoundedTrieNode {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for BoundedTrieNode {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BoundedTrieNodeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BoundedTrieNodeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BoundedTrieNodeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrieNode>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoundedTrieNodeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BoundedTrieNodeMut<'msg> {
  type Message = BoundedTrieNode;
}

impl ::std::fmt::Debug for BoundedTrieNodeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrieNode>> for BoundedTrieNodeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrieNode>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoundedTrieNodeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrieNode> {
    self.inner
  }

  pub fn to_owned(&self) -> BoundedTrieNode {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // truncated: optional bool
  pub fn truncated(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_truncated(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // children: repeated message org.apache.beam.model.pipeline.v1.BoundedTrieNode.ChildrenEntry
  pub fn children(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::BoundedTrieNode> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::BoundedTrieNode>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn children_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::BoundedTrieNode> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_children(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::BoundedTrieNode>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `BoundedTrieNodeMut` does not perform any shared mutation.
unsafe impl Send for BoundedTrieNodeMut<'_> {}

// SAFETY:
// - `BoundedTrieNodeMut` does not perform any shared mutation.
unsafe impl Sync for BoundedTrieNodeMut<'_> {}

impl<'msg> ::protobuf::AsView for BoundedTrieNodeMut<'msg> {
  type Proxied = BoundedTrieNode;
  fn as_view(&self) -> ::protobuf::View<'_, BoundedTrieNode> {
    BoundedTrieNodeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoundedTrieNodeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BoundedTrieNode>
  where
      'msg: 'shorter {
    BoundedTrieNodeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for BoundedTrieNodeMut<'msg> {
  type MutProxied = BoundedTrieNode;
  fn as_mut(&mut self) -> BoundedTrieNodeMut<'msg> {
    BoundedTrieNodeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BoundedTrieNodeMut<'msg> {
  fn into_mut<'shorter>(self) -> BoundedTrieNodeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BoundedTrieNode {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BoundedTrieNode> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BoundedTrieNodeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BoundedTrieNodeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // truncated: optional bool
  pub fn truncated(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_truncated(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // children: repeated message org.apache.beam.model.pipeline.v1.BoundedTrieNode.ChildrenEntry
  pub fn children(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::BoundedTrieNode> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::BoundedTrieNode>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn children_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::BoundedTrieNode> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_children(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::BoundedTrieNode>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl BoundedTrieNode

impl ::std::ops::Drop for BoundedTrieNode {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BoundedTrieNode {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BoundedTrieNode {
  type Proxied = Self;
  fn as_view(&self) -> BoundedTrieNodeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BoundedTrieNode {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BoundedTrieNodeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BoundedTrieNode {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__BoundedTrieNode_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/PG");
        super::bounded_trie_node::org__apache__beam__model__pipeline__v1__BoundedTrieNode__ChildrenEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__BoundedTrieNode_msg_init.0, &[super::bounded_trie_node::org__apache__beam__model__pipeline__v1__BoundedTrieNode__ChildrenEntry_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::bounded_trie_node::org__apache__beam__model__pipeline__v1__BoundedTrieNode__ChildrenEntry_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__BoundedTrieNode_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__BoundedTrieNode_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoundedTrieNode {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoundedTrieNode {
  type Msg = BoundedTrieNode;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrieNode> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoundedTrieNode {
  type Msg = BoundedTrieNode;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrieNode> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoundedTrieNodeMut<'_> {
  type Msg = BoundedTrieNode;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrieNode> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoundedTrieNodeMut<'_> {
  type Msg = BoundedTrieNode;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrieNode> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoundedTrieNodeView<'_> {
  type Msg = BoundedTrieNode;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrieNode> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoundedTrieNodeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod bounded_trie_node {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__BoundedTrieNode__ChildrenEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ChildrenEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ChildrenEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::BoundedTrieNode as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bounded_trie_node::org__apache__beam__model__pipeline__v1__BoundedTrieNode__ChildrenEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod bounded_trie_node


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__BoundedTrie_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BoundedTrie {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BoundedTrie>
}

impl ::protobuf::Message for BoundedTrie {}

impl ::std::default::Default for BoundedTrie {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BoundedTrie {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BoundedTrie` is `Sync` because it does not implement interior mutability.
//    Neither does `BoundedTrieMut`.
unsafe impl Sync for BoundedTrie {}

// SAFETY:
// - `BoundedTrie` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for BoundedTrie {}

impl ::protobuf::Proxied for BoundedTrie {
  type View<'msg> = BoundedTrieView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BoundedTrie {}

impl ::protobuf::MutProxied for BoundedTrie {
  type Mut<'msg> = BoundedTrieMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BoundedTrieView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoundedTrie>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoundedTrieView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BoundedTrieView<'msg> {
  type Message = BoundedTrie;
}

impl ::std::fmt::Debug for BoundedTrieView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BoundedTrieView<'_> {
  fn default() -> BoundedTrieView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BoundedTrie>> for BoundedTrieView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BoundedTrie>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoundedTrieView<'msg> {

  pub fn to_owned(&self) -> BoundedTrie {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bound: optional int32
  pub fn bound(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // root: optional message org.apache.beam.model.pipeline.v1.BoundedTrieNode
  pub fn has_root(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn root_opt(self) -> ::protobuf::Optional<super::BoundedTrieNodeView<'msg>> {
        ::protobuf::Optional::new(self.root(), self.has_root())
  }
  pub fn root(self) -> super::BoundedTrieNodeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BoundedTrieNodeView::default())
  }

  // singleton: repeated string
  pub fn singleton(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
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
// - `BoundedTrieView` is `Sync` because it does not support mutation.
unsafe impl Sync for BoundedTrieView<'_> {}

// SAFETY:
// - `BoundedTrieView` is `Send` because while its alive a `BoundedTrieMut` cannot.
// - `BoundedTrieView` does not use thread-local data.
unsafe impl Send for BoundedTrieView<'_> {}

impl<'msg> ::protobuf::AsView for BoundedTrieView<'msg> {
  type Proxied = BoundedTrie;
  fn as_view(&self) -> ::protobuf::View<'msg, BoundedTrie> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoundedTrieView<'msg> {
  fn into_view<'shorter>(self) -> BoundedTrieView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BoundedTrie> for BoundedTrieView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoundedTrie {
    let mut dst = BoundedTrie::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BoundedTrie> for BoundedTrieMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BoundedTrie {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for BoundedTrie {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BoundedTrieView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BoundedTrieMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BoundedTrieMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrie>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BoundedTrieMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BoundedTrieMut<'msg> {
  type Message = BoundedTrie;
}

impl ::std::fmt::Debug for BoundedTrieMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrie>> for BoundedTrieMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrie>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BoundedTrieMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BoundedTrie> {
    self.inner
  }

  pub fn to_owned(&self) -> BoundedTrie {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bound: optional int32
  pub fn bound(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bound(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // root: optional message org.apache.beam.model.pipeline.v1.BoundedTrieNode
  pub fn has_root(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_root(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn root_opt(&self) -> ::protobuf::Optional<super::BoundedTrieNodeView<'_>> {
        ::protobuf::Optional::new(self.root(), self.has_root())
  }
  pub fn root(&self) -> super::BoundedTrieNodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BoundedTrieNodeView::default())
  }
  pub fn root_mut(&mut self) -> super::BoundedTrieNodeMut<'_> {
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
  pub fn set_root(&mut self,
    val: impl ::protobuf::IntoProxied<super::BoundedTrieNode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // singleton: repeated string
  pub fn singleton(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn singleton_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_singleton(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `BoundedTrieMut` does not perform any shared mutation.
unsafe impl Send for BoundedTrieMut<'_> {}

// SAFETY:
// - `BoundedTrieMut` does not perform any shared mutation.
unsafe impl Sync for BoundedTrieMut<'_> {}

impl<'msg> ::protobuf::AsView for BoundedTrieMut<'msg> {
  type Proxied = BoundedTrie;
  fn as_view(&self) -> ::protobuf::View<'_, BoundedTrie> {
    BoundedTrieView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BoundedTrieMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BoundedTrie>
  where
      'msg: 'shorter {
    BoundedTrieView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for BoundedTrieMut<'msg> {
  type MutProxied = BoundedTrie;
  fn as_mut(&mut self) -> BoundedTrieMut<'msg> {
    BoundedTrieMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BoundedTrieMut<'msg> {
  fn into_mut<'shorter>(self) -> BoundedTrieMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BoundedTrie {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BoundedTrie> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BoundedTrieView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BoundedTrieMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bound: optional int32
  pub fn bound(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bound(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // root: optional message org.apache.beam.model.pipeline.v1.BoundedTrieNode
  pub fn has_root(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_root(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn root_opt(&self) -> ::protobuf::Optional<super::BoundedTrieNodeView<'_>> {
        ::protobuf::Optional::new(self.root(), self.has_root())
  }
  pub fn root(&self) -> super::BoundedTrieNodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BoundedTrieNodeView::default())
  }
  pub fn root_mut(&mut self) -> super::BoundedTrieNodeMut<'_> {
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
  pub fn set_root(&mut self,
    val: impl ::protobuf::IntoProxied<super::BoundedTrieNode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // singleton: repeated string
  pub fn singleton(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn singleton_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_singleton(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl BoundedTrie

impl ::std::ops::Drop for BoundedTrie {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BoundedTrie {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BoundedTrie {
  type Proxied = Self;
  fn as_view(&self) -> BoundedTrieView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BoundedTrie {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BoundedTrieMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BoundedTrie {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__BoundedTrie_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P3ET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__BoundedTrie_msg_init.0, &[<super::BoundedTrieNode as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__BoundedTrie_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoundedTrie {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoundedTrie {
  type Msg = BoundedTrie;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrie> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoundedTrie {
  type Msg = BoundedTrie;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrie> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BoundedTrieMut<'_> {
  type Msg = BoundedTrie;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrie> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoundedTrieMut<'_> {
  type Msg = BoundedTrie;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrie> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BoundedTrieView<'_> {
  type Msg = BoundedTrie;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BoundedTrie> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BoundedTrieMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__HistogramValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HistogramValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HistogramValue>
}

impl ::protobuf::Message for HistogramValue {}

impl ::std::default::Default for HistogramValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HistogramValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HistogramValue` is `Sync` because it does not implement interior mutability.
//    Neither does `HistogramValueMut`.
unsafe impl Sync for HistogramValue {}

// SAFETY:
// - `HistogramValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for HistogramValue {}

impl ::protobuf::Proxied for HistogramValue {
  type View<'msg> = HistogramValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HistogramValue {}

impl ::protobuf::MutProxied for HistogramValue {
  type Mut<'msg> = HistogramValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HistogramValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HistogramValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HistogramValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HistogramValueView<'msg> {
  type Message = HistogramValue;
}

impl ::std::fmt::Debug for HistogramValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HistogramValueView<'_> {
  fn default() -> HistogramValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HistogramValue>> for HistogramValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HistogramValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HistogramValueView<'msg> {

  pub fn to_owned(&self) -> HistogramValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // count: optional int64
  pub fn has_count(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn count_opt(self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.count(), self.has_count())
  }
  pub fn count(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // bucket_options: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions
  pub fn has_bucket_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn bucket_options_opt(self) -> ::protobuf::Optional<super::histogram_value::BucketOptionsView<'msg>> {
        ::protobuf::Optional::new(self.bucket_options(), self.has_bucket_options())
  }
  pub fn bucket_options(self) -> super::histogram_value::BucketOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::histogram_value::BucketOptionsView::default())
  }

  // bucket_counts: repeated int64
  pub fn bucket_counts(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `HistogramValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for HistogramValueView<'_> {}

// SAFETY:
// - `HistogramValueView` is `Send` because while its alive a `HistogramValueMut` cannot.
// - `HistogramValueView` does not use thread-local data.
unsafe impl Send for HistogramValueView<'_> {}

impl<'msg> ::protobuf::AsView for HistogramValueView<'msg> {
  type Proxied = HistogramValue;
  fn as_view(&self) -> ::protobuf::View<'msg, HistogramValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HistogramValueView<'msg> {
  fn into_view<'shorter>(self) -> HistogramValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HistogramValue> for HistogramValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HistogramValue {
    let mut dst = HistogramValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HistogramValue> for HistogramValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HistogramValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for HistogramValue {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HistogramValueView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for HistogramValueMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HistogramValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HistogramValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HistogramValueMut<'msg> {
  type Message = HistogramValue;
}

impl ::std::fmt::Debug for HistogramValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramValue>> for HistogramValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HistogramValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramValue> {
    self.inner
  }

  pub fn to_owned(&self) -> HistogramValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // count: optional int64
  pub fn has_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn count_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.count(), self.has_count())
  }
  pub fn count(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_count(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // bucket_options: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions
  pub fn has_bucket_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_bucket_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn bucket_options_opt(&self) -> ::protobuf::Optional<super::histogram_value::BucketOptionsView<'_>> {
        ::protobuf::Optional::new(self.bucket_options(), self.has_bucket_options())
  }
  pub fn bucket_options(&self) -> super::histogram_value::BucketOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::histogram_value::BucketOptionsView::default())
  }
  pub fn bucket_options_mut(&mut self) -> super::histogram_value::BucketOptionsMut<'_> {
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
  pub fn set_bucket_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::histogram_value::BucketOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // bucket_counts: repeated int64
  pub fn bucket_counts(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bucket_counts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_bucket_counts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `HistogramValueMut` does not perform any shared mutation.
unsafe impl Send for HistogramValueMut<'_> {}

// SAFETY:
// - `HistogramValueMut` does not perform any shared mutation.
unsafe impl Sync for HistogramValueMut<'_> {}

impl<'msg> ::protobuf::AsView for HistogramValueMut<'msg> {
  type Proxied = HistogramValue;
  fn as_view(&self) -> ::protobuf::View<'_, HistogramValue> {
    HistogramValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HistogramValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HistogramValue>
  where
      'msg: 'shorter {
    HistogramValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for HistogramValueMut<'msg> {
  type MutProxied = HistogramValue;
  fn as_mut(&mut self) -> HistogramValueMut<'msg> {
    HistogramValueMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HistogramValueMut<'msg> {
  fn into_mut<'shorter>(self) -> HistogramValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HistogramValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HistogramValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HistogramValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HistogramValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // count: optional int64
  pub fn has_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn count_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.count(), self.has_count())
  }
  pub fn count(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_count(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // bucket_options: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions
  pub fn has_bucket_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_bucket_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn bucket_options_opt(&self) -> ::protobuf::Optional<super::histogram_value::BucketOptionsView<'_>> {
        ::protobuf::Optional::new(self.bucket_options(), self.has_bucket_options())
  }
  pub fn bucket_options(&self) -> super::histogram_value::BucketOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::histogram_value::BucketOptionsView::default())
  }
  pub fn bucket_options_mut(&mut self) -> super::histogram_value::BucketOptionsMut<'_> {
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
  pub fn set_bucket_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::histogram_value::BucketOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // bucket_counts: repeated int64
  pub fn bucket_counts(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bucket_counts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_bucket_counts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl HistogramValue

impl ::std::ops::Drop for HistogramValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HistogramValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HistogramValue {
  type Proxied = Self;
  fn as_view(&self) -> HistogramValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HistogramValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HistogramValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HistogramValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__HistogramValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N+3?");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__HistogramValue_msg_init.0, &[<super::histogram_value::BucketOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__HistogramValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HistogramValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HistogramValue {
  type Msg = HistogramValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HistogramValue {
  type Msg = HistogramValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HistogramValueMut<'_> {
  type Msg = HistogramValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HistogramValueMut<'_> {
  type Msg = HistogramValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HistogramValueView<'_> {
  type Msg = HistogramValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HistogramValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod histogram_value {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BucketOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BucketOptions>
}

impl ::protobuf::Message for BucketOptions {}

impl ::std::default::Default for BucketOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BucketOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BucketOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `BucketOptionsMut`.
unsafe impl Sync for BucketOptions {}

// SAFETY:
// - `BucketOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for BucketOptions {}

impl ::protobuf::Proxied for BucketOptions {
  type View<'msg> = BucketOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BucketOptions {}

impl ::protobuf::MutProxied for BucketOptions {
  type Mut<'msg> = BucketOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BucketOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BucketOptionsView<'msg> {
  type Message = BucketOptions;
}

impl ::std::fmt::Debug for BucketOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BucketOptionsView<'_> {
  fn default() -> BucketOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BucketOptions>> for BucketOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketOptionsView<'msg> {

  pub fn to_owned(&self) -> BucketOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // linear: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions.Linear
  pub fn has_linear(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn linear_opt(self) -> ::protobuf::Optional<super::super::histogram_value::bucket_options::LinearView<'msg>> {
        ::protobuf::Optional::new(self.linear(), self.has_linear())
  }
  pub fn linear(self) -> super::super::histogram_value::bucket_options::LinearView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::histogram_value::bucket_options::LinearView::default())
  }

  // exponential: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions.Base2Exponent
  pub fn has_exponential(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn exponential_opt(self) -> ::protobuf::Optional<super::super::histogram_value::bucket_options::Base2ExponentView<'msg>> {
        ::protobuf::Optional::new(self.exponential(), self.has_exponential())
  }
  pub fn exponential(self) -> super::super::histogram_value::bucket_options::Base2ExponentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::histogram_value::bucket_options::Base2ExponentView::default())
  }

  pub fn bucket_type(self) -> super::super::histogram_value::bucket_options::BucketTypeOneof<'msg> {
    match self.bucket_type_case() {
      super::super::histogram_value::bucket_options::BucketTypeCase::Linear =>
          super::super::histogram_value::bucket_options::BucketTypeOneof::Linear(self.linear()),
      super::super::histogram_value::bucket_options::BucketTypeCase::Exponential =>
          super::super::histogram_value::bucket_options::BucketTypeOneof::Exponential(self.exponential()),
      _ => super::super::histogram_value::bucket_options::BucketTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn bucket_type_case(self) -> super::super::histogram_value::bucket_options::BucketTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::histogram_value::bucket_options::BucketTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BucketOptionsView` is `Sync` because it does not support mutation.
unsafe impl Sync for BucketOptionsView<'_> {}

// SAFETY:
// - `BucketOptionsView` is `Send` because while its alive a `BucketOptionsMut` cannot.
// - `BucketOptionsView` does not use thread-local data.
unsafe impl Send for BucketOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for BucketOptionsView<'msg> {
  type Proxied = BucketOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, BucketOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketOptionsView<'msg> {
  fn into_view<'shorter>(self) -> BucketOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketOptions> for BucketOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketOptions {
    let mut dst = BucketOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketOptions> for BucketOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for BucketOptions {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BucketOptionsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for BucketOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BucketOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BucketOptionsMut<'msg> {
  type Message = BucketOptions;
}

impl ::std::fmt::Debug for BucketOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BucketOptions>> for BucketOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketOptions> {
    self.inner
  }

  pub fn to_owned(&self) -> BucketOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // linear: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions.Linear
  pub fn has_linear(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_linear(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn linear_opt(&self) -> ::protobuf::Optional<super::super::histogram_value::bucket_options::LinearView<'_>> {
        ::protobuf::Optional::new(self.linear(), self.has_linear())
  }
  pub fn linear(&self) -> super::super::histogram_value::bucket_options::LinearView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::histogram_value::bucket_options::LinearView::default())
  }
  pub fn linear_mut(&mut self) -> super::super::histogram_value::bucket_options::LinearMut<'_> {
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
  pub fn set_linear(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::histogram_value::bucket_options::Linear>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exponential: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions.Base2Exponent
  pub fn has_exponential(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exponential(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exponential_opt(&self) -> ::protobuf::Optional<super::super::histogram_value::bucket_options::Base2ExponentView<'_>> {
        ::protobuf::Optional::new(self.exponential(), self.has_exponential())
  }
  pub fn exponential(&self) -> super::super::histogram_value::bucket_options::Base2ExponentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::histogram_value::bucket_options::Base2ExponentView::default())
  }
  pub fn exponential_mut(&mut self) -> super::super::histogram_value::bucket_options::Base2ExponentMut<'_> {
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
  pub fn set_exponential(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::histogram_value::bucket_options::Base2Exponent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn bucket_type(&self) -> super::super::histogram_value::bucket_options::BucketTypeOneof<'_> {
    match &self.bucket_type_case() {
      super::super::histogram_value::bucket_options::BucketTypeCase::Linear =>
          super::super::histogram_value::bucket_options::BucketTypeOneof::Linear(self.linear()),
      super::super::histogram_value::bucket_options::BucketTypeCase::Exponential =>
          super::super::histogram_value::bucket_options::BucketTypeOneof::Exponential(self.exponential()),
      _ => super::super::histogram_value::bucket_options::BucketTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn bucket_type_case(&self) -> super::super::histogram_value::bucket_options::BucketTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::histogram_value::bucket_options::BucketTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BucketOptionsMut` does not perform any shared mutation.
unsafe impl Send for BucketOptionsMut<'_> {}

// SAFETY:
// - `BucketOptionsMut` does not perform any shared mutation.
unsafe impl Sync for BucketOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for BucketOptionsMut<'msg> {
  type Proxied = BucketOptions;
  fn as_view(&self) -> ::protobuf::View<'_, BucketOptions> {
    BucketOptionsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BucketOptions>
  where
      'msg: 'shorter {
    BucketOptionsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for BucketOptionsMut<'msg> {
  type MutProxied = BucketOptions;
  fn as_mut(&mut self) -> BucketOptionsMut<'msg> {
    BucketOptionsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BucketOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> BucketOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BucketOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BucketOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BucketOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BucketOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // linear: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions.Linear
  pub fn has_linear(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_linear(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn linear_opt(&self) -> ::protobuf::Optional<super::super::histogram_value::bucket_options::LinearView<'_>> {
        ::protobuf::Optional::new(self.linear(), self.has_linear())
  }
  pub fn linear(&self) -> super::super::histogram_value::bucket_options::LinearView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::histogram_value::bucket_options::LinearView::default())
  }
  pub fn linear_mut(&mut self) -> super::super::histogram_value::bucket_options::LinearMut<'_> {
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
  pub fn set_linear(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::histogram_value::bucket_options::Linear>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exponential: optional message org.apache.beam.model.pipeline.v1.HistogramValue.BucketOptions.Base2Exponent
  pub fn has_exponential(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exponential(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exponential_opt(&self) -> ::protobuf::Optional<super::super::histogram_value::bucket_options::Base2ExponentView<'_>> {
        ::protobuf::Optional::new(self.exponential(), self.has_exponential())
  }
  pub fn exponential(&self) -> super::super::histogram_value::bucket_options::Base2ExponentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::histogram_value::bucket_options::Base2ExponentView::default())
  }
  pub fn exponential_mut(&mut self) -> super::super::histogram_value::bucket_options::Base2ExponentMut<'_> {
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
  pub fn set_exponential(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::histogram_value::bucket_options::Base2Exponent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn bucket_type(&self) -> super::super::histogram_value::bucket_options::BucketTypeOneof<'_> {
    match &self.bucket_type_case() {
      super::super::histogram_value::bucket_options::BucketTypeCase::Linear =>
          super::super::histogram_value::bucket_options::BucketTypeOneof::Linear(self.linear()),
      super::super::histogram_value::bucket_options::BucketTypeCase::Exponential =>
          super::super::histogram_value::bucket_options::BucketTypeOneof::Exponential(self.exponential()),
      _ => super::super::histogram_value::bucket_options::BucketTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn bucket_type_case(&self) -> super::super::histogram_value::bucket_options::BucketTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::histogram_value::bucket_options::BucketTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl BucketOptions

impl ::std::ops::Drop for BucketOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BucketOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BucketOptions {
  type Proxied = Self;
  fn as_view(&self) -> BucketOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BucketOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BucketOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BucketOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::histogram_value::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::histogram_value::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions_msg_init.0, &[<super::super::histogram_value::bucket_options::Linear as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::histogram_value::bucket_options::Base2Exponent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::histogram_value::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketOptions {
  type Msg = BucketOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketOptions {
  type Msg = BucketOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketOptionsMut<'_> {
  type Msg = BucketOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketOptionsMut<'_> {
  type Msg = BucketOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketOptionsView<'_> {
  type Msg = BucketOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod bucket_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Linear_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Linear {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Linear>
}

impl ::protobuf::Message for Linear {}

impl ::std::default::Default for Linear {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Linear {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Linear` is `Sync` because it does not implement interior mutability.
//    Neither does `LinearMut`.
unsafe impl Sync for Linear {}

// SAFETY:
// - `Linear` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Linear {}

impl ::protobuf::Proxied for Linear {
  type View<'msg> = LinearView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Linear {}

impl ::protobuf::MutProxied for Linear {
  type Mut<'msg> = LinearMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LinearView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Linear>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LinearView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LinearView<'msg> {
  type Message = Linear;
}

impl ::std::fmt::Debug for LinearView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LinearView<'_> {
  fn default() -> LinearView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Linear>> for LinearView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Linear>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LinearView<'msg> {

  pub fn to_owned(&self) -> Linear {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // number_of_buckets: optional int32
  pub fn has_number_of_buckets(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn number_of_buckets_opt(self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.number_of_buckets(), self.has_number_of_buckets())
  }
  pub fn number_of_buckets(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // width: optional double
  pub fn has_width(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn width_opt(self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.width(), self.has_width())
  }
  pub fn width(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // start: optional double
  pub fn has_start(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn start_opt(self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.start(), self.has_start())
  }
  pub fn start(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `LinearView` is `Sync` because it does not support mutation.
unsafe impl Sync for LinearView<'_> {}

// SAFETY:
// - `LinearView` is `Send` because while its alive a `LinearMut` cannot.
// - `LinearView` does not use thread-local data.
unsafe impl Send for LinearView<'_> {}

impl<'msg> ::protobuf::AsView for LinearView<'msg> {
  type Proxied = Linear;
  fn as_view(&self) -> ::protobuf::View<'msg, Linear> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LinearView<'msg> {
  fn into_view<'shorter>(self) -> LinearView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Linear> for LinearView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Linear {
    let mut dst = Linear::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Linear> for LinearMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Linear {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Linear {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LinearView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LinearMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LinearMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Linear>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LinearMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LinearMut<'msg> {
  type Message = Linear;
}

impl ::std::fmt::Debug for LinearMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Linear>> for LinearMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Linear>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LinearMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Linear> {
    self.inner
  }

  pub fn to_owned(&self) -> Linear {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // number_of_buckets: optional int32
  pub fn has_number_of_buckets(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_number_of_buckets(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn number_of_buckets_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.number_of_buckets(), self.has_number_of_buckets())
  }
  pub fn number_of_buckets(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_buckets(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // width: optional double
  pub fn has_width(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_width(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn width_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.width(), self.has_width())
  }
  pub fn width(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_width(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

  // start: optional double
  pub fn has_start(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.start(), self.has_start())
  }
  pub fn start(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `LinearMut` does not perform any shared mutation.
unsafe impl Send for LinearMut<'_> {}

// SAFETY:
// - `LinearMut` does not perform any shared mutation.
unsafe impl Sync for LinearMut<'_> {}

impl<'msg> ::protobuf::AsView for LinearMut<'msg> {
  type Proxied = Linear;
  fn as_view(&self) -> ::protobuf::View<'_, Linear> {
    LinearView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LinearMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Linear>
  where
      'msg: 'shorter {
    LinearView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for LinearMut<'msg> {
  type MutProxied = Linear;
  fn as_mut(&mut self) -> LinearMut<'msg> {
    LinearMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LinearMut<'msg> {
  fn into_mut<'shorter>(self) -> LinearMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Linear {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Linear> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LinearView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LinearMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // number_of_buckets: optional int32
  pub fn has_number_of_buckets(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_number_of_buckets(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn number_of_buckets_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.number_of_buckets(), self.has_number_of_buckets())
  }
  pub fn number_of_buckets(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_buckets(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // width: optional double
  pub fn has_width(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_width(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn width_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.width(), self.has_width())
  }
  pub fn width(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_width(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

  // start: optional double
  pub fn has_start(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.start(), self.has_start())
  }
  pub fn start(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

}  // impl Linear

impl ::std::ops::Drop for Linear {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Linear {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Linear {
  type Proxied = Self;
  fn as_view(&self) -> LinearView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Linear {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LinearMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Linear {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::histogram_value::bucket_options::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Linear_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(  ");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::histogram_value::bucket_options::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Linear_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::histogram_value::bucket_options::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Linear_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Linear {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Linear {
  type Msg = Linear;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Linear> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Linear {
  type Msg = Linear;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Linear> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LinearMut<'_> {
  type Msg = Linear;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Linear> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LinearMut<'_> {
  type Msg = Linear;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Linear> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LinearView<'_> {
  type Msg = Linear;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Linear> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LinearMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Base2Exponent_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Base2Exponent {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Base2Exponent>
}

impl ::protobuf::Message for Base2Exponent {}

impl ::std::default::Default for Base2Exponent {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Base2Exponent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Base2Exponent` is `Sync` because it does not implement interior mutability.
//    Neither does `Base2ExponentMut`.
unsafe impl Sync for Base2Exponent {}

// SAFETY:
// - `Base2Exponent` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Base2Exponent {}

impl ::protobuf::Proxied for Base2Exponent {
  type View<'msg> = Base2ExponentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Base2Exponent {}

impl ::protobuf::MutProxied for Base2Exponent {
  type Mut<'msg> = Base2ExponentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Base2ExponentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Base2Exponent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Base2ExponentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Base2ExponentView<'msg> {
  type Message = Base2Exponent;
}

impl ::std::fmt::Debug for Base2ExponentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Base2ExponentView<'_> {
  fn default() -> Base2ExponentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Base2Exponent>> for Base2ExponentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Base2Exponent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Base2ExponentView<'msg> {

  pub fn to_owned(&self) -> Base2Exponent {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // number_of_buckets: optional int32
  pub fn has_number_of_buckets(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn number_of_buckets_opt(self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.number_of_buckets(), self.has_number_of_buckets())
  }
  pub fn number_of_buckets(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // scale: optional int32
  pub fn has_scale(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn scale_opt(self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.scale(), self.has_scale())
  }
  pub fn scale(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Base2ExponentView` is `Sync` because it does not support mutation.
unsafe impl Sync for Base2ExponentView<'_> {}

// SAFETY:
// - `Base2ExponentView` is `Send` because while its alive a `Base2ExponentMut` cannot.
// - `Base2ExponentView` does not use thread-local data.
unsafe impl Send for Base2ExponentView<'_> {}

impl<'msg> ::protobuf::AsView for Base2ExponentView<'msg> {
  type Proxied = Base2Exponent;
  fn as_view(&self) -> ::protobuf::View<'msg, Base2Exponent> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Base2ExponentView<'msg> {
  fn into_view<'shorter>(self) -> Base2ExponentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Base2Exponent> for Base2ExponentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Base2Exponent {
    let mut dst = Base2Exponent::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Base2Exponent> for Base2ExponentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Base2Exponent {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Base2Exponent {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Base2ExponentView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for Base2ExponentMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Base2ExponentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Base2Exponent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Base2ExponentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Base2ExponentMut<'msg> {
  type Message = Base2Exponent;
}

impl ::std::fmt::Debug for Base2ExponentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Base2Exponent>> for Base2ExponentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Base2Exponent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Base2ExponentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Base2Exponent> {
    self.inner
  }

  pub fn to_owned(&self) -> Base2Exponent {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // number_of_buckets: optional int32
  pub fn has_number_of_buckets(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_number_of_buckets(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn number_of_buckets_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.number_of_buckets(), self.has_number_of_buckets())
  }
  pub fn number_of_buckets(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_buckets(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // scale: optional int32
  pub fn has_scale(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_scale(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn scale_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.scale(), self.has_scale())
  }
  pub fn scale(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scale(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `Base2ExponentMut` does not perform any shared mutation.
unsafe impl Send for Base2ExponentMut<'_> {}

// SAFETY:
// - `Base2ExponentMut` does not perform any shared mutation.
unsafe impl Sync for Base2ExponentMut<'_> {}

impl<'msg> ::protobuf::AsView for Base2ExponentMut<'msg> {
  type Proxied = Base2Exponent;
  fn as_view(&self) -> ::protobuf::View<'_, Base2Exponent> {
    Base2ExponentView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Base2ExponentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Base2Exponent>
  where
      'msg: 'shorter {
    Base2ExponentView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for Base2ExponentMut<'msg> {
  type MutProxied = Base2Exponent;
  fn as_mut(&mut self) -> Base2ExponentMut<'msg> {
    Base2ExponentMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Base2ExponentMut<'msg> {
  fn into_mut<'shorter>(self) -> Base2ExponentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Base2Exponent {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Base2Exponent> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Base2ExponentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Base2ExponentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // number_of_buckets: optional int32
  pub fn has_number_of_buckets(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_number_of_buckets(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn number_of_buckets_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.number_of_buckets(), self.has_number_of_buckets())
  }
  pub fn number_of_buckets(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_number_of_buckets(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // scale: optional int32
  pub fn has_scale(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_scale(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn scale_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.scale(), self.has_scale())
  }
  pub fn scale(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scale(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}  // impl Base2Exponent

impl ::std::ops::Drop for Base2Exponent {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Base2Exponent {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Base2Exponent {
  type Proxied = Self;
  fn as_view(&self) -> Base2ExponentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Base2Exponent {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Base2ExponentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Base2Exponent {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::histogram_value::bucket_options::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Base2Exponent_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$((");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::histogram_value::bucket_options::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Base2Exponent_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::histogram_value::bucket_options::org__apache__beam__model__pipeline__v1__HistogramValue__BucketOptions__Base2Exponent_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Base2Exponent {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Base2Exponent {
  type Msg = Base2Exponent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Base2Exponent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Base2Exponent {
  type Msg = Base2Exponent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Base2Exponent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Base2ExponentMut<'_> {
  type Msg = Base2Exponent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Base2Exponent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Base2ExponentMut<'_> {
  type Msg = Base2Exponent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Base2Exponent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Base2ExponentView<'_> {
  type Msg = Base2Exponent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Base2Exponent> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Base2ExponentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum BucketTypeOneof<'msg> {
  Linear(::protobuf::View<'msg, super::super::super::histogram_value::bucket_options::Linear>) = 1,
  Exponential(::protobuf::View<'msg, super::super::super::histogram_value::bucket_options::Base2Exponent>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum BucketTypeCase {
  Linear = 1,
  Exponential = 2,

  not_set = 0
}

impl BucketTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<BucketTypeCase> {
    match v {
      0 => Some(BucketTypeCase::not_set),
      1 => Some(BucketTypeCase::Linear),
      2 => Some(BucketTypeCase::Exponential),
      _ => None
    }
  }
}
}  // pub mod bucket_options


}  // pub mod histogram_value


