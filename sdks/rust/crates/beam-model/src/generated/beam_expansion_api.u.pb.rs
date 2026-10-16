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
pub(crate) static mut org__apache__beam__model__expansion__v1__ExpansionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExpansionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExpansionRequest>
}

impl ::protobuf::Message for ExpansionRequest {}

impl ::std::default::Default for ExpansionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExpansionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExpansionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ExpansionRequestMut`.
unsafe impl Sync for ExpansionRequest {}

// SAFETY:
// - `ExpansionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ExpansionRequest {}

impl ::protobuf::Proxied for ExpansionRequest {
  type View<'msg> = ExpansionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExpansionRequest {}

impl ::protobuf::MutProxied for ExpansionRequest {
  type Mut<'msg> = ExpansionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExpansionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpansionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExpansionRequestView<'msg> {
  type Message = ExpansionRequest;
}

impl ::std::fmt::Debug for ExpansionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExpansionRequestView<'_> {
  fn default() -> ExpansionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionRequest>> for ExpansionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpansionRequestView<'msg> {

  pub fn to_owned(&self) -> ExpansionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // components: optional message org.apache.beam.model.pipeline.v1.Components
  pub fn has_components(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn components_opt(self) -> ::protobuf::Optional<super::ComponentsView<'msg>> {
        ::protobuf::Optional::new(self.components(), self.has_components())
  }
  pub fn components(self) -> super::ComponentsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComponentsView::default())
  }

  // transform: optional message org.apache.beam.model.pipeline.v1.PTransform
  pub fn has_transform(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn transform_opt(self) -> ::protobuf::Optional<super::PTransformView<'msg>> {
        ::protobuf::Optional::new(self.transform(), self.has_transform())
  }
  pub fn transform(self) -> super::PTransformView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PTransformView::default())
  }

  // namespace: optional string
  pub fn namespace(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // output_coder_requests: repeated message org.apache.beam.model.expansion.v1.ExpansionRequest.OutputCoderRequestsEntry
  pub fn output_coder_requests(self)
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

  // requirements: repeated string
  pub fn requirements(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn pipeline_options_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'msg>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

}

// SAFETY:
// - `ExpansionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ExpansionRequestView<'_> {}

// SAFETY:
// - `ExpansionRequestView` is `Send` because while its alive a `ExpansionRequestMut` cannot.
// - `ExpansionRequestView` does not use thread-local data.
unsafe impl Send for ExpansionRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ExpansionRequestView<'msg> {
  type Proxied = ExpansionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ExpansionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpansionRequestView<'msg> {
  fn into_view<'shorter>(self) -> ExpansionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpansionRequest> for ExpansionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpansionRequest {
    let mut dst = ExpansionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpansionRequest> for ExpansionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpansionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ExpansionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpansionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpansionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExpansionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpansionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExpansionRequestMut<'msg> {
  type Message = ExpansionRequest;
}

impl ::std::fmt::Debug for ExpansionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionRequest>> for ExpansionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpansionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ExpansionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // components: optional message org.apache.beam.model.pipeline.v1.Components
  pub fn has_components(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_components(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn components_opt(&self) -> ::protobuf::Optional<super::ComponentsView<'_>> {
        ::protobuf::Optional::new(self.components(), self.has_components())
  }
  pub fn components(&self) -> super::ComponentsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComponentsView::default())
  }
  pub fn components_mut(&mut self) -> super::ComponentsMut<'_> {
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
  pub fn set_components(&mut self,
    val: impl ::protobuf::IntoProxied<super::Components>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // transform: optional message org.apache.beam.model.pipeline.v1.PTransform
  pub fn has_transform(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_transform(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn transform_opt(&self) -> ::protobuf::Optional<super::PTransformView<'_>> {
        ::protobuf::Optional::new(self.transform(), self.has_transform())
  }
  pub fn transform(&self) -> super::PTransformView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PTransformView::default())
  }
  pub fn transform_mut(&mut self) -> super::PTransformMut<'_> {
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
  pub fn set_transform(&mut self,
    val: impl ::protobuf::IntoProxied<super::PTransform>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // namespace: optional string
  pub fn namespace(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_namespace(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // output_coder_requests: repeated message org.apache.beam.model.expansion.v1.ExpansionRequest.OutputCoderRequestsEntry
  pub fn output_coder_requests(&self)
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
  pub fn output_coder_requests_mut(&mut self)
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
  pub fn set_output_coder_requests(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // requirements: repeated string
  pub fn requirements(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn requirements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_requirements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}

// SAFETY:
// - `ExpansionRequestMut` does not perform any shared mutation.
unsafe impl Send for ExpansionRequestMut<'_> {}

// SAFETY:
// - `ExpansionRequestMut` does not perform any shared mutation.
unsafe impl Sync for ExpansionRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ExpansionRequestMut<'msg> {
  type Proxied = ExpansionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ExpansionRequest> {
    ExpansionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpansionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExpansionRequest>
  where
      'msg: 'shorter {
    ExpansionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ExpansionRequestMut<'msg> {
  type MutProxied = ExpansionRequest;
  fn as_mut(&mut self) -> ExpansionRequestMut<'msg> {
    ExpansionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExpansionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ExpansionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExpansionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExpansionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExpansionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExpansionRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // components: optional message org.apache.beam.model.pipeline.v1.Components
  pub fn has_components(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_components(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn components_opt(&self) -> ::protobuf::Optional<super::ComponentsView<'_>> {
        ::protobuf::Optional::new(self.components(), self.has_components())
  }
  pub fn components(&self) -> super::ComponentsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComponentsView::default())
  }
  pub fn components_mut(&mut self) -> super::ComponentsMut<'_> {
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
  pub fn set_components(&mut self,
    val: impl ::protobuf::IntoProxied<super::Components>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // transform: optional message org.apache.beam.model.pipeline.v1.PTransform
  pub fn has_transform(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_transform(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn transform_opt(&self) -> ::protobuf::Optional<super::PTransformView<'_>> {
        ::protobuf::Optional::new(self.transform(), self.has_transform())
  }
  pub fn transform(&self) -> super::PTransformView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PTransformView::default())
  }
  pub fn transform_mut(&mut self) -> super::PTransformMut<'_> {
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
  pub fn set_transform(&mut self,
    val: impl ::protobuf::IntoProxied<super::PTransform>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // namespace: optional string
  pub fn namespace(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_namespace(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // output_coder_requests: repeated message org.apache.beam.model.expansion.v1.ExpansionRequest.OutputCoderRequestsEntry
  pub fn output_coder_requests(&self)
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
  pub fn output_coder_requests_mut(&mut self)
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
  pub fn set_output_coder_requests(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // requirements: repeated string
  pub fn requirements(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn requirements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_requirements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // pipeline_options: optional message google.protobuf.Struct
  pub fn has_pipeline_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_pipeline_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn pipeline_options_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::StructView<'_>> {
        ::protobuf::Optional::new(self.pipeline_options(), self.has_pipeline_options())
  }
  pub fn pipeline_options(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn pipeline_options_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_pipeline_options(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl ExpansionRequest

impl ::std::ops::Drop for ExpansionRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExpansionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExpansionRequest {
  type Proxied = Self;
  fn as_view(&self) -> ExpansionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExpansionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExpansionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExpansionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__expansion__v1__ExpansionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331XGET3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__expansion__v1__ExpansionRequest_msg_init.0, &[<super::Components as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::PTransform as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::expansion_request::OutputCoderRequestsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__expansion__v1__ExpansionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpansionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpansionRequest {
  type Msg = ExpansionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionRequest {
  type Msg = ExpansionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpansionRequestMut<'_> {
  type Msg = ExpansionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionRequestMut<'_> {
  type Msg = ExpansionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionRequestView<'_> {
  type Msg = ExpansionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpansionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod expansion_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__expansion__v1__ExpansionRequest__OutputCoderRequestsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct OutputCoderRequestsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OutputCoderRequestsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::expansion_request::org__apache__beam__model__expansion__v1__ExpansionRequest__OutputCoderRequestsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::expansion_request::org__apache__beam__model__expansion__v1__ExpansionRequest__OutputCoderRequestsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::expansion_request::org__apache__beam__model__expansion__v1__ExpansionRequest__OutputCoderRequestsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod expansion_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__expansion__v1__ExpansionResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExpansionResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExpansionResponse>
}

impl ::protobuf::Message for ExpansionResponse {}

impl ::std::default::Default for ExpansionResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExpansionResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExpansionResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ExpansionResponseMut`.
unsafe impl Sync for ExpansionResponse {}

// SAFETY:
// - `ExpansionResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ExpansionResponse {}

impl ::protobuf::Proxied for ExpansionResponse {
  type View<'msg> = ExpansionResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExpansionResponse {}

impl ::protobuf::MutProxied for ExpansionResponse {
  type Mut<'msg> = ExpansionResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExpansionResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpansionResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExpansionResponseView<'msg> {
  type Message = ExpansionResponse;
}

impl ::std::fmt::Debug for ExpansionResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExpansionResponseView<'_> {
  fn default() -> ExpansionResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionResponse>> for ExpansionResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpansionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpansionResponseView<'msg> {

  pub fn to_owned(&self) -> ExpansionResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // components: optional message org.apache.beam.model.pipeline.v1.Components
  pub fn has_components(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn components_opt(self) -> ::protobuf::Optional<super::ComponentsView<'msg>> {
        ::protobuf::Optional::new(self.components(), self.has_components())
  }
  pub fn components(self) -> super::ComponentsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComponentsView::default())
  }

  // transform: optional message org.apache.beam.model.pipeline.v1.PTransform
  pub fn has_transform(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn transform_opt(self) -> ::protobuf::Optional<super::PTransformView<'msg>> {
        ::protobuf::Optional::new(self.transform(), self.has_transform())
  }
  pub fn transform(self) -> super::PTransformView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PTransformView::default())
  }

  // requirements: repeated string
  pub fn requirements(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // error: optional string
  pub fn error(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `ExpansionResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ExpansionResponseView<'_> {}

// SAFETY:
// - `ExpansionResponseView` is `Send` because while its alive a `ExpansionResponseMut` cannot.
// - `ExpansionResponseView` does not use thread-local data.
unsafe impl Send for ExpansionResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ExpansionResponseView<'msg> {
  type Proxied = ExpansionResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ExpansionResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpansionResponseView<'msg> {
  fn into_view<'shorter>(self) -> ExpansionResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpansionResponse> for ExpansionResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpansionResponse {
    let mut dst = ExpansionResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpansionResponse> for ExpansionResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpansionResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ExpansionResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpansionResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpansionResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExpansionResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpansionResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExpansionResponseMut<'msg> {
  type Message = ExpansionResponse;
}

impl ::std::fmt::Debug for ExpansionResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionResponse>> for ExpansionResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpansionResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpansionResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ExpansionResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // components: optional message org.apache.beam.model.pipeline.v1.Components
  pub fn has_components(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_components(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn components_opt(&self) -> ::protobuf::Optional<super::ComponentsView<'_>> {
        ::protobuf::Optional::new(self.components(), self.has_components())
  }
  pub fn components(&self) -> super::ComponentsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComponentsView::default())
  }
  pub fn components_mut(&mut self) -> super::ComponentsMut<'_> {
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
  pub fn set_components(&mut self,
    val: impl ::protobuf::IntoProxied<super::Components>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // transform: optional message org.apache.beam.model.pipeline.v1.PTransform
  pub fn has_transform(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_transform(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn transform_opt(&self) -> ::protobuf::Optional<super::PTransformView<'_>> {
        ::protobuf::Optional::new(self.transform(), self.has_transform())
  }
  pub fn transform(&self) -> super::PTransformView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PTransformView::default())
  }
  pub fn transform_mut(&mut self) -> super::PTransformMut<'_> {
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
  pub fn set_transform(&mut self,
    val: impl ::protobuf::IntoProxied<super::PTransform>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // requirements: repeated string
  pub fn requirements(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn requirements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_requirements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // error: optional string
  pub fn error(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `ExpansionResponseMut` does not perform any shared mutation.
unsafe impl Send for ExpansionResponseMut<'_> {}

// SAFETY:
// - `ExpansionResponseMut` does not perform any shared mutation.
unsafe impl Sync for ExpansionResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ExpansionResponseMut<'msg> {
  type Proxied = ExpansionResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ExpansionResponse> {
    ExpansionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpansionResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExpansionResponse>
  where
      'msg: 'shorter {
    ExpansionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ExpansionResponseMut<'msg> {
  type MutProxied = ExpansionResponse;
  fn as_mut(&mut self) -> ExpansionResponseMut<'msg> {
    ExpansionResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExpansionResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ExpansionResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExpansionResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExpansionResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExpansionResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExpansionResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // components: optional message org.apache.beam.model.pipeline.v1.Components
  pub fn has_components(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_components(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn components_opt(&self) -> ::protobuf::Optional<super::ComponentsView<'_>> {
        ::protobuf::Optional::new(self.components(), self.has_components())
  }
  pub fn components(&self) -> super::ComponentsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComponentsView::default())
  }
  pub fn components_mut(&mut self) -> super::ComponentsMut<'_> {
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
  pub fn set_components(&mut self,
    val: impl ::protobuf::IntoProxied<super::Components>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // transform: optional message org.apache.beam.model.pipeline.v1.PTransform
  pub fn has_transform(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_transform(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn transform_opt(&self) -> ::protobuf::Optional<super::PTransformView<'_>> {
        ::protobuf::Optional::new(self.transform(), self.has_transform())
  }
  pub fn transform(&self) -> super::PTransformView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PTransformView::default())
  }
  pub fn transform_mut(&mut self) -> super::PTransformMut<'_> {
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
  pub fn set_transform(&mut self,
    val: impl ::protobuf::IntoProxied<super::PTransform>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // requirements: repeated string
  pub fn requirements(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn requirements_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_requirements(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // error: optional string
  pub fn error(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl ExpansionResponse

impl ::std::ops::Drop for ExpansionResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExpansionResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExpansionResponse {
  type Proxied = Self;
  fn as_view(&self) -> ExpansionResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExpansionResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExpansionResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExpansionResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__expansion__v1__ExpansionResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33ETf1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__expansion__v1__ExpansionResponse_msg_init.0, &[<super::Components as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::PTransform as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__expansion__v1__ExpansionResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpansionResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpansionResponse {
  type Msg = ExpansionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionResponse {
  type Msg = ExpansionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpansionResponseMut<'_> {
  type Msg = ExpansionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionResponseMut<'_> {
  type Msg = ExpansionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpansionResponseView<'_> {
  type Msg = ExpansionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpansionResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpansionResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__expansion__v1__DiscoverSchemaTransformRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DiscoverSchemaTransformRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DiscoverSchemaTransformRequest>
}

impl ::protobuf::Message for DiscoverSchemaTransformRequest {}

impl ::std::default::Default for DiscoverSchemaTransformRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DiscoverSchemaTransformRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DiscoverSchemaTransformRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DiscoverSchemaTransformRequestMut`.
unsafe impl Sync for DiscoverSchemaTransformRequest {}

// SAFETY:
// - `DiscoverSchemaTransformRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DiscoverSchemaTransformRequest {}

impl ::protobuf::Proxied for DiscoverSchemaTransformRequest {
  type View<'msg> = DiscoverSchemaTransformRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DiscoverSchemaTransformRequest {}

impl ::protobuf::MutProxied for DiscoverSchemaTransformRequest {
  type Mut<'msg> = DiscoverSchemaTransformRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DiscoverSchemaTransformRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoverSchemaTransformRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoverSchemaTransformRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DiscoverSchemaTransformRequestView<'msg> {
  type Message = DiscoverSchemaTransformRequest;
}

impl ::std::fmt::Debug for DiscoverSchemaTransformRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DiscoverSchemaTransformRequestView<'_> {
  fn default() -> DiscoverSchemaTransformRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoverSchemaTransformRequest>> for DiscoverSchemaTransformRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoverSchemaTransformRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoverSchemaTransformRequestView<'msg> {

  pub fn to_owned(&self) -> DiscoverSchemaTransformRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `DiscoverSchemaTransformRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DiscoverSchemaTransformRequestView<'_> {}

// SAFETY:
// - `DiscoverSchemaTransformRequestView` is `Send` because while its alive a `DiscoverSchemaTransformRequestMut` cannot.
// - `DiscoverSchemaTransformRequestView` does not use thread-local data.
unsafe impl Send for DiscoverSchemaTransformRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DiscoverSchemaTransformRequestView<'msg> {
  type Proxied = DiscoverSchemaTransformRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DiscoverSchemaTransformRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoverSchemaTransformRequestView<'msg> {
  fn into_view<'shorter>(self) -> DiscoverSchemaTransformRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoverSchemaTransformRequest> for DiscoverSchemaTransformRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoverSchemaTransformRequest {
    let mut dst = DiscoverSchemaTransformRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoverSchemaTransformRequest> for DiscoverSchemaTransformRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoverSchemaTransformRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DiscoverSchemaTransformRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DiscoverSchemaTransformRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DiscoverSchemaTransformRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DiscoverSchemaTransformRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoverSchemaTransformRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DiscoverSchemaTransformRequestMut<'msg> {
  type Message = DiscoverSchemaTransformRequest;
}

impl ::std::fmt::Debug for DiscoverSchemaTransformRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformRequest>> for DiscoverSchemaTransformRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoverSchemaTransformRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DiscoverSchemaTransformRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `DiscoverSchemaTransformRequestMut` does not perform any shared mutation.
unsafe impl Send for DiscoverSchemaTransformRequestMut<'_> {}

// SAFETY:
// - `DiscoverSchemaTransformRequestMut` does not perform any shared mutation.
unsafe impl Sync for DiscoverSchemaTransformRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DiscoverSchemaTransformRequestMut<'msg> {
  type Proxied = DiscoverSchemaTransformRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DiscoverSchemaTransformRequest> {
    DiscoverSchemaTransformRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoverSchemaTransformRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DiscoverSchemaTransformRequest>
  where
      'msg: 'shorter {
    DiscoverSchemaTransformRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DiscoverSchemaTransformRequestMut<'msg> {
  type MutProxied = DiscoverSchemaTransformRequest;
  fn as_mut(&mut self) -> DiscoverSchemaTransformRequestMut<'msg> {
    DiscoverSchemaTransformRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DiscoverSchemaTransformRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DiscoverSchemaTransformRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DiscoverSchemaTransformRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DiscoverSchemaTransformRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DiscoverSchemaTransformRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DiscoverSchemaTransformRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl DiscoverSchemaTransformRequest

impl ::std::ops::Drop for DiscoverSchemaTransformRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DiscoverSchemaTransformRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DiscoverSchemaTransformRequest {
  type Proxied = Self;
  fn as_view(&self) -> DiscoverSchemaTransformRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DiscoverSchemaTransformRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DiscoverSchemaTransformRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DiscoverSchemaTransformRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoverSchemaTransformRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoverSchemaTransformRequest {
  type Msg = DiscoverSchemaTransformRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoverSchemaTransformRequest {
  type Msg = DiscoverSchemaTransformRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoverSchemaTransformRequestMut<'_> {
  type Msg = DiscoverSchemaTransformRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoverSchemaTransformRequestMut<'_> {
  type Msg = DiscoverSchemaTransformRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoverSchemaTransformRequestView<'_> {
  type Msg = DiscoverSchemaTransformRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoverSchemaTransformRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__expansion__v1__SchemaTransformConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SchemaTransformConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SchemaTransformConfig>
}

impl ::protobuf::Message for SchemaTransformConfig {}

impl ::std::default::Default for SchemaTransformConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SchemaTransformConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SchemaTransformConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `SchemaTransformConfigMut`.
unsafe impl Sync for SchemaTransformConfig {}

// SAFETY:
// - `SchemaTransformConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SchemaTransformConfig {}

impl ::protobuf::Proxied for SchemaTransformConfig {
  type View<'msg> = SchemaTransformConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SchemaTransformConfig {}

impl ::protobuf::MutProxied for SchemaTransformConfig {
  type Mut<'msg> = SchemaTransformConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SchemaTransformConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaTransformConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaTransformConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SchemaTransformConfigView<'msg> {
  type Message = SchemaTransformConfig;
}

impl ::std::fmt::Debug for SchemaTransformConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SchemaTransformConfigView<'_> {
  fn default() -> SchemaTransformConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaTransformConfig>> for SchemaTransformConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaTransformConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaTransformConfigView<'msg> {

  pub fn to_owned(&self) -> SchemaTransformConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_config_schema(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn config_schema_opt(self) -> ::protobuf::Optional<super::SchemaView<'msg>> {
        ::protobuf::Optional::new(self.config_schema(), self.has_config_schema())
  }
  pub fn config_schema(self) -> super::SchemaView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }

  // input_pcollection_names: repeated string
  pub fn input_pcollection_names(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // output_pcollection_names: repeated string
  pub fn output_pcollection_names(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // description: optional string
  pub fn description(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `SchemaTransformConfigView` is `Sync` because it does not support mutation.
unsafe impl Sync for SchemaTransformConfigView<'_> {}

// SAFETY:
// - `SchemaTransformConfigView` is `Send` because while its alive a `SchemaTransformConfigMut` cannot.
// - `SchemaTransformConfigView` does not use thread-local data.
unsafe impl Send for SchemaTransformConfigView<'_> {}

impl<'msg> ::protobuf::AsView for SchemaTransformConfigView<'msg> {
  type Proxied = SchemaTransformConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, SchemaTransformConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaTransformConfigView<'msg> {
  fn into_view<'shorter>(self) -> SchemaTransformConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaTransformConfig> for SchemaTransformConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaTransformConfig {
    let mut dst = SchemaTransformConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaTransformConfig> for SchemaTransformConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaTransformConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SchemaTransformConfig {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaTransformConfigView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaTransformConfigMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SchemaTransformConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaTransformConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SchemaTransformConfigMut<'msg> {
  type Message = SchemaTransformConfig;
}

impl ::std::fmt::Debug for SchemaTransformConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformConfig>> for SchemaTransformConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaTransformConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaTransformConfig> {
    self.inner
  }

  pub fn to_owned(&self) -> SchemaTransformConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // config_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_config_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.config_schema(), self.has_config_schema())
  }
  pub fn config_schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn config_schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_config_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // input_pcollection_names: repeated string
  pub fn input_pcollection_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn input_pcollection_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_input_pcollection_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // output_pcollection_names: repeated string
  pub fn output_pcollection_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn output_pcollection_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_output_pcollection_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `SchemaTransformConfigMut` does not perform any shared mutation.
unsafe impl Send for SchemaTransformConfigMut<'_> {}

// SAFETY:
// - `SchemaTransformConfigMut` does not perform any shared mutation.
unsafe impl Sync for SchemaTransformConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for SchemaTransformConfigMut<'msg> {
  type Proxied = SchemaTransformConfig;
  fn as_view(&self) -> ::protobuf::View<'_, SchemaTransformConfig> {
    SchemaTransformConfigView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaTransformConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SchemaTransformConfig>
  where
      'msg: 'shorter {
    SchemaTransformConfigView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SchemaTransformConfigMut<'msg> {
  type MutProxied = SchemaTransformConfig;
  fn as_mut(&mut self) -> SchemaTransformConfigMut<'msg> {
    SchemaTransformConfigMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SchemaTransformConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> SchemaTransformConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SchemaTransformConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SchemaTransformConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SchemaTransformConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SchemaTransformConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // config_schema: optional message org.apache.beam.model.pipeline.v1.Schema
  pub fn has_config_schema(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config_schema(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_schema_opt(&self) -> ::protobuf::Optional<super::SchemaView<'_>> {
        ::protobuf::Optional::new(self.config_schema(), self.has_config_schema())
  }
  pub fn config_schema(&self) -> super::SchemaView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaView::default())
  }
  pub fn config_schema_mut(&mut self) -> super::SchemaMut<'_> {
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
  pub fn set_config_schema(&mut self,
    val: impl ::protobuf::IntoProxied<super::Schema>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // input_pcollection_names: repeated string
  pub fn input_pcollection_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn input_pcollection_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_input_pcollection_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // output_pcollection_names: repeated string
  pub fn output_pcollection_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn output_pcollection_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_output_pcollection_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl SchemaTransformConfig

impl ::std::ops::Drop for SchemaTransformConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SchemaTransformConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SchemaTransformConfig {
  type Proxied = Self;
  fn as_view(&self) -> SchemaTransformConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SchemaTransformConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SchemaTransformConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SchemaTransformConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__expansion__v1__SchemaTransformConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3ETET1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__expansion__v1__SchemaTransformConfig_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__expansion__v1__SchemaTransformConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaTransformConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaTransformConfig {
  type Msg = SchemaTransformConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaTransformConfig {
  type Msg = SchemaTransformConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaTransformConfigMut<'_> {
  type Msg = SchemaTransformConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaTransformConfigMut<'_> {
  type Msg = SchemaTransformConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaTransformConfigView<'_> {
  type Msg = SchemaTransformConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaTransformConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaTransformConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DiscoverSchemaTransformResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DiscoverSchemaTransformResponse>
}

impl ::protobuf::Message for DiscoverSchemaTransformResponse {}

impl ::std::default::Default for DiscoverSchemaTransformResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DiscoverSchemaTransformResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DiscoverSchemaTransformResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DiscoverSchemaTransformResponseMut`.
unsafe impl Sync for DiscoverSchemaTransformResponse {}

// SAFETY:
// - `DiscoverSchemaTransformResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DiscoverSchemaTransformResponse {}

impl ::protobuf::Proxied for DiscoverSchemaTransformResponse {
  type View<'msg> = DiscoverSchemaTransformResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DiscoverSchemaTransformResponse {}

impl ::protobuf::MutProxied for DiscoverSchemaTransformResponse {
  type Mut<'msg> = DiscoverSchemaTransformResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DiscoverSchemaTransformResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoverSchemaTransformResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoverSchemaTransformResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DiscoverSchemaTransformResponseView<'msg> {
  type Message = DiscoverSchemaTransformResponse;
}

impl ::std::fmt::Debug for DiscoverSchemaTransformResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DiscoverSchemaTransformResponseView<'_> {
  fn default() -> DiscoverSchemaTransformResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoverSchemaTransformResponse>> for DiscoverSchemaTransformResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoverSchemaTransformResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoverSchemaTransformResponseView<'msg> {

  pub fn to_owned(&self) -> DiscoverSchemaTransformResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // schema_transform_configs: repeated message org.apache.beam.model.expansion.v1.DiscoverSchemaTransformResponse.SchemaTransformConfigsEntry
  pub fn schema_transform_configs(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, super::SchemaTransformConfig> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::SchemaTransformConfig>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // error: optional string
  pub fn error(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `DiscoverSchemaTransformResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for DiscoverSchemaTransformResponseView<'_> {}

// SAFETY:
// - `DiscoverSchemaTransformResponseView` is `Send` because while its alive a `DiscoverSchemaTransformResponseMut` cannot.
// - `DiscoverSchemaTransformResponseView` does not use thread-local data.
unsafe impl Send for DiscoverSchemaTransformResponseView<'_> {}

impl<'msg> ::protobuf::AsView for DiscoverSchemaTransformResponseView<'msg> {
  type Proxied = DiscoverSchemaTransformResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DiscoverSchemaTransformResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoverSchemaTransformResponseView<'msg> {
  fn into_view<'shorter>(self) -> DiscoverSchemaTransformResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoverSchemaTransformResponse> for DiscoverSchemaTransformResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoverSchemaTransformResponse {
    let mut dst = DiscoverSchemaTransformResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoverSchemaTransformResponse> for DiscoverSchemaTransformResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoverSchemaTransformResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DiscoverSchemaTransformResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DiscoverSchemaTransformResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DiscoverSchemaTransformResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DiscoverSchemaTransformResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoverSchemaTransformResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DiscoverSchemaTransformResponseMut<'msg> {
  type Message = DiscoverSchemaTransformResponse;
}

impl ::std::fmt::Debug for DiscoverSchemaTransformResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformResponse>> for DiscoverSchemaTransformResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoverSchemaTransformResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoverSchemaTransformResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> DiscoverSchemaTransformResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // schema_transform_configs: repeated message org.apache.beam.model.expansion.v1.DiscoverSchemaTransformResponse.SchemaTransformConfigsEntry
  pub fn schema_transform_configs(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::SchemaTransformConfig> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::SchemaTransformConfig>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn schema_transform_configs_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::SchemaTransformConfig> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_schema_transform_configs(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::SchemaTransformConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // error: optional string
  pub fn error(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `DiscoverSchemaTransformResponseMut` does not perform any shared mutation.
unsafe impl Send for DiscoverSchemaTransformResponseMut<'_> {}

// SAFETY:
// - `DiscoverSchemaTransformResponseMut` does not perform any shared mutation.
unsafe impl Sync for DiscoverSchemaTransformResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for DiscoverSchemaTransformResponseMut<'msg> {
  type Proxied = DiscoverSchemaTransformResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DiscoverSchemaTransformResponse> {
    DiscoverSchemaTransformResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoverSchemaTransformResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DiscoverSchemaTransformResponse>
  where
      'msg: 'shorter {
    DiscoverSchemaTransformResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DiscoverSchemaTransformResponseMut<'msg> {
  type MutProxied = DiscoverSchemaTransformResponse;
  fn as_mut(&mut self) -> DiscoverSchemaTransformResponseMut<'msg> {
    DiscoverSchemaTransformResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DiscoverSchemaTransformResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DiscoverSchemaTransformResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DiscoverSchemaTransformResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DiscoverSchemaTransformResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DiscoverSchemaTransformResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DiscoverSchemaTransformResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // schema_transform_configs: repeated message org.apache.beam.model.expansion.v1.DiscoverSchemaTransformResponse.SchemaTransformConfigsEntry
  pub fn schema_transform_configs(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::SchemaTransformConfig> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::SchemaTransformConfig>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn schema_transform_configs_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::SchemaTransformConfig> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_schema_transform_configs(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::SchemaTransformConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // error: optional string
  pub fn error(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl DiscoverSchemaTransformResponse

impl ::std::ops::Drop for DiscoverSchemaTransformResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DiscoverSchemaTransformResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DiscoverSchemaTransformResponse {
  type Proxied = Self;
  fn as_view(&self) -> DiscoverSchemaTransformResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DiscoverSchemaTransformResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DiscoverSchemaTransformResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DiscoverSchemaTransformResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse_msg_init.0, &[<super::discover_schema_transform_response::SchemaTransformConfigsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoverSchemaTransformResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoverSchemaTransformResponse {
  type Msg = DiscoverSchemaTransformResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoverSchemaTransformResponse {
  type Msg = DiscoverSchemaTransformResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoverSchemaTransformResponseMut<'_> {
  type Msg = DiscoverSchemaTransformResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoverSchemaTransformResponseMut<'_> {
  type Msg = DiscoverSchemaTransformResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoverSchemaTransformResponseView<'_> {
  type Msg = DiscoverSchemaTransformResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoverSchemaTransformResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoverSchemaTransformResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod discover_schema_transform_response {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse__SchemaTransformConfigsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct SchemaTransformConfigsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SchemaTransformConfigsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::discover_schema_transform_response::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse__SchemaTransformConfigsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::discover_schema_transform_response::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse__SchemaTransformConfigsEntry_msg_init.0, &[<super::super::SchemaTransformConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::discover_schema_transform_response::org__apache__beam__model__expansion__v1__DiscoverSchemaTransformResponse__SchemaTransformConfigsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod discover_schema_transform_response


