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
pub(crate) static mut org__apache__beam__model__job_0management__v1__PrepareJobRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PrepareJobRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PrepareJobRequest>
}

impl ::protobuf::Message for PrepareJobRequest {}

impl ::std::default::Default for PrepareJobRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PrepareJobRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PrepareJobRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PrepareJobRequestMut`.
unsafe impl Sync for PrepareJobRequest {}

// SAFETY:
// - `PrepareJobRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PrepareJobRequest {}

impl ::protobuf::Proxied for PrepareJobRequest {
  type View<'msg> = PrepareJobRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PrepareJobRequest {}

impl ::protobuf::MutProxied for PrepareJobRequest {
  type Mut<'msg> = PrepareJobRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PrepareJobRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PrepareJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrepareJobRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PrepareJobRequestView<'msg> {
  type Message = PrepareJobRequest;
}

impl ::std::fmt::Debug for PrepareJobRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PrepareJobRequestView<'_> {
  fn default() -> PrepareJobRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PrepareJobRequest>> for PrepareJobRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PrepareJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrepareJobRequestView<'msg> {

  pub fn to_owned(&self) -> PrepareJobRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // pipeline: optional message org.apache.beam.model.pipeline.v1.Pipeline
  pub fn has_pipeline(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn pipeline_opt(self) -> ::protobuf::Optional<super::PipelineView<'msg>> {
        ::protobuf::Optional::new(self.pipeline(), self.has_pipeline())
  }
  pub fn pipeline(self) -> super::PipelineView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipelineView::default())
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn pipeline_options_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'msg>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // job_name: optional string
  pub fn job_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `PrepareJobRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PrepareJobRequestView<'_> {}

// SAFETY:
// - `PrepareJobRequestView` is `Send` because while its alive a `PrepareJobRequestMut` cannot.
// - `PrepareJobRequestView` does not use thread-local data.
unsafe impl Send for PrepareJobRequestView<'_> {}

impl<'msg> ::protobuf::AsView for PrepareJobRequestView<'msg> {
  type Proxied = PrepareJobRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PrepareJobRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrepareJobRequestView<'msg> {
  fn into_view<'shorter>(self) -> PrepareJobRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PrepareJobRequest> for PrepareJobRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PrepareJobRequest {
    let mut dst = PrepareJobRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PrepareJobRequest> for PrepareJobRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PrepareJobRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PrepareJobRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PrepareJobRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PrepareJobRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PrepareJobRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrepareJobRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PrepareJobRequestMut<'msg> {
  type Message = PrepareJobRequest;
}

impl ::std::fmt::Debug for PrepareJobRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobRequest>> for PrepareJobRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrepareJobRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PrepareJobRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // pipeline: optional message org.apache.beam.model.pipeline.v1.Pipeline
  pub fn has_pipeline(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pipeline(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pipeline_opt(&self) -> ::protobuf::Optional<super::PipelineView<'_>> {
        ::protobuf::Optional::new(self.pipeline(), self.has_pipeline())
  }
  pub fn pipeline(&self) -> super::PipelineView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipelineView::default())
  }
  pub fn pipeline_mut(&mut self) -> super::PipelineMut<'_> {
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
  pub fn set_pipeline(&mut self,
    val: impl ::protobuf::IntoProxied<super::Pipeline>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // job_name: optional string
  pub fn job_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `PrepareJobRequestMut` does not perform any shared mutation.
unsafe impl Send for PrepareJobRequestMut<'_> {}

// SAFETY:
// - `PrepareJobRequestMut` does not perform any shared mutation.
unsafe impl Sync for PrepareJobRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for PrepareJobRequestMut<'msg> {
  type Proxied = PrepareJobRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PrepareJobRequest> {
    PrepareJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrepareJobRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PrepareJobRequest>
  where
      'msg: 'shorter {
    PrepareJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PrepareJobRequestMut<'msg> {
  type MutProxied = PrepareJobRequest;
  fn as_mut(&mut self) -> PrepareJobRequestMut<'msg> {
    PrepareJobRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PrepareJobRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PrepareJobRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PrepareJobRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PrepareJobRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PrepareJobRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PrepareJobRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // pipeline: optional message org.apache.beam.model.pipeline.v1.Pipeline
  pub fn has_pipeline(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pipeline(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pipeline_opt(&self) -> ::protobuf::Optional<super::PipelineView<'_>> {
        ::protobuf::Optional::new(self.pipeline(), self.has_pipeline())
  }
  pub fn pipeline(&self) -> super::PipelineView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipelineView::default())
  }
  pub fn pipeline_mut(&mut self) -> super::PipelineMut<'_> {
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
  pub fn set_pipeline(&mut self,
    val: impl ::protobuf::IntoProxied<super::Pipeline>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // job_name: optional string
  pub fn job_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl PrepareJobRequest

impl ::std::ops::Drop for PrepareJobRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PrepareJobRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PrepareJobRequest {
  type Proxied = Self;
  fn as_view(&self) -> PrepareJobRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PrepareJobRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PrepareJobRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PrepareJobRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__PrepareJobRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__PrepareJobRequest_msg_init.0, &[<super::Pipeline as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__PrepareJobRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PrepareJobRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PrepareJobRequest {
  type Msg = PrepareJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrepareJobRequest {
  type Msg = PrepareJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PrepareJobRequestMut<'_> {
  type Msg = PrepareJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrepareJobRequestMut<'_> {
  type Msg = PrepareJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrepareJobRequestView<'_> {
  type Msg = PrepareJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PrepareJobRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__PrepareJobResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PrepareJobResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PrepareJobResponse>
}

impl ::protobuf::Message for PrepareJobResponse {}

impl ::std::default::Default for PrepareJobResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PrepareJobResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PrepareJobResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PrepareJobResponseMut`.
unsafe impl Sync for PrepareJobResponse {}

// SAFETY:
// - `PrepareJobResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PrepareJobResponse {}

impl ::protobuf::Proxied for PrepareJobResponse {
  type View<'msg> = PrepareJobResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PrepareJobResponse {}

impl ::protobuf::MutProxied for PrepareJobResponse {
  type Mut<'msg> = PrepareJobResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PrepareJobResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PrepareJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrepareJobResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PrepareJobResponseView<'msg> {
  type Message = PrepareJobResponse;
}

impl ::std::fmt::Debug for PrepareJobResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PrepareJobResponseView<'_> {
  fn default() -> PrepareJobResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PrepareJobResponse>> for PrepareJobResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PrepareJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrepareJobResponseView<'msg> {

  pub fn to_owned(&self) -> PrepareJobResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // preparation_id: optional string
  pub fn preparation_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // artifact_staging_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_artifact_staging_endpoint(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn artifact_staging_endpoint_opt(self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'msg>> {
        ::protobuf::Optional::new(self.artifact_staging_endpoint(), self.has_artifact_staging_endpoint())
  }
  pub fn artifact_staging_endpoint(self) -> super::ApiServiceDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }

  // staging_session_token: optional string
  pub fn staging_session_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `PrepareJobResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PrepareJobResponseView<'_> {}

// SAFETY:
// - `PrepareJobResponseView` is `Send` because while its alive a `PrepareJobResponseMut` cannot.
// - `PrepareJobResponseView` does not use thread-local data.
unsafe impl Send for PrepareJobResponseView<'_> {}

impl<'msg> ::protobuf::AsView for PrepareJobResponseView<'msg> {
  type Proxied = PrepareJobResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PrepareJobResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrepareJobResponseView<'msg> {
  fn into_view<'shorter>(self) -> PrepareJobResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PrepareJobResponse> for PrepareJobResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PrepareJobResponse {
    let mut dst = PrepareJobResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PrepareJobResponse> for PrepareJobResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PrepareJobResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PrepareJobResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PrepareJobResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PrepareJobResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PrepareJobResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrepareJobResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PrepareJobResponseMut<'msg> {
  type Message = PrepareJobResponse;
}

impl ::std::fmt::Debug for PrepareJobResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobResponse>> for PrepareJobResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrepareJobResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PrepareJobResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PrepareJobResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // preparation_id: optional string
  pub fn preparation_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_preparation_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // artifact_staging_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_artifact_staging_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_artifact_staging_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn artifact_staging_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.artifact_staging_endpoint(), self.has_artifact_staging_endpoint())
  }
  pub fn artifact_staging_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn artifact_staging_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
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
  pub fn set_artifact_staging_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // staging_session_token: optional string
  pub fn staging_session_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_session_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `PrepareJobResponseMut` does not perform any shared mutation.
unsafe impl Send for PrepareJobResponseMut<'_> {}

// SAFETY:
// - `PrepareJobResponseMut` does not perform any shared mutation.
unsafe impl Sync for PrepareJobResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for PrepareJobResponseMut<'msg> {
  type Proxied = PrepareJobResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PrepareJobResponse> {
    PrepareJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrepareJobResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PrepareJobResponse>
  where
      'msg: 'shorter {
    PrepareJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PrepareJobResponseMut<'msg> {
  type MutProxied = PrepareJobResponse;
  fn as_mut(&mut self) -> PrepareJobResponseMut<'msg> {
    PrepareJobResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PrepareJobResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PrepareJobResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PrepareJobResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PrepareJobResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PrepareJobResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PrepareJobResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // preparation_id: optional string
  pub fn preparation_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_preparation_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // artifact_staging_endpoint: optional message org.apache.beam.model.pipeline.v1.ApiServiceDescriptor
  pub fn has_artifact_staging_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_artifact_staging_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn artifact_staging_endpoint_opt(&self) -> ::protobuf::Optional<super::ApiServiceDescriptorView<'_>> {
        ::protobuf::Optional::new(self.artifact_staging_endpoint(), self.has_artifact_staging_endpoint())
  }
  pub fn artifact_staging_endpoint(&self) -> super::ApiServiceDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiServiceDescriptorView::default())
  }
  pub fn artifact_staging_endpoint_mut(&mut self) -> super::ApiServiceDescriptorMut<'_> {
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
  pub fn set_artifact_staging_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiServiceDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // staging_session_token: optional string
  pub fn staging_session_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_staging_session_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl PrepareJobResponse

impl ::std::ops::Drop for PrepareJobResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PrepareJobResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PrepareJobResponse {
  type Proxied = Self;
  fn as_view(&self) -> PrepareJobResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PrepareJobResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PrepareJobResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PrepareJobResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__PrepareJobResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__PrepareJobResponse_msg_init.0, &[<super::ApiServiceDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__PrepareJobResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PrepareJobResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PrepareJobResponse {
  type Msg = PrepareJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrepareJobResponse {
  type Msg = PrepareJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PrepareJobResponseMut<'_> {
  type Msg = PrepareJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrepareJobResponseMut<'_> {
  type Msg = PrepareJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrepareJobResponseView<'_> {
  type Msg = PrepareJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrepareJobResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PrepareJobResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__RunJobRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RunJobRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RunJobRequest>
}

impl ::protobuf::Message for RunJobRequest {}

impl ::std::default::Default for RunJobRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RunJobRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RunJobRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `RunJobRequestMut`.
unsafe impl Sync for RunJobRequest {}

// SAFETY:
// - `RunJobRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RunJobRequest {}

impl ::protobuf::Proxied for RunJobRequest {
  type View<'msg> = RunJobRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RunJobRequest {}

impl ::protobuf::MutProxied for RunJobRequest {
  type Mut<'msg> = RunJobRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RunJobRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RunJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RunJobRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RunJobRequestView<'msg> {
  type Message = RunJobRequest;
}

impl ::std::fmt::Debug for RunJobRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RunJobRequestView<'_> {
  fn default() -> RunJobRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RunJobRequest>> for RunJobRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RunJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RunJobRequestView<'msg> {

  pub fn to_owned(&self) -> RunJobRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // preparation_id: optional string
  pub fn preparation_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `RunJobRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for RunJobRequestView<'_> {}

// SAFETY:
// - `RunJobRequestView` is `Send` because while its alive a `RunJobRequestMut` cannot.
// - `RunJobRequestView` does not use thread-local data.
unsafe impl Send for RunJobRequestView<'_> {}

impl<'msg> ::protobuf::AsView for RunJobRequestView<'msg> {
  type Proxied = RunJobRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, RunJobRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RunJobRequestView<'msg> {
  fn into_view<'shorter>(self) -> RunJobRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RunJobRequest> for RunJobRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RunJobRequest {
    let mut dst = RunJobRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RunJobRequest> for RunJobRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RunJobRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for RunJobRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RunJobRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RunJobRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RunJobRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RunJobRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RunJobRequestMut<'msg> {
  type Message = RunJobRequest;
}

impl ::std::fmt::Debug for RunJobRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobRequest>> for RunJobRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RunJobRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> RunJobRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // preparation_id: optional string
  pub fn preparation_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_preparation_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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
// - `RunJobRequestMut` does not perform any shared mutation.
unsafe impl Send for RunJobRequestMut<'_> {}

// SAFETY:
// - `RunJobRequestMut` does not perform any shared mutation.
unsafe impl Sync for RunJobRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for RunJobRequestMut<'msg> {
  type Proxied = RunJobRequest;
  fn as_view(&self) -> ::protobuf::View<'_, RunJobRequest> {
    RunJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RunJobRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RunJobRequest>
  where
      'msg: 'shorter {
    RunJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RunJobRequestMut<'msg> {
  type MutProxied = RunJobRequest;
  fn as_mut(&mut self) -> RunJobRequestMut<'msg> {
    RunJobRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RunJobRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> RunJobRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RunJobRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RunJobRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RunJobRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RunJobRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // preparation_id: optional string
  pub fn preparation_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_preparation_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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

}  // impl RunJobRequest

impl ::std::ops::Drop for RunJobRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RunJobRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RunJobRequest {
  type Proxied = Self;
  fn as_view(&self) -> RunJobRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RunJobRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RunJobRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RunJobRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__RunJobRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__RunJobRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__RunJobRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RunJobRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RunJobRequest {
  type Msg = RunJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RunJobRequest {
  type Msg = RunJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RunJobRequestMut<'_> {
  type Msg = RunJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RunJobRequestMut<'_> {
  type Msg = RunJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RunJobRequestView<'_> {
  type Msg = RunJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RunJobRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__RunJobResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RunJobResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RunJobResponse>
}

impl ::protobuf::Message for RunJobResponse {}

impl ::std::default::Default for RunJobResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RunJobResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RunJobResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `RunJobResponseMut`.
unsafe impl Sync for RunJobResponse {}

// SAFETY:
// - `RunJobResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RunJobResponse {}

impl ::protobuf::Proxied for RunJobResponse {
  type View<'msg> = RunJobResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RunJobResponse {}

impl ::protobuf::MutProxied for RunJobResponse {
  type Mut<'msg> = RunJobResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RunJobResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RunJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RunJobResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RunJobResponseView<'msg> {
  type Message = RunJobResponse;
}

impl ::std::fmt::Debug for RunJobResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RunJobResponseView<'_> {
  fn default() -> RunJobResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RunJobResponse>> for RunJobResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RunJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RunJobResponseView<'msg> {

  pub fn to_owned(&self) -> RunJobResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `RunJobResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for RunJobResponseView<'_> {}

// SAFETY:
// - `RunJobResponseView` is `Send` because while its alive a `RunJobResponseMut` cannot.
// - `RunJobResponseView` does not use thread-local data.
unsafe impl Send for RunJobResponseView<'_> {}

impl<'msg> ::protobuf::AsView for RunJobResponseView<'msg> {
  type Proxied = RunJobResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, RunJobResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RunJobResponseView<'msg> {
  fn into_view<'shorter>(self) -> RunJobResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RunJobResponse> for RunJobResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RunJobResponse {
    let mut dst = RunJobResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RunJobResponse> for RunJobResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RunJobResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for RunJobResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RunJobResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RunJobResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RunJobResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RunJobResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RunJobResponseMut<'msg> {
  type Message = RunJobResponse;
}

impl ::std::fmt::Debug for RunJobResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobResponse>> for RunJobResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RunJobResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RunJobResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> RunJobResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `RunJobResponseMut` does not perform any shared mutation.
unsafe impl Send for RunJobResponseMut<'_> {}

// SAFETY:
// - `RunJobResponseMut` does not perform any shared mutation.
unsafe impl Sync for RunJobResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for RunJobResponseMut<'msg> {
  type Proxied = RunJobResponse;
  fn as_view(&self) -> ::protobuf::View<'_, RunJobResponse> {
    RunJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RunJobResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RunJobResponse>
  where
      'msg: 'shorter {
    RunJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RunJobResponseMut<'msg> {
  type MutProxied = RunJobResponse;
  fn as_mut(&mut self) -> RunJobResponseMut<'msg> {
    RunJobResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RunJobResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> RunJobResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RunJobResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RunJobResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RunJobResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RunJobResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl RunJobResponse

impl ::std::ops::Drop for RunJobResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RunJobResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RunJobResponse {
  type Proxied = Self;
  fn as_view(&self) -> RunJobResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RunJobResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RunJobResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RunJobResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__RunJobResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__RunJobResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__RunJobResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RunJobResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RunJobResponse {
  type Msg = RunJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RunJobResponse {
  type Msg = RunJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RunJobResponseMut<'_> {
  type Msg = RunJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RunJobResponseMut<'_> {
  type Msg = RunJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RunJobResponseView<'_> {
  type Msg = RunJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RunJobResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RunJobResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__CancelJobRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CancelJobRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CancelJobRequest>
}

impl ::protobuf::Message for CancelJobRequest {}

impl ::std::default::Default for CancelJobRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CancelJobRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CancelJobRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CancelJobRequestMut`.
unsafe impl Sync for CancelJobRequest {}

// SAFETY:
// - `CancelJobRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CancelJobRequest {}

impl ::protobuf::Proxied for CancelJobRequest {
  type View<'msg> = CancelJobRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CancelJobRequest {}

impl ::protobuf::MutProxied for CancelJobRequest {
  type Mut<'msg> = CancelJobRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CancelJobRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CancelJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CancelJobRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CancelJobRequestView<'msg> {
  type Message = CancelJobRequest;
}

impl ::std::fmt::Debug for CancelJobRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CancelJobRequestView<'_> {
  fn default() -> CancelJobRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CancelJobRequest>> for CancelJobRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CancelJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CancelJobRequestView<'msg> {

  pub fn to_owned(&self) -> CancelJobRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `CancelJobRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for CancelJobRequestView<'_> {}

// SAFETY:
// - `CancelJobRequestView` is `Send` because while its alive a `CancelJobRequestMut` cannot.
// - `CancelJobRequestView` does not use thread-local data.
unsafe impl Send for CancelJobRequestView<'_> {}

impl<'msg> ::protobuf::AsView for CancelJobRequestView<'msg> {
  type Proxied = CancelJobRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, CancelJobRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CancelJobRequestView<'msg> {
  fn into_view<'shorter>(self) -> CancelJobRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CancelJobRequest> for CancelJobRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CancelJobRequest {
    let mut dst = CancelJobRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CancelJobRequest> for CancelJobRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CancelJobRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for CancelJobRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CancelJobRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CancelJobRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CancelJobRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CancelJobRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CancelJobRequestMut<'msg> {
  type Message = CancelJobRequest;
}

impl ::std::fmt::Debug for CancelJobRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobRequest>> for CancelJobRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CancelJobRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> CancelJobRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `CancelJobRequestMut` does not perform any shared mutation.
unsafe impl Send for CancelJobRequestMut<'_> {}

// SAFETY:
// - `CancelJobRequestMut` does not perform any shared mutation.
unsafe impl Sync for CancelJobRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for CancelJobRequestMut<'msg> {
  type Proxied = CancelJobRequest;
  fn as_view(&self) -> ::protobuf::View<'_, CancelJobRequest> {
    CancelJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CancelJobRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CancelJobRequest>
  where
      'msg: 'shorter {
    CancelJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CancelJobRequestMut<'msg> {
  type MutProxied = CancelJobRequest;
  fn as_mut(&mut self) -> CancelJobRequestMut<'msg> {
    CancelJobRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CancelJobRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> CancelJobRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CancelJobRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CancelJobRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CancelJobRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CancelJobRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl CancelJobRequest

impl ::std::ops::Drop for CancelJobRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CancelJobRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CancelJobRequest {
  type Proxied = Self;
  fn as_view(&self) -> CancelJobRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CancelJobRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CancelJobRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CancelJobRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__CancelJobRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__CancelJobRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__CancelJobRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CancelJobRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CancelJobRequest {
  type Msg = CancelJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CancelJobRequest {
  type Msg = CancelJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CancelJobRequestMut<'_> {
  type Msg = CancelJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CancelJobRequestMut<'_> {
  type Msg = CancelJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CancelJobRequestView<'_> {
  type Msg = CancelJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CancelJobRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__CancelJobResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CancelJobResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CancelJobResponse>
}

impl ::protobuf::Message for CancelJobResponse {}

impl ::std::default::Default for CancelJobResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CancelJobResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CancelJobResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CancelJobResponseMut`.
unsafe impl Sync for CancelJobResponse {}

// SAFETY:
// - `CancelJobResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CancelJobResponse {}

impl ::protobuf::Proxied for CancelJobResponse {
  type View<'msg> = CancelJobResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CancelJobResponse {}

impl ::protobuf::MutProxied for CancelJobResponse {
  type Mut<'msg> = CancelJobResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CancelJobResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CancelJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CancelJobResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CancelJobResponseView<'msg> {
  type Message = CancelJobResponse;
}

impl ::std::fmt::Debug for CancelJobResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CancelJobResponseView<'_> {
  fn default() -> CancelJobResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CancelJobResponse>> for CancelJobResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CancelJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CancelJobResponseView<'msg> {

  pub fn to_owned(&self) -> CancelJobResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `CancelJobResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for CancelJobResponseView<'_> {}

// SAFETY:
// - `CancelJobResponseView` is `Send` because while its alive a `CancelJobResponseMut` cannot.
// - `CancelJobResponseView` does not use thread-local data.
unsafe impl Send for CancelJobResponseView<'_> {}

impl<'msg> ::protobuf::AsView for CancelJobResponseView<'msg> {
  type Proxied = CancelJobResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, CancelJobResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CancelJobResponseView<'msg> {
  fn into_view<'shorter>(self) -> CancelJobResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CancelJobResponse> for CancelJobResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CancelJobResponse {
    let mut dst = CancelJobResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CancelJobResponse> for CancelJobResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CancelJobResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for CancelJobResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CancelJobResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CancelJobResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CancelJobResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CancelJobResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CancelJobResponseMut<'msg> {
  type Message = CancelJobResponse;
}

impl ::std::fmt::Debug for CancelJobResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobResponse>> for CancelJobResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CancelJobResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CancelJobResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> CancelJobResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
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

}

// SAFETY:
// - `CancelJobResponseMut` does not perform any shared mutation.
unsafe impl Send for CancelJobResponseMut<'_> {}

// SAFETY:
// - `CancelJobResponseMut` does not perform any shared mutation.
unsafe impl Sync for CancelJobResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for CancelJobResponseMut<'msg> {
  type Proxied = CancelJobResponse;
  fn as_view(&self) -> ::protobuf::View<'_, CancelJobResponse> {
    CancelJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CancelJobResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CancelJobResponse>
  where
      'msg: 'shorter {
    CancelJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CancelJobResponseMut<'msg> {
  type MutProxied = CancelJobResponse;
  fn as_mut(&mut self) -> CancelJobResponseMut<'msg> {
    CancelJobResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CancelJobResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> CancelJobResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CancelJobResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CancelJobResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CancelJobResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CancelJobResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
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

}  // impl CancelJobResponse

impl ::std::ops::Drop for CancelJobResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CancelJobResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CancelJobResponse {
  type Proxied = Self;
  fn as_view(&self) -> CancelJobResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CancelJobResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CancelJobResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CancelJobResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__CancelJobResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__CancelJobResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__CancelJobResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CancelJobResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CancelJobResponse {
  type Msg = CancelJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CancelJobResponse {
  type Msg = CancelJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CancelJobResponseMut<'_> {
  type Msg = CancelJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CancelJobResponseMut<'_> {
  type Msg = CancelJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CancelJobResponseView<'_> {
  type Msg = CancelJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CancelJobResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CancelJobResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__DrainJobRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DrainJobRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DrainJobRequest>
}

impl ::protobuf::Message for DrainJobRequest {}

impl ::std::default::Default for DrainJobRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DrainJobRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DrainJobRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DrainJobRequestMut`.
unsafe impl Sync for DrainJobRequest {}

// SAFETY:
// - `DrainJobRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DrainJobRequest {}

impl ::protobuf::Proxied for DrainJobRequest {
  type View<'msg> = DrainJobRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DrainJobRequest {}

impl ::protobuf::MutProxied for DrainJobRequest {
  type Mut<'msg> = DrainJobRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DrainJobRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DrainJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DrainJobRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DrainJobRequestView<'msg> {
  type Message = DrainJobRequest;
}

impl ::std::fmt::Debug for DrainJobRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DrainJobRequestView<'_> {
  fn default() -> DrainJobRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DrainJobRequest>> for DrainJobRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DrainJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DrainJobRequestView<'msg> {

  pub fn to_owned(&self) -> DrainJobRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `DrainJobRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DrainJobRequestView<'_> {}

// SAFETY:
// - `DrainJobRequestView` is `Send` because while its alive a `DrainJobRequestMut` cannot.
// - `DrainJobRequestView` does not use thread-local data.
unsafe impl Send for DrainJobRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DrainJobRequestView<'msg> {
  type Proxied = DrainJobRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DrainJobRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DrainJobRequestView<'msg> {
  fn into_view<'shorter>(self) -> DrainJobRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DrainJobRequest> for DrainJobRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DrainJobRequest {
    let mut dst = DrainJobRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DrainJobRequest> for DrainJobRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DrainJobRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DrainJobRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DrainJobRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DrainJobRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DrainJobRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DrainJobRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DrainJobRequestMut<'msg> {
  type Message = DrainJobRequest;
}

impl ::std::fmt::Debug for DrainJobRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobRequest>> for DrainJobRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DrainJobRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DrainJobRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `DrainJobRequestMut` does not perform any shared mutation.
unsafe impl Send for DrainJobRequestMut<'_> {}

// SAFETY:
// - `DrainJobRequestMut` does not perform any shared mutation.
unsafe impl Sync for DrainJobRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DrainJobRequestMut<'msg> {
  type Proxied = DrainJobRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DrainJobRequest> {
    DrainJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DrainJobRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DrainJobRequest>
  where
      'msg: 'shorter {
    DrainJobRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DrainJobRequestMut<'msg> {
  type MutProxied = DrainJobRequest;
  fn as_mut(&mut self) -> DrainJobRequestMut<'msg> {
    DrainJobRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DrainJobRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DrainJobRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DrainJobRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DrainJobRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DrainJobRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DrainJobRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl DrainJobRequest

impl ::std::ops::Drop for DrainJobRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DrainJobRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DrainJobRequest {
  type Proxied = Self;
  fn as_view(&self) -> DrainJobRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DrainJobRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DrainJobRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DrainJobRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__DrainJobRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__DrainJobRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__DrainJobRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DrainJobRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DrainJobRequest {
  type Msg = DrainJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DrainJobRequest {
  type Msg = DrainJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DrainJobRequestMut<'_> {
  type Msg = DrainJobRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DrainJobRequestMut<'_> {
  type Msg = DrainJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DrainJobRequestView<'_> {
  type Msg = DrainJobRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DrainJobRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__DrainJobResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DrainJobResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DrainJobResponse>
}

impl ::protobuf::Message for DrainJobResponse {}

impl ::std::default::Default for DrainJobResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DrainJobResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DrainJobResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DrainJobResponseMut`.
unsafe impl Sync for DrainJobResponse {}

// SAFETY:
// - `DrainJobResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DrainJobResponse {}

impl ::protobuf::Proxied for DrainJobResponse {
  type View<'msg> = DrainJobResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DrainJobResponse {}

impl ::protobuf::MutProxied for DrainJobResponse {
  type Mut<'msg> = DrainJobResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DrainJobResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DrainJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DrainJobResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DrainJobResponseView<'msg> {
  type Message = DrainJobResponse;
}

impl ::std::fmt::Debug for DrainJobResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DrainJobResponseView<'_> {
  fn default() -> DrainJobResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DrainJobResponse>> for DrainJobResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DrainJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DrainJobResponseView<'msg> {

  pub fn to_owned(&self) -> DrainJobResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DrainJobResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for DrainJobResponseView<'_> {}

// SAFETY:
// - `DrainJobResponseView` is `Send` because while its alive a `DrainJobResponseMut` cannot.
// - `DrainJobResponseView` does not use thread-local data.
unsafe impl Send for DrainJobResponseView<'_> {}

impl<'msg> ::protobuf::AsView for DrainJobResponseView<'msg> {
  type Proxied = DrainJobResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DrainJobResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DrainJobResponseView<'msg> {
  fn into_view<'shorter>(self) -> DrainJobResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DrainJobResponse> for DrainJobResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DrainJobResponse {
    let mut dst = DrainJobResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DrainJobResponse> for DrainJobResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DrainJobResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DrainJobResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DrainJobResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DrainJobResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DrainJobResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DrainJobResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DrainJobResponseMut<'msg> {
  type Message = DrainJobResponse;
}

impl ::std::fmt::Debug for DrainJobResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobResponse>> for DrainJobResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DrainJobResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DrainJobResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> DrainJobResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
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

}

// SAFETY:
// - `DrainJobResponseMut` does not perform any shared mutation.
unsafe impl Send for DrainJobResponseMut<'_> {}

// SAFETY:
// - `DrainJobResponseMut` does not perform any shared mutation.
unsafe impl Sync for DrainJobResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for DrainJobResponseMut<'msg> {
  type Proxied = DrainJobResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DrainJobResponse> {
    DrainJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DrainJobResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DrainJobResponse>
  where
      'msg: 'shorter {
    DrainJobResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DrainJobResponseMut<'msg> {
  type MutProxied = DrainJobResponse;
  fn as_mut(&mut self) -> DrainJobResponseMut<'msg> {
    DrainJobResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DrainJobResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DrainJobResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DrainJobResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DrainJobResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DrainJobResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DrainJobResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
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

}  // impl DrainJobResponse

impl ::std::ops::Drop for DrainJobResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DrainJobResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DrainJobResponse {
  type Proxied = Self;
  fn as_view(&self) -> DrainJobResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DrainJobResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DrainJobResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DrainJobResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__DrainJobResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__DrainJobResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__DrainJobResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DrainJobResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DrainJobResponse {
  type Msg = DrainJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DrainJobResponse {
  type Msg = DrainJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DrainJobResponseMut<'_> {
  type Msg = DrainJobResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DrainJobResponseMut<'_> {
  type Msg = DrainJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DrainJobResponseView<'_> {
  type Msg = DrainJobResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DrainJobResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DrainJobResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__JobInfo_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JobInfo {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JobInfo>
}

impl ::protobuf::Message for JobInfo {}

impl ::std::default::Default for JobInfo {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JobInfo {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JobInfo` is `Sync` because it does not implement interior mutability.
//    Neither does `JobInfoMut`.
unsafe impl Sync for JobInfo {}

// SAFETY:
// - `JobInfo` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JobInfo {}

impl ::protobuf::Proxied for JobInfo {
  type View<'msg> = JobInfoView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JobInfo {}

impl ::protobuf::MutProxied for JobInfo {
  type Mut<'msg> = JobInfoMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JobInfoView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobInfoView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JobInfoView<'msg> {
  type Message = JobInfo;
}

impl ::std::fmt::Debug for JobInfoView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JobInfoView<'_> {
  fn default() -> JobInfoView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JobInfo>> for JobInfoView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobInfoView<'msg> {

  pub fn to_owned(&self) -> JobInfo {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // job_name: optional string
  pub fn job_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn pipeline_options_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'msg>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `JobInfoView` is `Sync` because it does not support mutation.
unsafe impl Sync for JobInfoView<'_> {}

// SAFETY:
// - `JobInfoView` is `Send` because while its alive a `JobInfoMut` cannot.
// - `JobInfoView` does not use thread-local data.
unsafe impl Send for JobInfoView<'_> {}

impl<'msg> ::protobuf::AsView for JobInfoView<'msg> {
  type Proxied = JobInfo;
  fn as_view(&self) -> ::protobuf::View<'msg, JobInfo> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobInfoView<'msg> {
  fn into_view<'shorter>(self) -> JobInfoView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JobInfo> for JobInfoView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobInfo {
    let mut dst = JobInfo::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JobInfo> for JobInfoMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobInfo {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JobInfo {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobInfoView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobInfoMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JobInfoMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobInfoMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JobInfoMut<'msg> {
  type Message = JobInfo;
}

impl ::std::fmt::Debug for JobInfoMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JobInfo>> for JobInfoMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobInfoMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JobInfo> {
    self.inner
  }

  pub fn to_owned(&self) -> JobInfo {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // job_name: optional string
  pub fn job_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `JobInfoMut` does not perform any shared mutation.
unsafe impl Send for JobInfoMut<'_> {}

// SAFETY:
// - `JobInfoMut` does not perform any shared mutation.
unsafe impl Sync for JobInfoMut<'_> {}

impl<'msg> ::protobuf::AsView for JobInfoMut<'msg> {
  type Proxied = JobInfo;
  fn as_view(&self) -> ::protobuf::View<'_, JobInfo> {
    JobInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobInfoMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JobInfo>
  where
      'msg: 'shorter {
    JobInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JobInfoMut<'msg> {
  type MutProxied = JobInfo;
  fn as_mut(&mut self) -> JobInfoMut<'msg> {
    JobInfoMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JobInfoMut<'msg> {
  fn into_mut<'shorter>(self) -> JobInfoMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JobInfo {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JobInfo> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JobInfoView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JobInfoMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // job_name: optional string
  pub fn job_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}  // impl JobInfo

impl ::std::ops::Drop for JobInfo {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JobInfo {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JobInfo {
  type Proxied = Self;
  fn as_view(&self) -> JobInfoView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JobInfo {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JobInfoMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JobInfo {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__JobInfo_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__JobInfo_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__JobInfo_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobInfo {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobInfo {
  type Msg = JobInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobInfo {
  type Msg = JobInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobInfoMut<'_> {
  type Msg = JobInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobInfoMut<'_> {
  type Msg = JobInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobInfoView<'_> {
  type Msg = JobInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobInfo> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobInfoMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetJobsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetJobsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetJobsRequest>
}

impl ::protobuf::Message for GetJobsRequest {}

impl ::std::default::Default for GetJobsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetJobsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetJobsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetJobsRequestMut`.
unsafe impl Sync for GetJobsRequest {}

// SAFETY:
// - `GetJobsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetJobsRequest {}

impl ::protobuf::Proxied for GetJobsRequest {
  type View<'msg> = GetJobsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetJobsRequest {}

impl ::protobuf::MutProxied for GetJobsRequest {
  type Mut<'msg> = GetJobsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetJobsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetJobsRequestView<'msg> {
  type Message = GetJobsRequest;
}

impl ::std::fmt::Debug for GetJobsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetJobsRequestView<'_> {
  fn default() -> GetJobsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobsRequest>> for GetJobsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobsRequestView<'msg> {

  pub fn to_owned(&self) -> GetJobsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetJobsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetJobsRequestView<'_> {}

// SAFETY:
// - `GetJobsRequestView` is `Send` because while its alive a `GetJobsRequestMut` cannot.
// - `GetJobsRequestView` does not use thread-local data.
unsafe impl Send for GetJobsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetJobsRequestView<'msg> {
  type Proxied = GetJobsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetJobsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobsRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetJobsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobsRequest> for GetJobsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobsRequest {
    let mut dst = GetJobsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobsRequest> for GetJobsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetJobsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetJobsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetJobsRequestMut<'msg> {
  type Message = GetJobsRequest;
}

impl ::std::fmt::Debug for GetJobsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsRequest>> for GetJobsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetJobsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `GetJobsRequestMut` does not perform any shared mutation.
unsafe impl Send for GetJobsRequestMut<'_> {}

// SAFETY:
// - `GetJobsRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetJobsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetJobsRequestMut<'msg> {
  type Proxied = GetJobsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetJobsRequest> {
    GetJobsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetJobsRequest>
  where
      'msg: 'shorter {
    GetJobsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetJobsRequestMut<'msg> {
  type MutProxied = GetJobsRequest;
  fn as_mut(&mut self) -> GetJobsRequestMut<'msg> {
    GetJobsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetJobsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetJobsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetJobsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetJobsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetJobsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetJobsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl GetJobsRequest

impl ::std::ops::Drop for GetJobsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetJobsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetJobsRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetJobsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetJobsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetJobsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetJobsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetJobsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetJobsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetJobsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobsRequest {
  type Msg = GetJobsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobsRequest {
  type Msg = GetJobsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobsRequestMut<'_> {
  type Msg = GetJobsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobsRequestMut<'_> {
  type Msg = GetJobsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobsRequestView<'_> {
  type Msg = GetJobsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetJobsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetJobsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetJobsResponse>
}

impl ::protobuf::Message for GetJobsResponse {}

impl ::std::default::Default for GetJobsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetJobsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetJobsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetJobsResponseMut`.
unsafe impl Sync for GetJobsResponse {}

// SAFETY:
// - `GetJobsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetJobsResponse {}

impl ::protobuf::Proxied for GetJobsResponse {
  type View<'msg> = GetJobsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetJobsResponse {}

impl ::protobuf::MutProxied for GetJobsResponse {
  type Mut<'msg> = GetJobsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetJobsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetJobsResponseView<'msg> {
  type Message = GetJobsResponse;
}

impl ::std::fmt::Debug for GetJobsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetJobsResponseView<'_> {
  fn default() -> GetJobsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobsResponse>> for GetJobsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobsResponseView<'msg> {

  pub fn to_owned(&self) -> GetJobsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_info: repeated message org.apache.beam.model.job_management.v1.JobInfo
  pub fn job_info(self) -> ::protobuf::RepeatedView<'msg, super::JobInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::JobInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `GetJobsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetJobsResponseView<'_> {}

// SAFETY:
// - `GetJobsResponseView` is `Send` because while its alive a `GetJobsResponseMut` cannot.
// - `GetJobsResponseView` does not use thread-local data.
unsafe impl Send for GetJobsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GetJobsResponseView<'msg> {
  type Proxied = GetJobsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetJobsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobsResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetJobsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobsResponse> for GetJobsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobsResponse {
    let mut dst = GetJobsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobsResponse> for GetJobsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetJobsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetJobsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetJobsResponseMut<'msg> {
  type Message = GetJobsResponse;
}

impl ::std::fmt::Debug for GetJobsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsResponse>> for GetJobsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GetJobsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_info: repeated message org.apache.beam.model.job_management.v1.JobInfo
  pub fn job_info(&self) -> ::protobuf::RepeatedView<'_, super::JobInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::JobInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn job_info_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::JobInfo> {
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
  pub fn set_job_info(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::JobInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `GetJobsResponseMut` does not perform any shared mutation.
unsafe impl Send for GetJobsResponseMut<'_> {}

// SAFETY:
// - `GetJobsResponseMut` does not perform any shared mutation.
unsafe impl Sync for GetJobsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GetJobsResponseMut<'msg> {
  type Proxied = GetJobsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetJobsResponse> {
    GetJobsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetJobsResponse>
  where
      'msg: 'shorter {
    GetJobsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetJobsResponseMut<'msg> {
  type MutProxied = GetJobsResponse;
  fn as_mut(&mut self) -> GetJobsResponseMut<'msg> {
    GetJobsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetJobsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetJobsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetJobsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetJobsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetJobsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetJobsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_info: repeated message org.apache.beam.model.job_management.v1.JobInfo
  pub fn job_info(&self) -> ::protobuf::RepeatedView<'_, super::JobInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::JobInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn job_info_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::JobInfo> {
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
  pub fn set_job_info(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::JobInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl GetJobsResponse

impl ::std::ops::Drop for GetJobsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetJobsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetJobsResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetJobsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetJobsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetJobsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetJobsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetJobsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetJobsResponse_msg_init.0, &[<super::JobInfo as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetJobsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobsResponse {
  type Msg = GetJobsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobsResponse {
  type Msg = GetJobsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobsResponseMut<'_> {
  type Msg = GetJobsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobsResponseMut<'_> {
  type Msg = GetJobsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobsResponseView<'_> {
  type Msg = GetJobsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetJobStateRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetJobStateRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetJobStateRequest>
}

impl ::protobuf::Message for GetJobStateRequest {}

impl ::std::default::Default for GetJobStateRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetJobStateRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetJobStateRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetJobStateRequestMut`.
unsafe impl Sync for GetJobStateRequest {}

// SAFETY:
// - `GetJobStateRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetJobStateRequest {}

impl ::protobuf::Proxied for GetJobStateRequest {
  type View<'msg> = GetJobStateRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetJobStateRequest {}

impl ::protobuf::MutProxied for GetJobStateRequest {
  type Mut<'msg> = GetJobStateRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetJobStateRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobStateRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobStateRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetJobStateRequestView<'msg> {
  type Message = GetJobStateRequest;
}

impl ::std::fmt::Debug for GetJobStateRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetJobStateRequestView<'_> {
  fn default() -> GetJobStateRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobStateRequest>> for GetJobStateRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobStateRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobStateRequestView<'msg> {

  pub fn to_owned(&self) -> GetJobStateRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `GetJobStateRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetJobStateRequestView<'_> {}

// SAFETY:
// - `GetJobStateRequestView` is `Send` because while its alive a `GetJobStateRequestMut` cannot.
// - `GetJobStateRequestView` does not use thread-local data.
unsafe impl Send for GetJobStateRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetJobStateRequestView<'msg> {
  type Proxied = GetJobStateRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetJobStateRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobStateRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetJobStateRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobStateRequest> for GetJobStateRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobStateRequest {
    let mut dst = GetJobStateRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobStateRequest> for GetJobStateRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobStateRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetJobStateRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobStateRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobStateRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetJobStateRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobStateRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobStateRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetJobStateRequestMut<'msg> {
  type Message = GetJobStateRequest;
}

impl ::std::fmt::Debug for GetJobStateRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobStateRequest>> for GetJobStateRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobStateRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobStateRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobStateRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetJobStateRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetJobStateRequestMut` does not perform any shared mutation.
unsafe impl Send for GetJobStateRequestMut<'_> {}

// SAFETY:
// - `GetJobStateRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetJobStateRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetJobStateRequestMut<'msg> {
  type Proxied = GetJobStateRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetJobStateRequest> {
    GetJobStateRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobStateRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetJobStateRequest>
  where
      'msg: 'shorter {
    GetJobStateRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetJobStateRequestMut<'msg> {
  type MutProxied = GetJobStateRequest;
  fn as_mut(&mut self) -> GetJobStateRequestMut<'msg> {
    GetJobStateRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetJobStateRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetJobStateRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetJobStateRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetJobStateRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetJobStateRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetJobStateRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetJobStateRequest

impl ::std::ops::Drop for GetJobStateRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetJobStateRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetJobStateRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetJobStateRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetJobStateRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetJobStateRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetJobStateRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetJobStateRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetJobStateRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetJobStateRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobStateRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobStateRequest {
  type Msg = GetJobStateRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobStateRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobStateRequest {
  type Msg = GetJobStateRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobStateRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobStateRequestMut<'_> {
  type Msg = GetJobStateRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobStateRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobStateRequestMut<'_> {
  type Msg = GetJobStateRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobStateRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobStateRequestView<'_> {
  type Msg = GetJobStateRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobStateRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobStateRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__JobStateEvent_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JobStateEvent {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JobStateEvent>
}

impl ::protobuf::Message for JobStateEvent {}

impl ::std::default::Default for JobStateEvent {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JobStateEvent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JobStateEvent` is `Sync` because it does not implement interior mutability.
//    Neither does `JobStateEventMut`.
unsafe impl Sync for JobStateEvent {}

// SAFETY:
// - `JobStateEvent` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JobStateEvent {}

impl ::protobuf::Proxied for JobStateEvent {
  type View<'msg> = JobStateEventView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JobStateEvent {}

impl ::protobuf::MutProxied for JobStateEvent {
  type Mut<'msg> = JobStateEventMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JobStateEventView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobStateEvent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobStateEventView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JobStateEventView<'msg> {
  type Message = JobStateEvent;
}

impl ::std::fmt::Debug for JobStateEventView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JobStateEventView<'_> {
  fn default() -> JobStateEventView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JobStateEvent>> for JobStateEventView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobStateEvent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobStateEventView<'msg> {

  pub fn to_owned(&self) -> JobStateEvent {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // timestamp: optional message google.protobuf.Timestamp
  pub fn has_timestamp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn timestamp_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.timestamp(), self.has_timestamp())
  }
  pub fn timestamp(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `JobStateEventView` is `Sync` because it does not support mutation.
unsafe impl Sync for JobStateEventView<'_> {}

// SAFETY:
// - `JobStateEventView` is `Send` because while its alive a `JobStateEventMut` cannot.
// - `JobStateEventView` does not use thread-local data.
unsafe impl Send for JobStateEventView<'_> {}

impl<'msg> ::protobuf::AsView for JobStateEventView<'msg> {
  type Proxied = JobStateEvent;
  fn as_view(&self) -> ::protobuf::View<'msg, JobStateEvent> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobStateEventView<'msg> {
  fn into_view<'shorter>(self) -> JobStateEventView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JobStateEvent> for JobStateEventView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobStateEvent {
    let mut dst = JobStateEvent::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JobStateEvent> for JobStateEventMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobStateEvent {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JobStateEvent {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobStateEventView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobStateEventMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JobStateEventMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobStateEvent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobStateEventMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JobStateEventMut<'msg> {
  type Message = JobStateEvent;
}

impl ::std::fmt::Debug for JobStateEventMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JobStateEvent>> for JobStateEventMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobStateEvent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobStateEventMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JobStateEvent> {
    self.inner
  }

  pub fn to_owned(&self) -> JobStateEvent {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
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

  // timestamp: optional message google.protobuf.Timestamp
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.timestamp(), self.has_timestamp())
  }
  pub fn timestamp(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn timestamp_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_timestamp(&mut self,
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
// - `JobStateEventMut` does not perform any shared mutation.
unsafe impl Send for JobStateEventMut<'_> {}

// SAFETY:
// - `JobStateEventMut` does not perform any shared mutation.
unsafe impl Sync for JobStateEventMut<'_> {}

impl<'msg> ::protobuf::AsView for JobStateEventMut<'msg> {
  type Proxied = JobStateEvent;
  fn as_view(&self) -> ::protobuf::View<'_, JobStateEvent> {
    JobStateEventView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobStateEventMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JobStateEvent>
  where
      'msg: 'shorter {
    JobStateEventView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JobStateEventMut<'msg> {
  type MutProxied = JobStateEvent;
  fn as_mut(&mut self) -> JobStateEventMut<'msg> {
    JobStateEventMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JobStateEventMut<'msg> {
  fn into_mut<'shorter>(self) -> JobStateEventMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JobStateEvent {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JobStateEvent> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JobStateEventView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JobStateEventMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // state: optional enum org.apache.beam.model.job_management.v1.JobState.Enum
  pub fn state(&self) -> super::job_state::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::job_state::Enum::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::job_state::Enum) {
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

  // timestamp: optional message google.protobuf.Timestamp
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.timestamp(), self.has_timestamp())
  }
  pub fn timestamp(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn timestamp_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_timestamp(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl JobStateEvent

impl ::std::ops::Drop for JobStateEvent {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JobStateEvent {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JobStateEvent {
  type Proxied = Self;
  fn as_view(&self) -> JobStateEventView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JobStateEvent {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JobStateEventMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JobStateEvent {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__JobStateEvent_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__JobStateEvent_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__JobStateEvent_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobStateEvent {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobStateEvent {
  type Msg = JobStateEvent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobStateEvent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobStateEvent {
  type Msg = JobStateEvent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobStateEvent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobStateEventMut<'_> {
  type Msg = JobStateEvent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobStateEvent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobStateEventMut<'_> {
  type Msg = JobStateEvent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobStateEvent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobStateEventView<'_> {
  type Msg = JobStateEvent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobStateEvent> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobStateEventMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetJobPipelineRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetJobPipelineRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetJobPipelineRequest>
}

impl ::protobuf::Message for GetJobPipelineRequest {}

impl ::std::default::Default for GetJobPipelineRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetJobPipelineRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetJobPipelineRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetJobPipelineRequestMut`.
unsafe impl Sync for GetJobPipelineRequest {}

// SAFETY:
// - `GetJobPipelineRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetJobPipelineRequest {}

impl ::protobuf::Proxied for GetJobPipelineRequest {
  type View<'msg> = GetJobPipelineRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetJobPipelineRequest {}

impl ::protobuf::MutProxied for GetJobPipelineRequest {
  type Mut<'msg> = GetJobPipelineRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetJobPipelineRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobPipelineRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobPipelineRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetJobPipelineRequestView<'msg> {
  type Message = GetJobPipelineRequest;
}

impl ::std::fmt::Debug for GetJobPipelineRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetJobPipelineRequestView<'_> {
  fn default() -> GetJobPipelineRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobPipelineRequest>> for GetJobPipelineRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobPipelineRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobPipelineRequestView<'msg> {

  pub fn to_owned(&self) -> GetJobPipelineRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `GetJobPipelineRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetJobPipelineRequestView<'_> {}

// SAFETY:
// - `GetJobPipelineRequestView` is `Send` because while its alive a `GetJobPipelineRequestMut` cannot.
// - `GetJobPipelineRequestView` does not use thread-local data.
unsafe impl Send for GetJobPipelineRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetJobPipelineRequestView<'msg> {
  type Proxied = GetJobPipelineRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetJobPipelineRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobPipelineRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetJobPipelineRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobPipelineRequest> for GetJobPipelineRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobPipelineRequest {
    let mut dst = GetJobPipelineRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobPipelineRequest> for GetJobPipelineRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobPipelineRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetJobPipelineRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobPipelineRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobPipelineRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetJobPipelineRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobPipelineRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetJobPipelineRequestMut<'msg> {
  type Message = GetJobPipelineRequest;
}

impl ::std::fmt::Debug for GetJobPipelineRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineRequest>> for GetJobPipelineRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobPipelineRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetJobPipelineRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetJobPipelineRequestMut` does not perform any shared mutation.
unsafe impl Send for GetJobPipelineRequestMut<'_> {}

// SAFETY:
// - `GetJobPipelineRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetJobPipelineRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetJobPipelineRequestMut<'msg> {
  type Proxied = GetJobPipelineRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetJobPipelineRequest> {
    GetJobPipelineRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobPipelineRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetJobPipelineRequest>
  where
      'msg: 'shorter {
    GetJobPipelineRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetJobPipelineRequestMut<'msg> {
  type MutProxied = GetJobPipelineRequest;
  fn as_mut(&mut self) -> GetJobPipelineRequestMut<'msg> {
    GetJobPipelineRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetJobPipelineRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetJobPipelineRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetJobPipelineRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetJobPipelineRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetJobPipelineRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetJobPipelineRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetJobPipelineRequest

impl ::std::ops::Drop for GetJobPipelineRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetJobPipelineRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetJobPipelineRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetJobPipelineRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetJobPipelineRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetJobPipelineRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetJobPipelineRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetJobPipelineRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetJobPipelineRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetJobPipelineRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobPipelineRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobPipelineRequest {
  type Msg = GetJobPipelineRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobPipelineRequest {
  type Msg = GetJobPipelineRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobPipelineRequestMut<'_> {
  type Msg = GetJobPipelineRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobPipelineRequestMut<'_> {
  type Msg = GetJobPipelineRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobPipelineRequestView<'_> {
  type Msg = GetJobPipelineRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobPipelineRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetJobPipelineResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetJobPipelineResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetJobPipelineResponse>
}

impl ::protobuf::Message for GetJobPipelineResponse {}

impl ::std::default::Default for GetJobPipelineResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetJobPipelineResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetJobPipelineResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetJobPipelineResponseMut`.
unsafe impl Sync for GetJobPipelineResponse {}

// SAFETY:
// - `GetJobPipelineResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetJobPipelineResponse {}

impl ::protobuf::Proxied for GetJobPipelineResponse {
  type View<'msg> = GetJobPipelineResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetJobPipelineResponse {}

impl ::protobuf::MutProxied for GetJobPipelineResponse {
  type Mut<'msg> = GetJobPipelineResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetJobPipelineResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobPipelineResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobPipelineResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetJobPipelineResponseView<'msg> {
  type Message = GetJobPipelineResponse;
}

impl ::std::fmt::Debug for GetJobPipelineResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetJobPipelineResponseView<'_> {
  fn default() -> GetJobPipelineResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobPipelineResponse>> for GetJobPipelineResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobPipelineResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobPipelineResponseView<'msg> {

  pub fn to_owned(&self) -> GetJobPipelineResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // pipeline: optional message org.apache.beam.model.pipeline.v1.Pipeline
  pub fn has_pipeline(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn pipeline_opt(self) -> ::protobuf::Optional<super::PipelineView<'msg>> {
        ::protobuf::Optional::new(self.pipeline(), self.has_pipeline())
  }
  pub fn pipeline(self) -> super::PipelineView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipelineView::default())
  }

}

// SAFETY:
// - `GetJobPipelineResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetJobPipelineResponseView<'_> {}

// SAFETY:
// - `GetJobPipelineResponseView` is `Send` because while its alive a `GetJobPipelineResponseMut` cannot.
// - `GetJobPipelineResponseView` does not use thread-local data.
unsafe impl Send for GetJobPipelineResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GetJobPipelineResponseView<'msg> {
  type Proxied = GetJobPipelineResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetJobPipelineResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobPipelineResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetJobPipelineResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobPipelineResponse> for GetJobPipelineResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobPipelineResponse {
    let mut dst = GetJobPipelineResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobPipelineResponse> for GetJobPipelineResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobPipelineResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetJobPipelineResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobPipelineResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobPipelineResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetJobPipelineResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobPipelineResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetJobPipelineResponseMut<'msg> {
  type Message = GetJobPipelineResponse;
}

impl ::std::fmt::Debug for GetJobPipelineResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineResponse>> for GetJobPipelineResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobPipelineResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobPipelineResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GetJobPipelineResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // pipeline: optional message org.apache.beam.model.pipeline.v1.Pipeline
  pub fn has_pipeline(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pipeline(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pipeline_opt(&self) -> ::protobuf::Optional<super::PipelineView<'_>> {
        ::protobuf::Optional::new(self.pipeline(), self.has_pipeline())
  }
  pub fn pipeline(&self) -> super::PipelineView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipelineView::default())
  }
  pub fn pipeline_mut(&mut self) -> super::PipelineMut<'_> {
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
  pub fn set_pipeline(&mut self,
    val: impl ::protobuf::IntoProxied<super::Pipeline>) {

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
// - `GetJobPipelineResponseMut` does not perform any shared mutation.
unsafe impl Send for GetJobPipelineResponseMut<'_> {}

// SAFETY:
// - `GetJobPipelineResponseMut` does not perform any shared mutation.
unsafe impl Sync for GetJobPipelineResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GetJobPipelineResponseMut<'msg> {
  type Proxied = GetJobPipelineResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetJobPipelineResponse> {
    GetJobPipelineResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobPipelineResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetJobPipelineResponse>
  where
      'msg: 'shorter {
    GetJobPipelineResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetJobPipelineResponseMut<'msg> {
  type MutProxied = GetJobPipelineResponse;
  fn as_mut(&mut self) -> GetJobPipelineResponseMut<'msg> {
    GetJobPipelineResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetJobPipelineResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetJobPipelineResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetJobPipelineResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetJobPipelineResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetJobPipelineResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetJobPipelineResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // pipeline: optional message org.apache.beam.model.pipeline.v1.Pipeline
  pub fn has_pipeline(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pipeline(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pipeline_opt(&self) -> ::protobuf::Optional<super::PipelineView<'_>> {
        ::protobuf::Optional::new(self.pipeline(), self.has_pipeline())
  }
  pub fn pipeline(&self) -> super::PipelineView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipelineView::default())
  }
  pub fn pipeline_mut(&mut self) -> super::PipelineMut<'_> {
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
  pub fn set_pipeline(&mut self,
    val: impl ::protobuf::IntoProxied<super::Pipeline>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GetJobPipelineResponse

impl ::std::ops::Drop for GetJobPipelineResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetJobPipelineResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetJobPipelineResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetJobPipelineResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetJobPipelineResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetJobPipelineResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetJobPipelineResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetJobPipelineResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetJobPipelineResponse_msg_init.0, &[<super::Pipeline as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetJobPipelineResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobPipelineResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobPipelineResponse {
  type Msg = GetJobPipelineResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobPipelineResponse {
  type Msg = GetJobPipelineResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobPipelineResponseMut<'_> {
  type Msg = GetJobPipelineResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobPipelineResponseMut<'_> {
  type Msg = GetJobPipelineResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobPipelineResponseView<'_> {
  type Msg = GetJobPipelineResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobPipelineResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobPipelineResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__JobMessagesRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JobMessagesRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JobMessagesRequest>
}

impl ::protobuf::Message for JobMessagesRequest {}

impl ::std::default::Default for JobMessagesRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JobMessagesRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JobMessagesRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `JobMessagesRequestMut`.
unsafe impl Sync for JobMessagesRequest {}

// SAFETY:
// - `JobMessagesRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JobMessagesRequest {}

impl ::protobuf::Proxied for JobMessagesRequest {
  type View<'msg> = JobMessagesRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JobMessagesRequest {}

impl ::protobuf::MutProxied for JobMessagesRequest {
  type Mut<'msg> = JobMessagesRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JobMessagesRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessagesRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobMessagesRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JobMessagesRequestView<'msg> {
  type Message = JobMessagesRequest;
}

impl ::std::fmt::Debug for JobMessagesRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JobMessagesRequestView<'_> {
  fn default() -> JobMessagesRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessagesRequest>> for JobMessagesRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessagesRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobMessagesRequestView<'msg> {

  pub fn to_owned(&self) -> JobMessagesRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `JobMessagesRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for JobMessagesRequestView<'_> {}

// SAFETY:
// - `JobMessagesRequestView` is `Send` because while its alive a `JobMessagesRequestMut` cannot.
// - `JobMessagesRequestView` does not use thread-local data.
unsafe impl Send for JobMessagesRequestView<'_> {}

impl<'msg> ::protobuf::AsView for JobMessagesRequestView<'msg> {
  type Proxied = JobMessagesRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, JobMessagesRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobMessagesRequestView<'msg> {
  fn into_view<'shorter>(self) -> JobMessagesRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JobMessagesRequest> for JobMessagesRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobMessagesRequest {
    let mut dst = JobMessagesRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JobMessagesRequest> for JobMessagesRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobMessagesRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JobMessagesRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobMessagesRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobMessagesRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JobMessagesRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobMessagesRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JobMessagesRequestMut<'msg> {
  type Message = JobMessagesRequest;
}

impl ::std::fmt::Debug for JobMessagesRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesRequest>> for JobMessagesRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobMessagesRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> JobMessagesRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `JobMessagesRequestMut` does not perform any shared mutation.
unsafe impl Send for JobMessagesRequestMut<'_> {}

// SAFETY:
// - `JobMessagesRequestMut` does not perform any shared mutation.
unsafe impl Sync for JobMessagesRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for JobMessagesRequestMut<'msg> {
  type Proxied = JobMessagesRequest;
  fn as_view(&self) -> ::protobuf::View<'_, JobMessagesRequest> {
    JobMessagesRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobMessagesRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JobMessagesRequest>
  where
      'msg: 'shorter {
    JobMessagesRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JobMessagesRequestMut<'msg> {
  type MutProxied = JobMessagesRequest;
  fn as_mut(&mut self) -> JobMessagesRequestMut<'msg> {
    JobMessagesRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JobMessagesRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> JobMessagesRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JobMessagesRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JobMessagesRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JobMessagesRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JobMessagesRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl JobMessagesRequest

impl ::std::ops::Drop for JobMessagesRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JobMessagesRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JobMessagesRequest {
  type Proxied = Self;
  fn as_view(&self) -> JobMessagesRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JobMessagesRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JobMessagesRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JobMessagesRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__JobMessagesRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__JobMessagesRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__JobMessagesRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobMessagesRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobMessagesRequest {
  type Msg = JobMessagesRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessagesRequest {
  type Msg = JobMessagesRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobMessagesRequestMut<'_> {
  type Msg = JobMessagesRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessagesRequestMut<'_> {
  type Msg = JobMessagesRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessagesRequestView<'_> {
  type Msg = JobMessagesRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobMessagesRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__JobMessage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JobMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JobMessage>
}

impl ::protobuf::Message for JobMessage {}

impl ::std::default::Default for JobMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JobMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JobMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `JobMessageMut`.
unsafe impl Sync for JobMessage {}

// SAFETY:
// - `JobMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JobMessage {}

impl ::protobuf::Proxied for JobMessage {
  type View<'msg> = JobMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JobMessage {}

impl ::protobuf::MutProxied for JobMessage {
  type Mut<'msg> = JobMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JobMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JobMessageView<'msg> {
  type Message = JobMessage;
}

impl ::std::fmt::Debug for JobMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JobMessageView<'_> {
  fn default() -> JobMessageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessage>> for JobMessageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobMessageView<'msg> {

  pub fn to_owned(&self) -> JobMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // message_id: optional string
  pub fn message_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // time: optional string
  pub fn time(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // importance: optional enum org.apache.beam.model.job_management.v1.JobMessage.MessageImportance
  pub fn importance(self) -> super::job_message::MessageImportance {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::job_message::MessageImportance::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // message_text: optional string
  pub fn message_text(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `JobMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for JobMessageView<'_> {}

// SAFETY:
// - `JobMessageView` is `Send` because while its alive a `JobMessageMut` cannot.
// - `JobMessageView` does not use thread-local data.
unsafe impl Send for JobMessageView<'_> {}

impl<'msg> ::protobuf::AsView for JobMessageView<'msg> {
  type Proxied = JobMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, JobMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobMessageView<'msg> {
  fn into_view<'shorter>(self) -> JobMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JobMessage> for JobMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobMessage {
    let mut dst = JobMessage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JobMessage> for JobMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JobMessage {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobMessageView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobMessageMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JobMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JobMessageMut<'msg> {
  type Message = JobMessage;
}

impl ::std::fmt::Debug for JobMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessage>> for JobMessageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobMessageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> JobMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // message_id: optional string
  pub fn message_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // time: optional string
  pub fn time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // importance: optional enum org.apache.beam.model.job_management.v1.JobMessage.MessageImportance
  pub fn importance(&self) -> super::job_message::MessageImportance {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::job_message::MessageImportance::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_importance(&mut self, val: super::job_message::MessageImportance) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // message_text: optional string
  pub fn message_text(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message_text(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `JobMessageMut` does not perform any shared mutation.
unsafe impl Send for JobMessageMut<'_> {}

// SAFETY:
// - `JobMessageMut` does not perform any shared mutation.
unsafe impl Sync for JobMessageMut<'_> {}

impl<'msg> ::protobuf::AsView for JobMessageMut<'msg> {
  type Proxied = JobMessage;
  fn as_view(&self) -> ::protobuf::View<'_, JobMessage> {
    JobMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JobMessage>
  where
      'msg: 'shorter {
    JobMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JobMessageMut<'msg> {
  type MutProxied = JobMessage;
  fn as_mut(&mut self) -> JobMessageMut<'msg> {
    JobMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JobMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> JobMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JobMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JobMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JobMessageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JobMessageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // message_id: optional string
  pub fn message_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // time: optional string
  pub fn time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // importance: optional enum org.apache.beam.model.job_management.v1.JobMessage.MessageImportance
  pub fn importance(&self) -> super::job_message::MessageImportance {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::job_message::MessageImportance::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_importance(&mut self, val: super::job_message::MessageImportance) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // message_text: optional string
  pub fn message_text(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message_text(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl JobMessage

impl ::std::ops::Drop for JobMessage {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JobMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JobMessage {
  type Proxied = Self;
  fn as_view(&self) -> JobMessageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JobMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JobMessageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JobMessage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__JobMessage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X.P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__JobMessage_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__JobMessage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobMessage {
  type Msg = JobMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessage {
  type Msg = JobMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobMessageMut<'_> {
  type Msg = JobMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessageMut<'_> {
  type Msg = JobMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessageView<'_> {
  type Msg = JobMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod job_message {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageImportance(i32);

#[allow(non_upper_case_globals)]
impl MessageImportance {
  pub const Unspecified: MessageImportance = MessageImportance(0);
  pub const JobMessageDebug: MessageImportance = MessageImportance(1);
  pub const JobMessageDetailed: MessageImportance = MessageImportance(2);
  pub const JobMessageBasic: MessageImportance = MessageImportance(3);
  pub const JobMessageWarning: MessageImportance = MessageImportance(4);
  pub const JobMessageError: MessageImportance = MessageImportance(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "JobMessageDebug",
      2 => "JobMessageDetailed",
      3 => "JobMessageBasic",
      4 => "JobMessageWarning",
      5 => "JobMessageError",
      _ => return None
    })
  }
}

impl ::std::convert::From<MessageImportance> for i32 {
  fn from(val: MessageImportance) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for MessageImportance {
  fn from(val: i32) -> MessageImportance {
    Self(val)
  }
}

impl ::std::default::Default for MessageImportance {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for MessageImportance {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "MessageImportance::{}", constant_name)
    } else {
      write!(f, "MessageImportance::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for MessageImportance {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for MessageImportance {}

impl ::protobuf::Proxied for MessageImportance {
  type View<'a> = MessageImportance;
}

impl ::protobuf::AsView for MessageImportance {
  type Proxied = MessageImportance;

  fn as_view(&self) -> MessageImportance {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MessageImportance {
  fn into_view<'shorter>(self) -> MessageImportance where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for MessageImportance {
  const NAME: &'static str = "MessageImportance";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MessageImportance {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod job_message


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__JobMessagesResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JobMessagesResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JobMessagesResponse>
}

impl ::protobuf::Message for JobMessagesResponse {}

impl ::std::default::Default for JobMessagesResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JobMessagesResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JobMessagesResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `JobMessagesResponseMut`.
unsafe impl Sync for JobMessagesResponse {}

// SAFETY:
// - `JobMessagesResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JobMessagesResponse {}

impl ::protobuf::Proxied for JobMessagesResponse {
  type View<'msg> = JobMessagesResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JobMessagesResponse {}

impl ::protobuf::MutProxied for JobMessagesResponse {
  type Mut<'msg> = JobMessagesResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JobMessagesResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessagesResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobMessagesResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JobMessagesResponseView<'msg> {
  type Message = JobMessagesResponse;
}

impl ::std::fmt::Debug for JobMessagesResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JobMessagesResponseView<'_> {
  fn default() -> JobMessagesResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessagesResponse>> for JobMessagesResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobMessagesResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobMessagesResponseView<'msg> {

  pub fn to_owned(&self) -> JobMessagesResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // message_response: optional message org.apache.beam.model.job_management.v1.JobMessage
  pub fn has_message_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn message_response_opt(self) -> ::protobuf::Optional<super::JobMessageView<'msg>> {
        ::protobuf::Optional::new(self.message_response(), self.has_message_response())
  }
  pub fn message_response(self) -> super::JobMessageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JobMessageView::default())
  }

  // state_response: optional message org.apache.beam.model.job_management.v1.JobStateEvent
  pub fn has_state_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn state_response_opt(self) -> ::protobuf::Optional<super::JobStateEventView<'msg>> {
        ::protobuf::Optional::new(self.state_response(), self.has_state_response())
  }
  pub fn state_response(self) -> super::JobStateEventView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JobStateEventView::default())
  }

  pub fn response(self) -> super::job_messages_response::ResponseOneof<'msg> {
    match self.response_case() {
      super::job_messages_response::ResponseCase::MessageResponse =>
          super::job_messages_response::ResponseOneof::MessageResponse(self.message_response()),
      super::job_messages_response::ResponseCase::StateResponse =>
          super::job_messages_response::ResponseOneof::StateResponse(self.state_response()),
      _ => super::job_messages_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(self) -> super::job_messages_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::job_messages_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `JobMessagesResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for JobMessagesResponseView<'_> {}

// SAFETY:
// - `JobMessagesResponseView` is `Send` because while its alive a `JobMessagesResponseMut` cannot.
// - `JobMessagesResponseView` does not use thread-local data.
unsafe impl Send for JobMessagesResponseView<'_> {}

impl<'msg> ::protobuf::AsView for JobMessagesResponseView<'msg> {
  type Proxied = JobMessagesResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, JobMessagesResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobMessagesResponseView<'msg> {
  fn into_view<'shorter>(self) -> JobMessagesResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JobMessagesResponse> for JobMessagesResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobMessagesResponse {
    let mut dst = JobMessagesResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JobMessagesResponse> for JobMessagesResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobMessagesResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JobMessagesResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobMessagesResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobMessagesResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JobMessagesResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobMessagesResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JobMessagesResponseMut<'msg> {
  type Message = JobMessagesResponse;
}

impl ::std::fmt::Debug for JobMessagesResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesResponse>> for JobMessagesResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobMessagesResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JobMessagesResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> JobMessagesResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // message_response: optional message org.apache.beam.model.job_management.v1.JobMessage
  pub fn has_message_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_message_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn message_response_opt(&self) -> ::protobuf::Optional<super::JobMessageView<'_>> {
        ::protobuf::Optional::new(self.message_response(), self.has_message_response())
  }
  pub fn message_response(&self) -> super::JobMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JobMessageView::default())
  }
  pub fn message_response_mut(&mut self) -> super::JobMessageMut<'_> {
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
  pub fn set_message_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::JobMessage>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // state_response: optional message org.apache.beam.model.job_management.v1.JobStateEvent
  pub fn has_state_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_state_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn state_response_opt(&self) -> ::protobuf::Optional<super::JobStateEventView<'_>> {
        ::protobuf::Optional::new(self.state_response(), self.has_state_response())
  }
  pub fn state_response(&self) -> super::JobStateEventView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JobStateEventView::default())
  }
  pub fn state_response_mut(&mut self) -> super::JobStateEventMut<'_> {
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
  pub fn set_state_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::JobStateEvent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn response(&self) -> super::job_messages_response::ResponseOneof<'_> {
    match &self.response_case() {
      super::job_messages_response::ResponseCase::MessageResponse =>
          super::job_messages_response::ResponseOneof::MessageResponse(self.message_response()),
      super::job_messages_response::ResponseCase::StateResponse =>
          super::job_messages_response::ResponseOneof::StateResponse(self.state_response()),
      _ => super::job_messages_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::job_messages_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::job_messages_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `JobMessagesResponseMut` does not perform any shared mutation.
unsafe impl Send for JobMessagesResponseMut<'_> {}

// SAFETY:
// - `JobMessagesResponseMut` does not perform any shared mutation.
unsafe impl Sync for JobMessagesResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for JobMessagesResponseMut<'msg> {
  type Proxied = JobMessagesResponse;
  fn as_view(&self) -> ::protobuf::View<'_, JobMessagesResponse> {
    JobMessagesResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobMessagesResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JobMessagesResponse>
  where
      'msg: 'shorter {
    JobMessagesResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JobMessagesResponseMut<'msg> {
  type MutProxied = JobMessagesResponse;
  fn as_mut(&mut self) -> JobMessagesResponseMut<'msg> {
    JobMessagesResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JobMessagesResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> JobMessagesResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JobMessagesResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JobMessagesResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JobMessagesResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JobMessagesResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // message_response: optional message org.apache.beam.model.job_management.v1.JobMessage
  pub fn has_message_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_message_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn message_response_opt(&self) -> ::protobuf::Optional<super::JobMessageView<'_>> {
        ::protobuf::Optional::new(self.message_response(), self.has_message_response())
  }
  pub fn message_response(&self) -> super::JobMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JobMessageView::default())
  }
  pub fn message_response_mut(&mut self) -> super::JobMessageMut<'_> {
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
  pub fn set_message_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::JobMessage>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // state_response: optional message org.apache.beam.model.job_management.v1.JobStateEvent
  pub fn has_state_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_state_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn state_response_opt(&self) -> ::protobuf::Optional<super::JobStateEventView<'_>> {
        ::protobuf::Optional::new(self.state_response(), self.has_state_response())
  }
  pub fn state_response(&self) -> super::JobStateEventView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::JobStateEventView::default())
  }
  pub fn state_response_mut(&mut self) -> super::JobStateEventMut<'_> {
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
  pub fn set_state_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::JobStateEvent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn response(&self) -> super::job_messages_response::ResponseOneof<'_> {
    match &self.response_case() {
      super::job_messages_response::ResponseCase::MessageResponse =>
          super::job_messages_response::ResponseOneof::MessageResponse(self.message_response()),
      super::job_messages_response::ResponseCase::StateResponse =>
          super::job_messages_response::ResponseOneof::StateResponse(self.state_response()),
      _ => super::job_messages_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::job_messages_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::job_messages_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl JobMessagesResponse

impl ::std::ops::Drop for JobMessagesResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JobMessagesResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JobMessagesResponse {
  type Proxied = Self;
  fn as_view(&self) -> JobMessagesResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JobMessagesResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JobMessagesResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JobMessagesResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__JobMessagesResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__JobMessagesResponse_msg_init.0, &[<super::JobMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::JobStateEvent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__JobMessagesResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobMessagesResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobMessagesResponse {
  type Msg = JobMessagesResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessagesResponse {
  type Msg = JobMessagesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobMessagesResponseMut<'_> {
  type Msg = JobMessagesResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessagesResponseMut<'_> {
  type Msg = JobMessagesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobMessagesResponseView<'_> {
  type Msg = JobMessagesResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobMessagesResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobMessagesResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod job_messages_response {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ResponseOneof<'msg> {
  MessageResponse(::protobuf::View<'msg, super::super::JobMessage>) = 1,
  StateResponse(::protobuf::View<'msg, super::super::JobStateEvent>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ResponseCase {
  MessageResponse = 1,
  StateResponse = 2,

  not_set = 0
}

impl ResponseCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ResponseCase> {
    match v {
      0 => Some(ResponseCase::not_set),
      1 => Some(ResponseCase::MessageResponse),
      2 => Some(ResponseCase::StateResponse),
      _ => None
    }
  }
}
}  // pub mod job_messages_response


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__JobState_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct JobState {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<JobState>
}

impl ::protobuf::Message for JobState {}

impl ::std::default::Default for JobState {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for JobState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `JobState` is `Sync` because it does not implement interior mutability.
//    Neither does `JobStateMut`.
unsafe impl Sync for JobState {}

// SAFETY:
// - `JobState` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for JobState {}

impl ::protobuf::Proxied for JobState {
  type View<'msg> = JobStateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for JobState {}

impl ::protobuf::MutProxied for JobState {
  type Mut<'msg> = JobStateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct JobStateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobStateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for JobStateView<'msg> {
  type Message = JobState;
}

impl ::std::fmt::Debug for JobStateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for JobStateView<'_> {
  fn default() -> JobStateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, JobState>> for JobStateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, JobState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobStateView<'msg> {

  pub fn to_owned(&self) -> JobState {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `JobStateView` is `Sync` because it does not support mutation.
unsafe impl Sync for JobStateView<'_> {}

// SAFETY:
// - `JobStateView` is `Send` because while its alive a `JobStateMut` cannot.
// - `JobStateView` does not use thread-local data.
unsafe impl Send for JobStateView<'_> {}

impl<'msg> ::protobuf::AsView for JobStateView<'msg> {
  type Proxied = JobState;
  fn as_view(&self) -> ::protobuf::View<'msg, JobState> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobStateView<'msg> {
  fn into_view<'shorter>(self) -> JobStateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<JobState> for JobStateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobState {
    let mut dst = JobState::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<JobState> for JobStateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> JobState {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for JobState {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobStateView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for JobStateMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct JobStateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for JobStateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for JobStateMut<'msg> {
  type Message = JobState;
}

impl ::std::fmt::Debug for JobStateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, JobState>> for JobStateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, JobState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> JobStateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, JobState> {
    self.inner
  }

  pub fn to_owned(&self) -> JobState {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `JobStateMut` does not perform any shared mutation.
unsafe impl Send for JobStateMut<'_> {}

// SAFETY:
// - `JobStateMut` does not perform any shared mutation.
unsafe impl Sync for JobStateMut<'_> {}

impl<'msg> ::protobuf::AsView for JobStateMut<'msg> {
  type Proxied = JobState;
  fn as_view(&self) -> ::protobuf::View<'_, JobState> {
    JobStateView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for JobStateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, JobState>
  where
      'msg: 'shorter {
    JobStateView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for JobStateMut<'msg> {
  type MutProxied = JobState;
  fn as_mut(&mut self) -> JobStateMut<'msg> {
    JobStateMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for JobStateMut<'msg> {
  fn into_mut<'shorter>(self) -> JobStateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl JobState {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, JobState> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> JobStateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> JobStateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl JobState

impl ::std::ops::Drop for JobState {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for JobState {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for JobState {
  type Proxied = Self;
  fn as_view(&self) -> JobStateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for JobState {
  type MutProxied = Self;
  fn as_mut(&mut self) -> JobStateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for JobState {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__JobState_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__JobState_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__JobState_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobState {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobState {
  type Msg = JobState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobState {
  type Msg = JobState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for JobStateMut<'_> {
  type Msg = JobState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobStateMut<'_> {
  type Msg = JobState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for JobStateView<'_> {
  type Msg = JobState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<JobState> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for JobStateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod job_state {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const Unspecified: Enum = Enum(0);
  pub const Stopped: Enum = Enum(1);
  pub const Running: Enum = Enum(2);
  pub const Done: Enum = Enum(3);
  pub const Failed: Enum = Enum(4);
  pub const Cancelled: Enum = Enum(5);
  pub const Updated: Enum = Enum(6);
  pub const Draining: Enum = Enum(7);
  pub const Drained: Enum = Enum(8);
  pub const Starting: Enum = Enum(9);
  pub const Cancelling: Enum = Enum(10);
  pub const Updating: Enum = Enum(11);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Stopped",
      2 => "Running",
      3 => "Done",
      4 => "Failed",
      5 => "Cancelled",
      6 => "Updated",
      7 => "Draining",
      8 => "Drained",
      9 => "Starting",
      10 => "Cancelling",
      11 => "Updating",
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
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod job_state


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetJobMetricsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetJobMetricsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetJobMetricsRequest>
}

impl ::protobuf::Message for GetJobMetricsRequest {}

impl ::std::default::Default for GetJobMetricsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetJobMetricsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetJobMetricsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetJobMetricsRequestMut`.
unsafe impl Sync for GetJobMetricsRequest {}

// SAFETY:
// - `GetJobMetricsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetJobMetricsRequest {}

impl ::protobuf::Proxied for GetJobMetricsRequest {
  type View<'msg> = GetJobMetricsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetJobMetricsRequest {}

impl ::protobuf::MutProxied for GetJobMetricsRequest {
  type Mut<'msg> = GetJobMetricsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetJobMetricsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobMetricsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobMetricsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetJobMetricsRequestView<'msg> {
  type Message = GetJobMetricsRequest;
}

impl ::std::fmt::Debug for GetJobMetricsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetJobMetricsRequestView<'_> {
  fn default() -> GetJobMetricsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobMetricsRequest>> for GetJobMetricsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobMetricsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobMetricsRequestView<'msg> {

  pub fn to_owned(&self) -> GetJobMetricsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // job_id: optional string
  pub fn job_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `GetJobMetricsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetJobMetricsRequestView<'_> {}

// SAFETY:
// - `GetJobMetricsRequestView` is `Send` because while its alive a `GetJobMetricsRequestMut` cannot.
// - `GetJobMetricsRequestView` does not use thread-local data.
unsafe impl Send for GetJobMetricsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetJobMetricsRequestView<'msg> {
  type Proxied = GetJobMetricsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetJobMetricsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobMetricsRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetJobMetricsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobMetricsRequest> for GetJobMetricsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobMetricsRequest {
    let mut dst = GetJobMetricsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobMetricsRequest> for GetJobMetricsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobMetricsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetJobMetricsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobMetricsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobMetricsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetJobMetricsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobMetricsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetJobMetricsRequestMut<'msg> {
  type Message = GetJobMetricsRequest;
}

impl ::std::fmt::Debug for GetJobMetricsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsRequest>> for GetJobMetricsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobMetricsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetJobMetricsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetJobMetricsRequestMut` does not perform any shared mutation.
unsafe impl Send for GetJobMetricsRequestMut<'_> {}

// SAFETY:
// - `GetJobMetricsRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetJobMetricsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetJobMetricsRequestMut<'msg> {
  type Proxied = GetJobMetricsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetJobMetricsRequest> {
    GetJobMetricsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobMetricsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetJobMetricsRequest>
  where
      'msg: 'shorter {
    GetJobMetricsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetJobMetricsRequestMut<'msg> {
  type MutProxied = GetJobMetricsRequest;
  fn as_mut(&mut self) -> GetJobMetricsRequestMut<'msg> {
    GetJobMetricsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetJobMetricsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetJobMetricsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetJobMetricsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetJobMetricsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetJobMetricsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetJobMetricsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // job_id: optional string
  pub fn job_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_job_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetJobMetricsRequest

impl ::std::ops::Drop for GetJobMetricsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetJobMetricsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetJobMetricsRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetJobMetricsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetJobMetricsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetJobMetricsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetJobMetricsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetJobMetricsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetJobMetricsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetJobMetricsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobMetricsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobMetricsRequest {
  type Msg = GetJobMetricsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobMetricsRequest {
  type Msg = GetJobMetricsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobMetricsRequestMut<'_> {
  type Msg = GetJobMetricsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobMetricsRequestMut<'_> {
  type Msg = GetJobMetricsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobMetricsRequestView<'_> {
  type Msg = GetJobMetricsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobMetricsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__GetJobMetricsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetJobMetricsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetJobMetricsResponse>
}

impl ::protobuf::Message for GetJobMetricsResponse {}

impl ::std::default::Default for GetJobMetricsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetJobMetricsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetJobMetricsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetJobMetricsResponseMut`.
unsafe impl Sync for GetJobMetricsResponse {}

// SAFETY:
// - `GetJobMetricsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetJobMetricsResponse {}

impl ::protobuf::Proxied for GetJobMetricsResponse {
  type View<'msg> = GetJobMetricsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetJobMetricsResponse {}

impl ::protobuf::MutProxied for GetJobMetricsResponse {
  type Mut<'msg> = GetJobMetricsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetJobMetricsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobMetricsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobMetricsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetJobMetricsResponseView<'msg> {
  type Message = GetJobMetricsResponse;
}

impl ::std::fmt::Debug for GetJobMetricsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetJobMetricsResponseView<'_> {
  fn default() -> GetJobMetricsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobMetricsResponse>> for GetJobMetricsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetJobMetricsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobMetricsResponseView<'msg> {

  pub fn to_owned(&self) -> GetJobMetricsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // metrics: optional message org.apache.beam.model.job_management.v1.MetricResults
  pub fn has_metrics(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn metrics_opt(self) -> ::protobuf::Optional<super::MetricResultsView<'msg>> {
        ::protobuf::Optional::new(self.metrics(), self.has_metrics())
  }
  pub fn metrics(self) -> super::MetricResultsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetricResultsView::default())
  }

}

// SAFETY:
// - `GetJobMetricsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetJobMetricsResponseView<'_> {}

// SAFETY:
// - `GetJobMetricsResponseView` is `Send` because while its alive a `GetJobMetricsResponseMut` cannot.
// - `GetJobMetricsResponseView` does not use thread-local data.
unsafe impl Send for GetJobMetricsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for GetJobMetricsResponseView<'msg> {
  type Proxied = GetJobMetricsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetJobMetricsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobMetricsResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetJobMetricsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobMetricsResponse> for GetJobMetricsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobMetricsResponse {
    let mut dst = GetJobMetricsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetJobMetricsResponse> for GetJobMetricsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetJobMetricsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetJobMetricsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobMetricsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetJobMetricsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetJobMetricsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetJobMetricsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetJobMetricsResponseMut<'msg> {
  type Message = GetJobMetricsResponse;
}

impl ::std::fmt::Debug for GetJobMetricsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsResponse>> for GetJobMetricsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetJobMetricsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetJobMetricsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> GetJobMetricsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // metrics: optional message org.apache.beam.model.job_management.v1.MetricResults
  pub fn has_metrics(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_metrics(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn metrics_opt(&self) -> ::protobuf::Optional<super::MetricResultsView<'_>> {
        ::protobuf::Optional::new(self.metrics(), self.has_metrics())
  }
  pub fn metrics(&self) -> super::MetricResultsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetricResultsView::default())
  }
  pub fn metrics_mut(&mut self) -> super::MetricResultsMut<'_> {
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
  pub fn set_metrics(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetricResults>) {

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
// - `GetJobMetricsResponseMut` does not perform any shared mutation.
unsafe impl Send for GetJobMetricsResponseMut<'_> {}

// SAFETY:
// - `GetJobMetricsResponseMut` does not perform any shared mutation.
unsafe impl Sync for GetJobMetricsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for GetJobMetricsResponseMut<'msg> {
  type Proxied = GetJobMetricsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetJobMetricsResponse> {
    GetJobMetricsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetJobMetricsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetJobMetricsResponse>
  where
      'msg: 'shorter {
    GetJobMetricsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetJobMetricsResponseMut<'msg> {
  type MutProxied = GetJobMetricsResponse;
  fn as_mut(&mut self) -> GetJobMetricsResponseMut<'msg> {
    GetJobMetricsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetJobMetricsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetJobMetricsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetJobMetricsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetJobMetricsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetJobMetricsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetJobMetricsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // metrics: optional message org.apache.beam.model.job_management.v1.MetricResults
  pub fn has_metrics(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_metrics(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn metrics_opt(&self) -> ::protobuf::Optional<super::MetricResultsView<'_>> {
        ::protobuf::Optional::new(self.metrics(), self.has_metrics())
  }
  pub fn metrics(&self) -> super::MetricResultsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetricResultsView::default())
  }
  pub fn metrics_mut(&mut self) -> super::MetricResultsMut<'_> {
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
  pub fn set_metrics(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetricResults>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GetJobMetricsResponse

impl ::std::ops::Drop for GetJobMetricsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetJobMetricsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetJobMetricsResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetJobMetricsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetJobMetricsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetJobMetricsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetJobMetricsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__GetJobMetricsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__GetJobMetricsResponse_msg_init.0, &[<super::MetricResults as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__GetJobMetricsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobMetricsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobMetricsResponse {
  type Msg = GetJobMetricsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobMetricsResponse {
  type Msg = GetJobMetricsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetJobMetricsResponseMut<'_> {
  type Msg = GetJobMetricsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobMetricsResponseMut<'_> {
  type Msg = GetJobMetricsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetJobMetricsResponseView<'_> {
  type Msg = GetJobMetricsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetJobMetricsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetJobMetricsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__MetricResults_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetricResults {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetricResults>
}

impl ::protobuf::Message for MetricResults {}

impl ::std::default::Default for MetricResults {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetricResults {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetricResults` is `Sync` because it does not implement interior mutability.
//    Neither does `MetricResultsMut`.
unsafe impl Sync for MetricResults {}

// SAFETY:
// - `MetricResults` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MetricResults {}

impl ::protobuf::Proxied for MetricResults {
  type View<'msg> = MetricResultsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetricResults {}

impl ::protobuf::MutProxied for MetricResults {
  type Mut<'msg> = MetricResultsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetricResultsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetricResults>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetricResultsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetricResultsView<'msg> {
  type Message = MetricResults;
}

impl ::std::fmt::Debug for MetricResultsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetricResultsView<'_> {
  fn default() -> MetricResultsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetricResults>> for MetricResultsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetricResults>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetricResultsView<'msg> {

  pub fn to_owned(&self) -> MetricResults {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // attempted: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo
  pub fn attempted(self) -> ::protobuf::RepeatedView<'msg, super::MonitoringInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MonitoringInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // committed: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo
  pub fn committed(self) -> ::protobuf::RepeatedView<'msg, super::MonitoringInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MonitoringInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MetricResultsView` is `Sync` because it does not support mutation.
unsafe impl Sync for MetricResultsView<'_> {}

// SAFETY:
// - `MetricResultsView` is `Send` because while its alive a `MetricResultsMut` cannot.
// - `MetricResultsView` does not use thread-local data.
unsafe impl Send for MetricResultsView<'_> {}

impl<'msg> ::protobuf::AsView for MetricResultsView<'msg> {
  type Proxied = MetricResults;
  fn as_view(&self) -> ::protobuf::View<'msg, MetricResults> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetricResultsView<'msg> {
  fn into_view<'shorter>(self) -> MetricResultsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetricResults> for MetricResultsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetricResults {
    let mut dst = MetricResults::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetricResults> for MetricResultsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetricResults {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MetricResults {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MetricResultsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MetricResultsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetricResultsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetricResults>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetricResultsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetricResultsMut<'msg> {
  type Message = MetricResults;
}

impl ::std::fmt::Debug for MetricResultsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetricResults>> for MetricResultsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetricResults>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetricResultsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetricResults> {
    self.inner
  }

  pub fn to_owned(&self) -> MetricResults {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // attempted: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo
  pub fn attempted(&self) -> ::protobuf::RepeatedView<'_, super::MonitoringInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MonitoringInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn attempted_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::MonitoringInfo> {
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
  pub fn set_attempted(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::MonitoringInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // committed: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo
  pub fn committed(&self) -> ::protobuf::RepeatedView<'_, super::MonitoringInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MonitoringInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn committed_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::MonitoringInfo> {
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
  pub fn set_committed(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::MonitoringInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `MetricResultsMut` does not perform any shared mutation.
unsafe impl Send for MetricResultsMut<'_> {}

// SAFETY:
// - `MetricResultsMut` does not perform any shared mutation.
unsafe impl Sync for MetricResultsMut<'_> {}

impl<'msg> ::protobuf::AsView for MetricResultsMut<'msg> {
  type Proxied = MetricResults;
  fn as_view(&self) -> ::protobuf::View<'_, MetricResults> {
    MetricResultsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetricResultsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetricResults>
  where
      'msg: 'shorter {
    MetricResultsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MetricResultsMut<'msg> {
  type MutProxied = MetricResults;
  fn as_mut(&mut self) -> MetricResultsMut<'msg> {
    MetricResultsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetricResultsMut<'msg> {
  fn into_mut<'shorter>(self) -> MetricResultsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetricResults {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetricResults> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetricResultsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetricResultsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // attempted: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo
  pub fn attempted(&self) -> ::protobuf::RepeatedView<'_, super::MonitoringInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MonitoringInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn attempted_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::MonitoringInfo> {
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
  pub fn set_attempted(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::MonitoringInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // committed: repeated message org.apache.beam.model.pipeline.v1.MonitoringInfo
  pub fn committed(&self) -> ::protobuf::RepeatedView<'_, super::MonitoringInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MonitoringInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn committed_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::MonitoringInfo> {
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
  pub fn set_committed(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::MonitoringInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl MetricResults

impl ::std::ops::Drop for MetricResults {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetricResults {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetricResults {
  type Proxied = Self;
  fn as_view(&self) -> MetricResultsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetricResults {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetricResultsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetricResults {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__MetricResults_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__MetricResults_msg_init.0, &[<super::MonitoringInfo as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::MonitoringInfo as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__MetricResults_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetricResults {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetricResults {
  type Msg = MetricResults;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetricResults> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetricResults {
  type Msg = MetricResults;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetricResults> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetricResultsMut<'_> {
  type Msg = MetricResults;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetricResults> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetricResultsMut<'_> {
  type Msg = MetricResults;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetricResults> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetricResultsView<'_> {
  type Msg = MetricResults;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetricResults> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetricResultsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__DescribePipelineOptionsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DescribePipelineOptionsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DescribePipelineOptionsRequest>
}

impl ::protobuf::Message for DescribePipelineOptionsRequest {}

impl ::std::default::Default for DescribePipelineOptionsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DescribePipelineOptionsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DescribePipelineOptionsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DescribePipelineOptionsRequestMut`.
unsafe impl Sync for DescribePipelineOptionsRequest {}

// SAFETY:
// - `DescribePipelineOptionsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DescribePipelineOptionsRequest {}

impl ::protobuf::Proxied for DescribePipelineOptionsRequest {
  type View<'msg> = DescribePipelineOptionsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DescribePipelineOptionsRequest {}

impl ::protobuf::MutProxied for DescribePipelineOptionsRequest {
  type Mut<'msg> = DescribePipelineOptionsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DescribePipelineOptionsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DescribePipelineOptionsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DescribePipelineOptionsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DescribePipelineOptionsRequestView<'msg> {
  type Message = DescribePipelineOptionsRequest;
}

impl ::std::fmt::Debug for DescribePipelineOptionsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DescribePipelineOptionsRequestView<'_> {
  fn default() -> DescribePipelineOptionsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DescribePipelineOptionsRequest>> for DescribePipelineOptionsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DescribePipelineOptionsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DescribePipelineOptionsRequestView<'msg> {

  pub fn to_owned(&self) -> DescribePipelineOptionsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `DescribePipelineOptionsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DescribePipelineOptionsRequestView<'_> {}

// SAFETY:
// - `DescribePipelineOptionsRequestView` is `Send` because while its alive a `DescribePipelineOptionsRequestMut` cannot.
// - `DescribePipelineOptionsRequestView` does not use thread-local data.
unsafe impl Send for DescribePipelineOptionsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DescribePipelineOptionsRequestView<'msg> {
  type Proxied = DescribePipelineOptionsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DescribePipelineOptionsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DescribePipelineOptionsRequestView<'msg> {
  fn into_view<'shorter>(self) -> DescribePipelineOptionsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DescribePipelineOptionsRequest> for DescribePipelineOptionsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DescribePipelineOptionsRequest {
    let mut dst = DescribePipelineOptionsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DescribePipelineOptionsRequest> for DescribePipelineOptionsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DescribePipelineOptionsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DescribePipelineOptionsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DescribePipelineOptionsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DescribePipelineOptionsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DescribePipelineOptionsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DescribePipelineOptionsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DescribePipelineOptionsRequestMut<'msg> {
  type Message = DescribePipelineOptionsRequest;
}

impl ::std::fmt::Debug for DescribePipelineOptionsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsRequest>> for DescribePipelineOptionsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DescribePipelineOptionsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DescribePipelineOptionsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `DescribePipelineOptionsRequestMut` does not perform any shared mutation.
unsafe impl Send for DescribePipelineOptionsRequestMut<'_> {}

// SAFETY:
// - `DescribePipelineOptionsRequestMut` does not perform any shared mutation.
unsafe impl Sync for DescribePipelineOptionsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DescribePipelineOptionsRequestMut<'msg> {
  type Proxied = DescribePipelineOptionsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DescribePipelineOptionsRequest> {
    DescribePipelineOptionsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DescribePipelineOptionsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DescribePipelineOptionsRequest>
  where
      'msg: 'shorter {
    DescribePipelineOptionsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DescribePipelineOptionsRequestMut<'msg> {
  type MutProxied = DescribePipelineOptionsRequest;
  fn as_mut(&mut self) -> DescribePipelineOptionsRequestMut<'msg> {
    DescribePipelineOptionsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DescribePipelineOptionsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DescribePipelineOptionsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DescribePipelineOptionsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DescribePipelineOptionsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DescribePipelineOptionsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DescribePipelineOptionsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl DescribePipelineOptionsRequest

impl ::std::ops::Drop for DescribePipelineOptionsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DescribePipelineOptionsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DescribePipelineOptionsRequest {
  type Proxied = Self;
  fn as_view(&self) -> DescribePipelineOptionsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DescribePipelineOptionsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DescribePipelineOptionsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DescribePipelineOptionsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__DescribePipelineOptionsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__DescribePipelineOptionsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__DescribePipelineOptionsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DescribePipelineOptionsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DescribePipelineOptionsRequest {
  type Msg = DescribePipelineOptionsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DescribePipelineOptionsRequest {
  type Msg = DescribePipelineOptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DescribePipelineOptionsRequestMut<'_> {
  type Msg = DescribePipelineOptionsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DescribePipelineOptionsRequestMut<'_> {
  type Msg = DescribePipelineOptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DescribePipelineOptionsRequestView<'_> {
  type Msg = DescribePipelineOptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DescribePipelineOptionsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__PipelineOptionType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PipelineOptionType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PipelineOptionType>
}

impl ::protobuf::Message for PipelineOptionType {}

impl ::std::default::Default for PipelineOptionType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PipelineOptionType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PipelineOptionType` is `Sync` because it does not implement interior mutability.
//    Neither does `PipelineOptionTypeMut`.
unsafe impl Sync for PipelineOptionType {}

// SAFETY:
// - `PipelineOptionType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PipelineOptionType {}

impl ::protobuf::Proxied for PipelineOptionType {
  type View<'msg> = PipelineOptionTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PipelineOptionType {}

impl ::protobuf::MutProxied for PipelineOptionType {
  type Mut<'msg> = PipelineOptionTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PipelineOptionTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PipelineOptionType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PipelineOptionTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PipelineOptionTypeView<'msg> {
  type Message = PipelineOptionType;
}

impl ::std::fmt::Debug for PipelineOptionTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PipelineOptionTypeView<'_> {
  fn default() -> PipelineOptionTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PipelineOptionType>> for PipelineOptionTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PipelineOptionType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PipelineOptionTypeView<'msg> {

  pub fn to_owned(&self) -> PipelineOptionType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `PipelineOptionTypeView` is `Sync` because it does not support mutation.
unsafe impl Sync for PipelineOptionTypeView<'_> {}

// SAFETY:
// - `PipelineOptionTypeView` is `Send` because while its alive a `PipelineOptionTypeMut` cannot.
// - `PipelineOptionTypeView` does not use thread-local data.
unsafe impl Send for PipelineOptionTypeView<'_> {}

impl<'msg> ::protobuf::AsView for PipelineOptionTypeView<'msg> {
  type Proxied = PipelineOptionType;
  fn as_view(&self) -> ::protobuf::View<'msg, PipelineOptionType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PipelineOptionTypeView<'msg> {
  fn into_view<'shorter>(self) -> PipelineOptionTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PipelineOptionType> for PipelineOptionTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PipelineOptionType {
    let mut dst = PipelineOptionType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PipelineOptionType> for PipelineOptionTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PipelineOptionType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PipelineOptionType {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PipelineOptionTypeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PipelineOptionTypeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PipelineOptionTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PipelineOptionTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PipelineOptionTypeMut<'msg> {
  type Message = PipelineOptionType;
}

impl ::std::fmt::Debug for PipelineOptionTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionType>> for PipelineOptionTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PipelineOptionTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionType> {
    self.inner
  }

  pub fn to_owned(&self) -> PipelineOptionType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `PipelineOptionTypeMut` does not perform any shared mutation.
unsafe impl Send for PipelineOptionTypeMut<'_> {}

// SAFETY:
// - `PipelineOptionTypeMut` does not perform any shared mutation.
unsafe impl Sync for PipelineOptionTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for PipelineOptionTypeMut<'msg> {
  type Proxied = PipelineOptionType;
  fn as_view(&self) -> ::protobuf::View<'_, PipelineOptionType> {
    PipelineOptionTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PipelineOptionTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PipelineOptionType>
  where
      'msg: 'shorter {
    PipelineOptionTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PipelineOptionTypeMut<'msg> {
  type MutProxied = PipelineOptionType;
  fn as_mut(&mut self) -> PipelineOptionTypeMut<'msg> {
    PipelineOptionTypeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PipelineOptionTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> PipelineOptionTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PipelineOptionType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PipelineOptionType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PipelineOptionTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PipelineOptionTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl PipelineOptionType

impl ::std::ops::Drop for PipelineOptionType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PipelineOptionType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PipelineOptionType {
  type Proxied = Self;
  fn as_view(&self) -> PipelineOptionTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PipelineOptionType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PipelineOptionTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PipelineOptionType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__PipelineOptionType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__PipelineOptionType_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__PipelineOptionType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PipelineOptionType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PipelineOptionType {
  type Msg = PipelineOptionType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipelineOptionType {
  type Msg = PipelineOptionType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PipelineOptionTypeMut<'_> {
  type Msg = PipelineOptionType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipelineOptionTypeMut<'_> {
  type Msg = PipelineOptionType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipelineOptionTypeView<'_> {
  type Msg = PipelineOptionType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PipelineOptionTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod pipeline_option_type {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const String: Enum = Enum(0);
  pub const Boolean: Enum = Enum(1);
  pub const Integer: Enum = Enum(2);
  pub const Number: Enum = Enum(3);
  pub const Array: Enum = Enum(4);
  pub const Object: Enum = Enum(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "String",
      1 => "Boolean",
      2 => "Integer",
      3 => "Number",
      4 => "Array",
      5 => "Object",
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
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod pipeline_option_type


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__PipelineOptionDescriptor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PipelineOptionDescriptor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PipelineOptionDescriptor>
}

impl ::protobuf::Message for PipelineOptionDescriptor {}

impl ::std::default::Default for PipelineOptionDescriptor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PipelineOptionDescriptor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PipelineOptionDescriptor` is `Sync` because it does not implement interior mutability.
//    Neither does `PipelineOptionDescriptorMut`.
unsafe impl Sync for PipelineOptionDescriptor {}

// SAFETY:
// - `PipelineOptionDescriptor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PipelineOptionDescriptor {}

impl ::protobuf::Proxied for PipelineOptionDescriptor {
  type View<'msg> = PipelineOptionDescriptorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PipelineOptionDescriptor {}

impl ::protobuf::MutProxied for PipelineOptionDescriptor {
  type Mut<'msg> = PipelineOptionDescriptorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PipelineOptionDescriptorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PipelineOptionDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PipelineOptionDescriptorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PipelineOptionDescriptorView<'msg> {
  type Message = PipelineOptionDescriptor;
}

impl ::std::fmt::Debug for PipelineOptionDescriptorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PipelineOptionDescriptorView<'_> {
  fn default() -> PipelineOptionDescriptorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PipelineOptionDescriptor>> for PipelineOptionDescriptorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PipelineOptionDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PipelineOptionDescriptorView<'msg> {

  pub fn to_owned(&self) -> PipelineOptionDescriptor {
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

  // type: optional enum org.apache.beam.model.job_management.v1.PipelineOptionType.Enum
  pub fn r#type(self) -> super::pipeline_option_type::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::pipeline_option_type::Enum::String).into()
      ).try_into().unwrap()
    }
  }

  // description: optional string
  pub fn description(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // default_value: optional string
  pub fn default_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // group: optional string
  pub fn group(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `PipelineOptionDescriptorView` is `Sync` because it does not support mutation.
unsafe impl Sync for PipelineOptionDescriptorView<'_> {}

// SAFETY:
// - `PipelineOptionDescriptorView` is `Send` because while its alive a `PipelineOptionDescriptorMut` cannot.
// - `PipelineOptionDescriptorView` does not use thread-local data.
unsafe impl Send for PipelineOptionDescriptorView<'_> {}

impl<'msg> ::protobuf::AsView for PipelineOptionDescriptorView<'msg> {
  type Proxied = PipelineOptionDescriptor;
  fn as_view(&self) -> ::protobuf::View<'msg, PipelineOptionDescriptor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PipelineOptionDescriptorView<'msg> {
  fn into_view<'shorter>(self) -> PipelineOptionDescriptorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PipelineOptionDescriptor> for PipelineOptionDescriptorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PipelineOptionDescriptor {
    let mut dst = PipelineOptionDescriptor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PipelineOptionDescriptor> for PipelineOptionDescriptorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PipelineOptionDescriptor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PipelineOptionDescriptor {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PipelineOptionDescriptorView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PipelineOptionDescriptorMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PipelineOptionDescriptorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PipelineOptionDescriptorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PipelineOptionDescriptorMut<'msg> {
  type Message = PipelineOptionDescriptor;
}

impl ::std::fmt::Debug for PipelineOptionDescriptorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionDescriptor>> for PipelineOptionDescriptorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PipelineOptionDescriptorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PipelineOptionDescriptor> {
    self.inner
  }

  pub fn to_owned(&self) -> PipelineOptionDescriptor {
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

  // type: optional enum org.apache.beam.model.job_management.v1.PipelineOptionType.Enum
  pub fn r#type(&self) -> super::pipeline_option_type::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::pipeline_option_type::Enum::String).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: super::pipeline_option_type::Enum) {
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

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // group: optional string
  pub fn group(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_group(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}

// SAFETY:
// - `PipelineOptionDescriptorMut` does not perform any shared mutation.
unsafe impl Send for PipelineOptionDescriptorMut<'_> {}

// SAFETY:
// - `PipelineOptionDescriptorMut` does not perform any shared mutation.
unsafe impl Sync for PipelineOptionDescriptorMut<'_> {}

impl<'msg> ::protobuf::AsView for PipelineOptionDescriptorMut<'msg> {
  type Proxied = PipelineOptionDescriptor;
  fn as_view(&self) -> ::protobuf::View<'_, PipelineOptionDescriptor> {
    PipelineOptionDescriptorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PipelineOptionDescriptorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PipelineOptionDescriptor>
  where
      'msg: 'shorter {
    PipelineOptionDescriptorView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PipelineOptionDescriptorMut<'msg> {
  type MutProxied = PipelineOptionDescriptor;
  fn as_mut(&mut self) -> PipelineOptionDescriptorMut<'msg> {
    PipelineOptionDescriptorMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PipelineOptionDescriptorMut<'msg> {
  fn into_mut<'shorter>(self) -> PipelineOptionDescriptorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PipelineOptionDescriptor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PipelineOptionDescriptor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PipelineOptionDescriptorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PipelineOptionDescriptorMut<'_> {
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

  // type: optional enum org.apache.beam.model.job_management.v1.PipelineOptionType.Enum
  pub fn r#type(&self) -> super::pipeline_option_type::Enum {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::pipeline_option_type::Enum::String).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: super::pipeline_option_type::Enum) {
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

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // default_value: optional string
  pub fn default_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_default_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // group: optional string
  pub fn group(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_group(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}  // impl PipelineOptionDescriptor

impl ::std::ops::Drop for PipelineOptionDescriptor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PipelineOptionDescriptor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PipelineOptionDescriptor {
  type Proxied = Self;
  fn as_view(&self) -> PipelineOptionDescriptorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PipelineOptionDescriptor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PipelineOptionDescriptorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PipelineOptionDescriptor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__PipelineOptionDescriptor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P1X1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__PipelineOptionDescriptor_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__PipelineOptionDescriptor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PipelineOptionDescriptor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PipelineOptionDescriptor {
  type Msg = PipelineOptionDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipelineOptionDescriptor {
  type Msg = PipelineOptionDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PipelineOptionDescriptorMut<'_> {
  type Msg = PipelineOptionDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipelineOptionDescriptorMut<'_> {
  type Msg = PipelineOptionDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipelineOptionDescriptorView<'_> {
  type Msg = PipelineOptionDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PipelineOptionDescriptor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PipelineOptionDescriptorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__job_0management__v1__DescribePipelineOptionsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DescribePipelineOptionsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DescribePipelineOptionsResponse>
}

impl ::protobuf::Message for DescribePipelineOptionsResponse {}

impl ::std::default::Default for DescribePipelineOptionsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DescribePipelineOptionsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DescribePipelineOptionsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DescribePipelineOptionsResponseMut`.
unsafe impl Sync for DescribePipelineOptionsResponse {}

// SAFETY:
// - `DescribePipelineOptionsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DescribePipelineOptionsResponse {}

impl ::protobuf::Proxied for DescribePipelineOptionsResponse {
  type View<'msg> = DescribePipelineOptionsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DescribePipelineOptionsResponse {}

impl ::protobuf::MutProxied for DescribePipelineOptionsResponse {
  type Mut<'msg> = DescribePipelineOptionsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DescribePipelineOptionsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DescribePipelineOptionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DescribePipelineOptionsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DescribePipelineOptionsResponseView<'msg> {
  type Message = DescribePipelineOptionsResponse;
}

impl ::std::fmt::Debug for DescribePipelineOptionsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DescribePipelineOptionsResponseView<'_> {
  fn default() -> DescribePipelineOptionsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DescribePipelineOptionsResponse>> for DescribePipelineOptionsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DescribePipelineOptionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DescribePipelineOptionsResponseView<'msg> {

  pub fn to_owned(&self) -> DescribePipelineOptionsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // options: repeated message org.apache.beam.model.job_management.v1.PipelineOptionDescriptor
  pub fn options(self) -> ::protobuf::RepeatedView<'msg, super::PipelineOptionDescriptor> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PipelineOptionDescriptor>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `DescribePipelineOptionsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for DescribePipelineOptionsResponseView<'_> {}

// SAFETY:
// - `DescribePipelineOptionsResponseView` is `Send` because while its alive a `DescribePipelineOptionsResponseMut` cannot.
// - `DescribePipelineOptionsResponseView` does not use thread-local data.
unsafe impl Send for DescribePipelineOptionsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for DescribePipelineOptionsResponseView<'msg> {
  type Proxied = DescribePipelineOptionsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DescribePipelineOptionsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DescribePipelineOptionsResponseView<'msg> {
  fn into_view<'shorter>(self) -> DescribePipelineOptionsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DescribePipelineOptionsResponse> for DescribePipelineOptionsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DescribePipelineOptionsResponse {
    let mut dst = DescribePipelineOptionsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DescribePipelineOptionsResponse> for DescribePipelineOptionsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DescribePipelineOptionsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DescribePipelineOptionsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DescribePipelineOptionsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DescribePipelineOptionsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DescribePipelineOptionsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DescribePipelineOptionsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DescribePipelineOptionsResponseMut<'msg> {
  type Message = DescribePipelineOptionsResponse;
}

impl ::std::fmt::Debug for DescribePipelineOptionsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsResponse>> for DescribePipelineOptionsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DescribePipelineOptionsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DescribePipelineOptionsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> DescribePipelineOptionsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // options: repeated message org.apache.beam.model.job_management.v1.PipelineOptionDescriptor
  pub fn options(&self) -> ::protobuf::RepeatedView<'_, super::PipelineOptionDescriptor> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PipelineOptionDescriptor>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PipelineOptionDescriptor> {
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
  pub fn set_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PipelineOptionDescriptor>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `DescribePipelineOptionsResponseMut` does not perform any shared mutation.
unsafe impl Send for DescribePipelineOptionsResponseMut<'_> {}

// SAFETY:
// - `DescribePipelineOptionsResponseMut` does not perform any shared mutation.
unsafe impl Sync for DescribePipelineOptionsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for DescribePipelineOptionsResponseMut<'msg> {
  type Proxied = DescribePipelineOptionsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DescribePipelineOptionsResponse> {
    DescribePipelineOptionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DescribePipelineOptionsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DescribePipelineOptionsResponse>
  where
      'msg: 'shorter {
    DescribePipelineOptionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DescribePipelineOptionsResponseMut<'msg> {
  type MutProxied = DescribePipelineOptionsResponse;
  fn as_mut(&mut self) -> DescribePipelineOptionsResponseMut<'msg> {
    DescribePipelineOptionsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DescribePipelineOptionsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DescribePipelineOptionsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DescribePipelineOptionsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DescribePipelineOptionsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DescribePipelineOptionsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DescribePipelineOptionsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // options: repeated message org.apache.beam.model.job_management.v1.PipelineOptionDescriptor
  pub fn options(&self) -> ::protobuf::RepeatedView<'_, super::PipelineOptionDescriptor> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PipelineOptionDescriptor>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PipelineOptionDescriptor> {
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
  pub fn set_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PipelineOptionDescriptor>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl DescribePipelineOptionsResponse

impl ::std::ops::Drop for DescribePipelineOptionsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DescribePipelineOptionsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DescribePipelineOptionsResponse {
  type Proxied = Self;
  fn as_view(&self) -> DescribePipelineOptionsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DescribePipelineOptionsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DescribePipelineOptionsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DescribePipelineOptionsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__job_0management__v1__DescribePipelineOptionsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__job_0management__v1__DescribePipelineOptionsResponse_msg_init.0, &[<super::PipelineOptionDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__job_0management__v1__DescribePipelineOptionsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DescribePipelineOptionsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DescribePipelineOptionsResponse {
  type Msg = DescribePipelineOptionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DescribePipelineOptionsResponse {
  type Msg = DescribePipelineOptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DescribePipelineOptionsResponseMut<'_> {
  type Msg = DescribePipelineOptionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DescribePipelineOptionsResponseMut<'_> {
  type Msg = DescribePipelineOptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DescribePipelineOptionsResponseView<'_> {
  type Msg = DescribePipelineOptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DescribePipelineOptionsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DescribePipelineOptionsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



