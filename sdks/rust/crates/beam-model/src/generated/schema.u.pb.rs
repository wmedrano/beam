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
pub(crate) static mut org__apache__beam__model__pipeline__v1__Schema_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Schema {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Schema>
}

impl ::protobuf::Message for Schema {}

impl ::std::default::Default for Schema {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Schema {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Schema` is `Sync` because it does not implement interior mutability.
//    Neither does `SchemaMut`.
unsafe impl Sync for Schema {}

// SAFETY:
// - `Schema` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Schema {}

impl ::protobuf::Proxied for Schema {
  type View<'msg> = SchemaView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Schema {}

impl ::protobuf::MutProxied for Schema {
  type Mut<'msg> = SchemaMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SchemaView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Schema>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SchemaView<'msg> {
  type Message = Schema;
}

impl ::std::fmt::Debug for SchemaView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SchemaView<'_> {
  fn default() -> SchemaView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Schema>> for SchemaView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Schema>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaView<'msg> {

  pub fn to_owned(&self) -> Schema {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fields: repeated message org.apache.beam.model.pipeline.v1.Field
  pub fn fields(self) -> ::protobuf::RepeatedView<'msg, super::Field> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Field>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // id: optional string
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // options: repeated message org.apache.beam.model.pipeline.v1.Option
  pub fn options(self) -> ::protobuf::RepeatedView<'msg, super::Option> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Option>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // encoding_positions_set: optional bool
  pub fn encoding_positions_set(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SchemaView` is `Sync` because it does not support mutation.
unsafe impl Sync for SchemaView<'_> {}

// SAFETY:
// - `SchemaView` is `Send` because while its alive a `SchemaMut` cannot.
// - `SchemaView` does not use thread-local data.
unsafe impl Send for SchemaView<'_> {}

impl<'msg> ::protobuf::AsView for SchemaView<'msg> {
  type Proxied = Schema;
  fn as_view(&self) -> ::protobuf::View<'msg, Schema> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaView<'msg> {
  fn into_view<'shorter>(self) -> SchemaView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Schema> for SchemaView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Schema {
    let mut dst = Schema::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Schema> for SchemaMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Schema {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Schema {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SchemaMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Schema>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SchemaMut<'msg> {
  type Message = Schema;
}

impl ::std::fmt::Debug for SchemaMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Schema>> for SchemaMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Schema>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Schema> {
    self.inner
  }

  pub fn to_owned(&self) -> Schema {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fields: repeated message org.apache.beam.model.pipeline.v1.Field
  pub fn fields(&self) -> ::protobuf::RepeatedView<'_, super::Field> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Field>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fields_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Field> {
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
  pub fn set_fields(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Field>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // options: repeated message org.apache.beam.model.pipeline.v1.Option
  pub fn options(&self) -> ::protobuf::RepeatedView<'_, super::Option> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Option>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Option> {
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
  pub fn set_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Option>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // encoding_positions_set: optional bool
  pub fn encoding_positions_set(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding_positions_set(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `SchemaMut` does not perform any shared mutation.
unsafe impl Send for SchemaMut<'_> {}

// SAFETY:
// - `SchemaMut` does not perform any shared mutation.
unsafe impl Sync for SchemaMut<'_> {}

impl<'msg> ::protobuf::AsView for SchemaMut<'msg> {
  type Proxied = Schema;
  fn as_view(&self) -> ::protobuf::View<'_, Schema> {
    SchemaView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Schema>
  where
      'msg: 'shorter {
    SchemaView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SchemaMut<'msg> {
  type MutProxied = Schema;
  fn as_mut(&mut self) -> SchemaMut<'msg> {
    SchemaMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SchemaMut<'msg> {
  fn into_mut<'shorter>(self) -> SchemaMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Schema {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Schema> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SchemaView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SchemaMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fields: repeated message org.apache.beam.model.pipeline.v1.Field
  pub fn fields(&self) -> ::protobuf::RepeatedView<'_, super::Field> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Field>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fields_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Field> {
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
  pub fn set_fields(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Field>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // options: repeated message org.apache.beam.model.pipeline.v1.Option
  pub fn options(&self) -> ::protobuf::RepeatedView<'_, super::Option> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Option>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Option> {
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
  pub fn set_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Option>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // encoding_positions_set: optional bool
  pub fn encoding_positions_set(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding_positions_set(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}  // impl Schema

impl ::std::ops::Drop for Schema {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Schema {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Schema {
  type Proxied = Self;
  fn as_view(&self) -> SchemaView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Schema {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SchemaMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Schema {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__Schema_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Schema {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Schema {
  type Msg = Schema;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Schema {
  type Msg = Schema;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaMut<'_> {
  type Msg = Schema;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaMut<'_> {
  type Msg = Schema;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaView<'_> {
  type Msg = Schema;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Schema> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__Field_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Field {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Field>
}

impl ::protobuf::Message for Field {}

impl ::std::default::Default for Field {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Field {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Field` is `Sync` because it does not implement interior mutability.
//    Neither does `FieldMut`.
unsafe impl Sync for Field {}

// SAFETY:
// - `Field` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Field {}

impl ::protobuf::Proxied for Field {
  type View<'msg> = FieldView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Field {}

impl ::protobuf::MutProxied for Field {
  type Mut<'msg> = FieldMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FieldView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Field>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FieldView<'msg> {
  type Message = Field;
}

impl ::std::fmt::Debug for FieldView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FieldView<'_> {
  fn default() -> FieldView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Field>> for FieldView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Field>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldView<'msg> {

  pub fn to_owned(&self) -> Field {
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

  // description: optional string
  pub fn description(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn type_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.r#type(), self.has_type())
  }
  pub fn r#type(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

  // id: optional int32
  pub fn id(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // encoding_position: optional int32
  pub fn encoding_position(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // options: repeated message org.apache.beam.model.pipeline.v1.Option
  pub fn options(self) -> ::protobuf::RepeatedView<'msg, super::Option> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Option>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `FieldView` is `Sync` because it does not support mutation.
unsafe impl Sync for FieldView<'_> {}

// SAFETY:
// - `FieldView` is `Send` because while its alive a `FieldMut` cannot.
// - `FieldView` does not use thread-local data.
unsafe impl Send for FieldView<'_> {}

impl<'msg> ::protobuf::AsView for FieldView<'msg> {
  type Proxied = Field;
  fn as_view(&self) -> ::protobuf::View<'msg, Field> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldView<'msg> {
  fn into_view<'shorter>(self) -> FieldView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Field> for FieldView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Field {
    let mut dst = Field::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Field> for FieldMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Field {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Field {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FieldView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FieldMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FieldMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Field>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FieldMut<'msg> {
  type Message = Field;
}

impl ::std::fmt::Debug for FieldMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Field>> for FieldMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Field>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Field> {
    self.inner
  }

  pub fn to_owned(&self) -> Field {
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

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.r#type(), self.has_type())
  }
  pub fn r#type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // id: optional int32
  pub fn id(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i32) {
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

  // encoding_position: optional int32
  pub fn encoding_position(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding_position(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // options: repeated message org.apache.beam.model.pipeline.v1.Option
  pub fn options(&self) -> ::protobuf::RepeatedView<'_, super::Option> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Option>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Option> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Option>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}

// SAFETY:
// - `FieldMut` does not perform any shared mutation.
unsafe impl Send for FieldMut<'_> {}

// SAFETY:
// - `FieldMut` does not perform any shared mutation.
unsafe impl Sync for FieldMut<'_> {}

impl<'msg> ::protobuf::AsView for FieldMut<'msg> {
  type Proxied = Field;
  fn as_view(&self) -> ::protobuf::View<'_, Field> {
    FieldView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Field>
  where
      'msg: 'shorter {
    FieldView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for FieldMut<'msg> {
  type MutProxied = Field;
  fn as_mut(&mut self) -> FieldMut<'msg> {
    FieldMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FieldMut<'msg> {
  fn into_mut<'shorter>(self) -> FieldMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Field {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Field> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FieldView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FieldMut<'_> {
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

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.r#type(), self.has_type())
  }
  pub fn r#type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // id: optional int32
  pub fn id(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: i32) {
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

  // encoding_position: optional int32
  pub fn encoding_position(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding_position(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // options: repeated message org.apache.beam.model.pipeline.v1.Option
  pub fn options(&self) -> ::protobuf::RepeatedView<'_, super::Option> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Option>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Option> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Option>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}  // impl Field

impl ::std::ops::Drop for Field {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Field {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Field {
  type Proxied = Self;
  fn as_view(&self) -> FieldView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Field {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FieldMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Field {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__Field_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Field {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Field {
  type Msg = Field;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Field> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Field {
  type Msg = Field;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Field> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldMut<'_> {
  type Msg = Field;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Field> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldMut<'_> {
  type Msg = Field;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Field> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldView<'_> {
  type Msg = Field;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Field> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__FieldType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FieldType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FieldType>
}

impl ::protobuf::Message for FieldType {}

impl ::std::default::Default for FieldType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FieldType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FieldType` is `Sync` because it does not implement interior mutability.
//    Neither does `FieldTypeMut`.
unsafe impl Sync for FieldType {}

// SAFETY:
// - `FieldType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for FieldType {}

impl ::protobuf::Proxied for FieldType {
  type View<'msg> = FieldTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FieldType {}

impl ::protobuf::MutProxied for FieldType {
  type Mut<'msg> = FieldTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FieldTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FieldTypeView<'msg> {
  type Message = FieldType;
}

impl ::std::fmt::Debug for FieldTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FieldTypeView<'_> {
  fn default() -> FieldTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FieldType>> for FieldTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldTypeView<'msg> {

  pub fn to_owned(&self) -> FieldType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // nullable: optional bool
  pub fn nullable(self) -> bool {
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

  // atomic_type: optional enum org.apache.beam.model.pipeline.v1.AtomicType
  pub fn has_atomic_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn atomic_type_opt(self) -> ::protobuf::Optional<super::AtomicType> {
        ::protobuf::Optional::new(self.atomic_type(), self.has_atomic_type())
  }
  pub fn atomic_type(self) -> super::AtomicType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::AtomicType::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // array_type: optional message org.apache.beam.model.pipeline.v1.ArrayType
  pub fn has_array_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn array_type_opt(self) -> ::protobuf::Optional<super::ArrayTypeView<'msg>> {
        ::protobuf::Optional::new(self.array_type(), self.has_array_type())
  }
  pub fn array_type(self) -> super::ArrayTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArrayTypeView::default())
  }

  // iterable_type: optional message org.apache.beam.model.pipeline.v1.IterableType
  pub fn has_iterable_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn iterable_type_opt(self) -> ::protobuf::Optional<super::IterableTypeView<'msg>> {
        ::protobuf::Optional::new(self.iterable_type(), self.has_iterable_type())
  }
  pub fn iterable_type(self) -> super::IterableTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::IterableTypeView::default())
  }

  // map_type: optional message org.apache.beam.model.pipeline.v1.MapType
  pub fn has_map_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn map_type_opt(self) -> ::protobuf::Optional<super::MapTypeView<'msg>> {
        ::protobuf::Optional::new(self.map_type(), self.has_map_type())
  }
  pub fn map_type(self) -> super::MapTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapTypeView::default())
  }

  // row_type: optional message org.apache.beam.model.pipeline.v1.RowType
  pub fn has_row_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn row_type_opt(self) -> ::protobuf::Optional<super::RowTypeView<'msg>> {
        ::protobuf::Optional::new(self.row_type(), self.has_row_type())
  }
  pub fn row_type(self) -> super::RowTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RowTypeView::default())
  }

  // logical_type: optional message org.apache.beam.model.pipeline.v1.LogicalType
  pub fn has_logical_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn logical_type_opt(self) -> ::protobuf::Optional<super::LogicalTypeView<'msg>> {
        ::protobuf::Optional::new(self.logical_type(), self.has_logical_type())
  }
  pub fn logical_type(self) -> super::LogicalTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogicalTypeView::default())
  }

  pub fn type_info(self) -> super::field_type::TypeInfoOneof<'msg> {
    match self.type_info_case() {
      super::field_type::TypeInfoCase::AtomicType =>
          super::field_type::TypeInfoOneof::AtomicType(self.atomic_type()),
      super::field_type::TypeInfoCase::ArrayType =>
          super::field_type::TypeInfoOneof::ArrayType(self.array_type()),
      super::field_type::TypeInfoCase::IterableType =>
          super::field_type::TypeInfoOneof::IterableType(self.iterable_type()),
      super::field_type::TypeInfoCase::MapType =>
          super::field_type::TypeInfoOneof::MapType(self.map_type()),
      super::field_type::TypeInfoCase::RowType =>
          super::field_type::TypeInfoOneof::RowType(self.row_type()),
      super::field_type::TypeInfoCase::LogicalType =>
          super::field_type::TypeInfoOneof::LogicalType(self.logical_type()),
      _ => super::field_type::TypeInfoOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn type_info_case(self) -> super::field_type::TypeInfoCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::field_type::TypeInfoCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FieldTypeView` is `Sync` because it does not support mutation.
unsafe impl Sync for FieldTypeView<'_> {}

// SAFETY:
// - `FieldTypeView` is `Send` because while its alive a `FieldTypeMut` cannot.
// - `FieldTypeView` does not use thread-local data.
unsafe impl Send for FieldTypeView<'_> {}

impl<'msg> ::protobuf::AsView for FieldTypeView<'msg> {
  type Proxied = FieldType;
  fn as_view(&self) -> ::protobuf::View<'msg, FieldType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldTypeView<'msg> {
  fn into_view<'shorter>(self) -> FieldTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldType> for FieldTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldType {
    let mut dst = FieldType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldType> for FieldTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for FieldType {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FieldTypeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FieldTypeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FieldTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FieldTypeMut<'msg> {
  type Message = FieldType;
}

impl ::std::fmt::Debug for FieldTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FieldType>> for FieldTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldType> {
    self.inner
  }

  pub fn to_owned(&self) -> FieldType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // nullable: optional bool
  pub fn nullable(&self) -> bool {
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
  pub fn set_nullable(&mut self, val: bool) {
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

  // atomic_type: optional enum org.apache.beam.model.pipeline.v1.AtomicType
  pub fn has_atomic_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_atomic_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn atomic_type_opt(&self) -> ::protobuf::Optional<super::AtomicType> {
        ::protobuf::Optional::new(self.atomic_type(), self.has_atomic_type())
  }
  pub fn atomic_type(&self) -> super::AtomicType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::AtomicType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_atomic_type(&mut self, val: super::AtomicType) {
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

  // array_type: optional message org.apache.beam.model.pipeline.v1.ArrayType
  pub fn has_array_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_array_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn array_type_opt(&self) -> ::protobuf::Optional<super::ArrayTypeView<'_>> {
        ::protobuf::Optional::new(self.array_type(), self.has_array_type())
  }
  pub fn array_type(&self) -> super::ArrayTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArrayTypeView::default())
  }
  pub fn array_type_mut(&mut self) -> super::ArrayTypeMut<'_> {
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
  pub fn set_array_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArrayType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // iterable_type: optional message org.apache.beam.model.pipeline.v1.IterableType
  pub fn has_iterable_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_iterable_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn iterable_type_opt(&self) -> ::protobuf::Optional<super::IterableTypeView<'_>> {
        ::protobuf::Optional::new(self.iterable_type(), self.has_iterable_type())
  }
  pub fn iterable_type(&self) -> super::IterableTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::IterableTypeView::default())
  }
  pub fn iterable_type_mut(&mut self) -> super::IterableTypeMut<'_> {
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
  pub fn set_iterable_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::IterableType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // map_type: optional message org.apache.beam.model.pipeline.v1.MapType
  pub fn has_map_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_map_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn map_type_opt(&self) -> ::protobuf::Optional<super::MapTypeView<'_>> {
        ::protobuf::Optional::new(self.map_type(), self.has_map_type())
  }
  pub fn map_type(&self) -> super::MapTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapTypeView::default())
  }
  pub fn map_type_mut(&mut self) -> super::MapTypeMut<'_> {
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
  pub fn set_map_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::MapType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // row_type: optional message org.apache.beam.model.pipeline.v1.RowType
  pub fn has_row_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_row_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn row_type_opt(&self) -> ::protobuf::Optional<super::RowTypeView<'_>> {
        ::protobuf::Optional::new(self.row_type(), self.has_row_type())
  }
  pub fn row_type(&self) -> super::RowTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RowTypeView::default())
  }
  pub fn row_type_mut(&mut self) -> super::RowTypeMut<'_> {
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
  pub fn set_row_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::RowType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // logical_type: optional message org.apache.beam.model.pipeline.v1.LogicalType
  pub fn has_logical_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_logical_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn logical_type_opt(&self) -> ::protobuf::Optional<super::LogicalTypeView<'_>> {
        ::protobuf::Optional::new(self.logical_type(), self.has_logical_type())
  }
  pub fn logical_type(&self) -> super::LogicalTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogicalTypeView::default())
  }
  pub fn logical_type_mut(&mut self) -> super::LogicalTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_logical_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::LogicalType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn type_info(&self) -> super::field_type::TypeInfoOneof<'_> {
    match &self.type_info_case() {
      super::field_type::TypeInfoCase::AtomicType =>
          super::field_type::TypeInfoOneof::AtomicType(self.atomic_type()),
      super::field_type::TypeInfoCase::ArrayType =>
          super::field_type::TypeInfoOneof::ArrayType(self.array_type()),
      super::field_type::TypeInfoCase::IterableType =>
          super::field_type::TypeInfoOneof::IterableType(self.iterable_type()),
      super::field_type::TypeInfoCase::MapType =>
          super::field_type::TypeInfoOneof::MapType(self.map_type()),
      super::field_type::TypeInfoCase::RowType =>
          super::field_type::TypeInfoOneof::RowType(self.row_type()),
      super::field_type::TypeInfoCase::LogicalType =>
          super::field_type::TypeInfoOneof::LogicalType(self.logical_type()),
      _ => super::field_type::TypeInfoOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn type_info_case(&self) -> super::field_type::TypeInfoCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::field_type::TypeInfoCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FieldTypeMut` does not perform any shared mutation.
unsafe impl Send for FieldTypeMut<'_> {}

// SAFETY:
// - `FieldTypeMut` does not perform any shared mutation.
unsafe impl Sync for FieldTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for FieldTypeMut<'msg> {
  type Proxied = FieldType;
  fn as_view(&self) -> ::protobuf::View<'_, FieldType> {
    FieldTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FieldType>
  where
      'msg: 'shorter {
    FieldTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for FieldTypeMut<'msg> {
  type MutProxied = FieldType;
  fn as_mut(&mut self) -> FieldTypeMut<'msg> {
    FieldTypeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FieldTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> FieldTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FieldType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FieldType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FieldTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FieldTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // nullable: optional bool
  pub fn nullable(&self) -> bool {
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
  pub fn set_nullable(&mut self, val: bool) {
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

  // atomic_type: optional enum org.apache.beam.model.pipeline.v1.AtomicType
  pub fn has_atomic_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_atomic_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn atomic_type_opt(&self) -> ::protobuf::Optional<super::AtomicType> {
        ::protobuf::Optional::new(self.atomic_type(), self.has_atomic_type())
  }
  pub fn atomic_type(&self) -> super::AtomicType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::AtomicType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_atomic_type(&mut self, val: super::AtomicType) {
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

  // array_type: optional message org.apache.beam.model.pipeline.v1.ArrayType
  pub fn has_array_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_array_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn array_type_opt(&self) -> ::protobuf::Optional<super::ArrayTypeView<'_>> {
        ::protobuf::Optional::new(self.array_type(), self.has_array_type())
  }
  pub fn array_type(&self) -> super::ArrayTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArrayTypeView::default())
  }
  pub fn array_type_mut(&mut self) -> super::ArrayTypeMut<'_> {
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
  pub fn set_array_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArrayType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // iterable_type: optional message org.apache.beam.model.pipeline.v1.IterableType
  pub fn has_iterable_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_iterable_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn iterable_type_opt(&self) -> ::protobuf::Optional<super::IterableTypeView<'_>> {
        ::protobuf::Optional::new(self.iterable_type(), self.has_iterable_type())
  }
  pub fn iterable_type(&self) -> super::IterableTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::IterableTypeView::default())
  }
  pub fn iterable_type_mut(&mut self) -> super::IterableTypeMut<'_> {
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
  pub fn set_iterable_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::IterableType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // map_type: optional message org.apache.beam.model.pipeline.v1.MapType
  pub fn has_map_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_map_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn map_type_opt(&self) -> ::protobuf::Optional<super::MapTypeView<'_>> {
        ::protobuf::Optional::new(self.map_type(), self.has_map_type())
  }
  pub fn map_type(&self) -> super::MapTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapTypeView::default())
  }
  pub fn map_type_mut(&mut self) -> super::MapTypeMut<'_> {
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
  pub fn set_map_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::MapType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // row_type: optional message org.apache.beam.model.pipeline.v1.RowType
  pub fn has_row_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_row_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn row_type_opt(&self) -> ::protobuf::Optional<super::RowTypeView<'_>> {
        ::protobuf::Optional::new(self.row_type(), self.has_row_type())
  }
  pub fn row_type(&self) -> super::RowTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RowTypeView::default())
  }
  pub fn row_type_mut(&mut self) -> super::RowTypeMut<'_> {
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
  pub fn set_row_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::RowType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // logical_type: optional message org.apache.beam.model.pipeline.v1.LogicalType
  pub fn has_logical_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_logical_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn logical_type_opt(&self) -> ::protobuf::Optional<super::LogicalTypeView<'_>> {
        ::protobuf::Optional::new(self.logical_type(), self.has_logical_type())
  }
  pub fn logical_type(&self) -> super::LogicalTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogicalTypeView::default())
  }
  pub fn logical_type_mut(&mut self) -> super::LogicalTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_logical_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::LogicalType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn type_info(&self) -> super::field_type::TypeInfoOneof<'_> {
    match &self.type_info_case() {
      super::field_type::TypeInfoCase::AtomicType =>
          super::field_type::TypeInfoOneof::AtomicType(self.atomic_type()),
      super::field_type::TypeInfoCase::ArrayType =>
          super::field_type::TypeInfoOneof::ArrayType(self.array_type()),
      super::field_type::TypeInfoCase::IterableType =>
          super::field_type::TypeInfoOneof::IterableType(self.iterable_type()),
      super::field_type::TypeInfoCase::MapType =>
          super::field_type::TypeInfoOneof::MapType(self.map_type()),
      super::field_type::TypeInfoCase::RowType =>
          super::field_type::TypeInfoOneof::RowType(self.row_type()),
      super::field_type::TypeInfoCase::LogicalType =>
          super::field_type::TypeInfoOneof::LogicalType(self.logical_type()),
      _ => super::field_type::TypeInfoOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn type_info_case(&self) -> super::field_type::TypeInfoCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::field_type::TypeInfoCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FieldType

impl ::std::ops::Drop for FieldType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FieldType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FieldType {
  type Proxied = Self;
  fn as_view(&self) -> FieldTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FieldType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FieldTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FieldType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldType {
  type Msg = FieldType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldType {
  type Msg = FieldType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldTypeMut<'_> {
  type Msg = FieldType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldTypeMut<'_> {
  type Msg = FieldType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldTypeView<'_> {
  type Msg = FieldType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod field_type {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeInfoOneof<'msg> {
  AtomicType(::protobuf::View<'msg, super::super::AtomicType>) = 2,
  ArrayType(::protobuf::View<'msg, super::super::ArrayType>) = 3,
  IterableType(::protobuf::View<'msg, super::super::IterableType>) = 4,
  MapType(::protobuf::View<'msg, super::super::MapType>) = 5,
  RowType(::protobuf::View<'msg, super::super::RowType>) = 6,
  LogicalType(::protobuf::View<'msg, super::super::LogicalType>) = 7,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeInfoCase {
  AtomicType = 2,
  ArrayType = 3,
  IterableType = 4,
  MapType = 5,
  RowType = 6,
  LogicalType = 7,

  not_set = 0
}

impl TypeInfoCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeInfoCase> {
    match v {
      0 => Some(TypeInfoCase::not_set),
      2 => Some(TypeInfoCase::AtomicType),
      3 => Some(TypeInfoCase::ArrayType),
      4 => Some(TypeInfoCase::IterableType),
      5 => Some(TypeInfoCase::MapType),
      6 => Some(TypeInfoCase::RowType),
      7 => Some(TypeInfoCase::LogicalType),
      _ => None
    }
  }
}
}  // pub mod field_type


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__ArrayType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ArrayType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ArrayType>
}

impl ::protobuf::Message for ArrayType {}

impl ::std::default::Default for ArrayType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ArrayType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ArrayType` is `Sync` because it does not implement interior mutability.
//    Neither does `ArrayTypeMut`.
unsafe impl Sync for ArrayType {}

// SAFETY:
// - `ArrayType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ArrayType {}

impl ::protobuf::Proxied for ArrayType {
  type View<'msg> = ArrayTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ArrayType {}

impl ::protobuf::MutProxied for ArrayType {
  type Mut<'msg> = ArrayTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArrayTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArrayTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ArrayTypeView<'msg> {
  type Message = ArrayType;
}

impl ::std::fmt::Debug for ArrayTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ArrayTypeView<'_> {
  fn default() -> ArrayTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayType>> for ArrayTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArrayTypeView<'msg> {

  pub fn to_owned(&self) -> ArrayType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // element_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_element_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn element_type_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.element_type(), self.has_element_type())
  }
  pub fn element_type(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

}

// SAFETY:
// - `ArrayTypeView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArrayTypeView<'_> {}

// SAFETY:
// - `ArrayTypeView` is `Send` because while its alive a `ArrayTypeMut` cannot.
// - `ArrayTypeView` does not use thread-local data.
unsafe impl Send for ArrayTypeView<'_> {}

impl<'msg> ::protobuf::AsView for ArrayTypeView<'msg> {
  type Proxied = ArrayType;
  fn as_view(&self) -> ::protobuf::View<'msg, ArrayType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArrayTypeView<'msg> {
  fn into_view<'shorter>(self) -> ArrayTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ArrayType> for ArrayTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArrayType {
    let mut dst = ArrayType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ArrayType> for ArrayTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArrayType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ArrayType {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArrayTypeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArrayTypeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArrayTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArrayTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ArrayTypeMut<'msg> {
  type Message = ArrayType;
}

impl ::std::fmt::Debug for ArrayTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayType>> for ArrayTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArrayTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayType> {
    self.inner
  }

  pub fn to_owned(&self) -> ArrayType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // element_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_element_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_element_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn element_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.element_type(), self.has_element_type())
  }
  pub fn element_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn element_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_element_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

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
// - `ArrayTypeMut` does not perform any shared mutation.
unsafe impl Send for ArrayTypeMut<'_> {}

// SAFETY:
// - `ArrayTypeMut` does not perform any shared mutation.
unsafe impl Sync for ArrayTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for ArrayTypeMut<'msg> {
  type Proxied = ArrayType;
  fn as_view(&self) -> ::protobuf::View<'_, ArrayType> {
    ArrayTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArrayTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ArrayType>
  where
      'msg: 'shorter {
    ArrayTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ArrayTypeMut<'msg> {
  type MutProxied = ArrayType;
  fn as_mut(&mut self) -> ArrayTypeMut<'msg> {
    ArrayTypeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ArrayTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> ArrayTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ArrayType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ArrayType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ArrayTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ArrayTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // element_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_element_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_element_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn element_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.element_type(), self.has_element_type())
  }
  pub fn element_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn element_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_element_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ArrayType

impl ::std::ops::Drop for ArrayType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ArrayType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ArrayType {
  type Proxied = Self;
  fn as_view(&self) -> ArrayTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ArrayType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArrayTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArrayType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__ArrayType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        super::org__apache__beam__model__pipeline__v1__Field_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3(P(PG");
        super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P.33333^#|$|%|&|(|)");
        super::org__apache__beam__model__pipeline__v1__IterableType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        super::org__apache__beam__model__pipeline__v1__LogicalType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X0P333");
        super::org__apache__beam__model__pipeline__v1__MapType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        super::org__apache__beam__model__pipeline__v1__Option_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33");
        super::org__apache__beam__model__pipeline__v1__RowType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        super::org__apache__beam__model__pipeline__v1__Schema_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G1XG/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__ArrayType_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__Field_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__Option_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__ArrayType_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__IterableType_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__MapType_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__RowType_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__LogicalType_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__IterableType_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__LogicalType_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            <super::FieldValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MapType_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__Option_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldType_msg_init.0,
            <super::FieldValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__RowType_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__Schema_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__Schema_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__Field_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__Option_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__ArrayType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArrayType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArrayType {
  type Msg = ArrayType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayType {
  type Msg = ArrayType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArrayTypeMut<'_> {
  type Msg = ArrayType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayTypeMut<'_> {
  type Msg = ArrayType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayTypeView<'_> {
  type Msg = ArrayType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArrayTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__IterableType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct IterableType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<IterableType>
}

impl ::protobuf::Message for IterableType {}

impl ::std::default::Default for IterableType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for IterableType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `IterableType` is `Sync` because it does not implement interior mutability.
//    Neither does `IterableTypeMut`.
unsafe impl Sync for IterableType {}

// SAFETY:
// - `IterableType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IterableType {}

impl ::protobuf::Proxied for IterableType {
  type View<'msg> = IterableTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for IterableType {}

impl ::protobuf::MutProxied for IterableType {
  type Mut<'msg> = IterableTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IterableTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IterableType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IterableTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for IterableTypeView<'msg> {
  type Message = IterableType;
}

impl ::std::fmt::Debug for IterableTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for IterableTypeView<'_> {
  fn default() -> IterableTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, IterableType>> for IterableTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IterableType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IterableTypeView<'msg> {

  pub fn to_owned(&self) -> IterableType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // element_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_element_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn element_type_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.element_type(), self.has_element_type())
  }
  pub fn element_type(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

}

// SAFETY:
// - `IterableTypeView` is `Sync` because it does not support mutation.
unsafe impl Sync for IterableTypeView<'_> {}

// SAFETY:
// - `IterableTypeView` is `Send` because while its alive a `IterableTypeMut` cannot.
// - `IterableTypeView` does not use thread-local data.
unsafe impl Send for IterableTypeView<'_> {}

impl<'msg> ::protobuf::AsView for IterableTypeView<'msg> {
  type Proxied = IterableType;
  fn as_view(&self) -> ::protobuf::View<'msg, IterableType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IterableTypeView<'msg> {
  fn into_view<'shorter>(self) -> IterableTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<IterableType> for IterableTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IterableType {
    let mut dst = IterableType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<IterableType> for IterableTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IterableType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for IterableType {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for IterableTypeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for IterableTypeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IterableTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IterableType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IterableTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for IterableTypeMut<'msg> {
  type Message = IterableType;
}

impl ::std::fmt::Debug for IterableTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, IterableType>> for IterableTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IterableType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IterableTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, IterableType> {
    self.inner
  }

  pub fn to_owned(&self) -> IterableType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // element_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_element_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_element_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn element_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.element_type(), self.has_element_type())
  }
  pub fn element_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn element_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_element_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

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
// - `IterableTypeMut` does not perform any shared mutation.
unsafe impl Send for IterableTypeMut<'_> {}

// SAFETY:
// - `IterableTypeMut` does not perform any shared mutation.
unsafe impl Sync for IterableTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for IterableTypeMut<'msg> {
  type Proxied = IterableType;
  fn as_view(&self) -> ::protobuf::View<'_, IterableType> {
    IterableTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IterableTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, IterableType>
  where
      'msg: 'shorter {
    IterableTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for IterableTypeMut<'msg> {
  type MutProxied = IterableType;
  fn as_mut(&mut self) -> IterableTypeMut<'msg> {
    IterableTypeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for IterableTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> IterableTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl IterableType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, IterableType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> IterableTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> IterableTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // element_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_element_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_element_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn element_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.element_type(), self.has_element_type())
  }
  pub fn element_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn element_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_element_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl IterableType

impl ::std::ops::Drop for IterableType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for IterableType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for IterableType {
  type Proxied = Self;
  fn as_view(&self) -> IterableTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for IterableType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> IterableTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for IterableType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__IterableType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IterableType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IterableType {
  type Msg = IterableType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IterableType {
  type Msg = IterableType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IterableTypeMut<'_> {
  type Msg = IterableType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IterableTypeMut<'_> {
  type Msg = IterableType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IterableTypeView<'_> {
  type Msg = IterableType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IterableTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MapType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MapType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MapType>
}

impl ::protobuf::Message for MapType {}

impl ::std::default::Default for MapType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MapType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MapType` is `Sync` because it does not implement interior mutability.
//    Neither does `MapTypeMut`.
unsafe impl Sync for MapType {}

// SAFETY:
// - `MapType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MapType {}

impl ::protobuf::Proxied for MapType {
  type View<'msg> = MapTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MapType {}

impl ::protobuf::MutProxied for MapType {
  type Mut<'msg> = MapTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MapTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MapTypeView<'msg> {
  type Message = MapType;
}

impl ::std::fmt::Debug for MapTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MapTypeView<'_> {
  fn default() -> MapTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MapType>> for MapTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeView<'msg> {

  pub fn to_owned(&self) -> MapType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_key_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn key_type_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.key_type(), self.has_key_type())
  }
  pub fn key_type(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

  // value_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_value_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_type_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.value_type(), self.has_value_type())
  }
  pub fn value_type(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

}

// SAFETY:
// - `MapTypeView` is `Sync` because it does not support mutation.
unsafe impl Sync for MapTypeView<'_> {}

// SAFETY:
// - `MapTypeView` is `Send` because while its alive a `MapTypeMut` cannot.
// - `MapTypeView` does not use thread-local data.
unsafe impl Send for MapTypeView<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeView<'msg> {
  type Proxied = MapType;
  fn as_view(&self) -> ::protobuf::View<'msg, MapType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeView<'msg> {
  fn into_view<'shorter>(self) -> MapTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MapType> for MapTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapType {
    let mut dst = MapType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MapType> for MapTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MapType {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MapTypeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MapTypeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MapTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MapTypeMut<'msg> {
  type Message = MapType;
}

impl ::std::fmt::Debug for MapTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MapType>> for MapTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MapType> {
    self.inner
  }

  pub fn to_owned(&self) -> MapType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_key_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.key_type(), self.has_key_type())
  }
  pub fn key_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn key_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_key_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_value_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.value_type(), self.has_value_type())
  }
  pub fn value_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn value_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_value_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

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
// - `MapTypeMut` does not perform any shared mutation.
unsafe impl Send for MapTypeMut<'_> {}

// SAFETY:
// - `MapTypeMut` does not perform any shared mutation.
unsafe impl Sync for MapTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeMut<'msg> {
  type Proxied = MapType;
  fn as_view(&self) -> ::protobuf::View<'_, MapType> {
    MapTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MapType>
  where
      'msg: 'shorter {
    MapTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MapTypeMut<'msg> {
  type MutProxied = MapType;
  fn as_mut(&mut self) -> MapTypeMut<'msg> {
    MapTypeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MapTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> MapTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MapType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MapType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MapTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MapTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_key_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.key_type(), self.has_key_type())
  }
  pub fn key_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn key_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_key_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_value_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.value_type(), self.has_value_type())
  }
  pub fn value_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn value_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_value_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl MapType

impl ::std::ops::Drop for MapType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MapType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MapType {
  type Proxied = Self;
  fn as_view(&self) -> MapTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MapType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MapTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MapType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MapType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapType {
  type Msg = MapType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapType {
  type Msg = MapType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapTypeMut<'_> {
  type Msg = MapType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeMut<'_> {
  type Msg = MapType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeView<'_> {
  type Msg = MapType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__RowType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RowType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RowType>
}

impl ::protobuf::Message for RowType {}

impl ::std::default::Default for RowType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RowType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RowType` is `Sync` because it does not implement interior mutability.
//    Neither does `RowTypeMut`.
unsafe impl Sync for RowType {}

// SAFETY:
// - `RowType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RowType {}

impl ::protobuf::Proxied for RowType {
  type View<'msg> = RowTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RowType {}

impl ::protobuf::MutProxied for RowType {
  type Mut<'msg> = RowTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RowTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RowType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RowTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RowTypeView<'msg> {
  type Message = RowType;
}

impl ::std::fmt::Debug for RowTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RowTypeView<'_> {
  fn default() -> RowTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RowType>> for RowTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RowType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RowTypeView<'msg> {

  pub fn to_owned(&self) -> RowType {
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

}

// SAFETY:
// - `RowTypeView` is `Sync` because it does not support mutation.
unsafe impl Sync for RowTypeView<'_> {}

// SAFETY:
// - `RowTypeView` is `Send` because while its alive a `RowTypeMut` cannot.
// - `RowTypeView` does not use thread-local data.
unsafe impl Send for RowTypeView<'_> {}

impl<'msg> ::protobuf::AsView for RowTypeView<'msg> {
  type Proxied = RowType;
  fn as_view(&self) -> ::protobuf::View<'msg, RowType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RowTypeView<'msg> {
  fn into_view<'shorter>(self) -> RowTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RowType> for RowTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RowType {
    let mut dst = RowType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RowType> for RowTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RowType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for RowType {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RowTypeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RowTypeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RowTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RowType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RowTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RowTypeMut<'msg> {
  type Message = RowType;
}

impl ::std::fmt::Debug for RowTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RowType>> for RowTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RowType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RowTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RowType> {
    self.inner
  }

  pub fn to_owned(&self) -> RowType {
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

}

// SAFETY:
// - `RowTypeMut` does not perform any shared mutation.
unsafe impl Send for RowTypeMut<'_> {}

// SAFETY:
// - `RowTypeMut` does not perform any shared mutation.
unsafe impl Sync for RowTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for RowTypeMut<'msg> {
  type Proxied = RowType;
  fn as_view(&self) -> ::protobuf::View<'_, RowType> {
    RowTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RowTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RowType>
  where
      'msg: 'shorter {
    RowTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RowTypeMut<'msg> {
  type MutProxied = RowType;
  fn as_mut(&mut self) -> RowTypeMut<'msg> {
    RowTypeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RowTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> RowTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RowType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RowType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RowTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RowTypeMut<'_> {
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

}  // impl RowType

impl ::std::ops::Drop for RowType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RowType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RowType {
  type Proxied = Self;
  fn as_view(&self) -> RowTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RowType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RowTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RowType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__RowType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RowType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RowType {
  type Msg = RowType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RowType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RowType {
  type Msg = RowType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RowType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RowTypeMut<'_> {
  type Msg = RowType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RowType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RowTypeMut<'_> {
  type Msg = RowType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RowType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RowTypeView<'_> {
  type Msg = RowType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RowType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RowTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__LogicalType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LogicalType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LogicalType>
}

impl ::protobuf::Message for LogicalType {}

impl ::std::default::Default for LogicalType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LogicalType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LogicalType` is `Sync` because it does not implement interior mutability.
//    Neither does `LogicalTypeMut`.
unsafe impl Sync for LogicalType {}

// SAFETY:
// - `LogicalType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for LogicalType {}

impl ::protobuf::Proxied for LogicalType {
  type View<'msg> = LogicalTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LogicalType {}

impl ::protobuf::MutProxied for LogicalType {
  type Mut<'msg> = LogicalTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LogicalTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogicalTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LogicalTypeView<'msg> {
  type Message = LogicalType;
}

impl ::std::fmt::Debug for LogicalTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LogicalTypeView<'_> {
  fn default() -> LogicalTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalType>> for LogicalTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogicalTypeView<'msg> {

  pub fn to_owned(&self) -> LogicalType {
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

  // representation: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_representation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn representation_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.representation(), self.has_representation())
  }
  pub fn representation(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

  // argument_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_argument_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn argument_type_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.argument_type(), self.has_argument_type())
  }
  pub fn argument_type(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

  // argument: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_argument(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn argument_opt(self) -> ::protobuf::Optional<super::FieldValueView<'msg>> {
        ::protobuf::Optional::new(self.argument(), self.has_argument())
  }
  pub fn argument(self) -> super::FieldValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }

}

// SAFETY:
// - `LogicalTypeView` is `Sync` because it does not support mutation.
unsafe impl Sync for LogicalTypeView<'_> {}

// SAFETY:
// - `LogicalTypeView` is `Send` because while its alive a `LogicalTypeMut` cannot.
// - `LogicalTypeView` does not use thread-local data.
unsafe impl Send for LogicalTypeView<'_> {}

impl<'msg> ::protobuf::AsView for LogicalTypeView<'msg> {
  type Proxied = LogicalType;
  fn as_view(&self) -> ::protobuf::View<'msg, LogicalType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogicalTypeView<'msg> {
  fn into_view<'shorter>(self) -> LogicalTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LogicalType> for LogicalTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogicalType {
    let mut dst = LogicalType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LogicalType> for LogicalTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogicalType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for LogicalType {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LogicalTypeView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LogicalTypeMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LogicalTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogicalTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LogicalTypeMut<'msg> {
  type Message = LogicalType;
}

impl ::std::fmt::Debug for LogicalTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalType>> for LogicalTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogicalTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalType> {
    self.inner
  }

  pub fn to_owned(&self) -> LogicalType {
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

  // representation: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_representation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_representation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn representation_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.representation(), self.has_representation())
  }
  pub fn representation(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn representation_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_representation(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // argument_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_argument_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_argument_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn argument_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.argument_type(), self.has_argument_type())
  }
  pub fn argument_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn argument_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_argument_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // argument: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_argument(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_argument(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn argument_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.argument(), self.has_argument())
  }
  pub fn argument(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn argument_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_argument(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

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
// - `LogicalTypeMut` does not perform any shared mutation.
unsafe impl Send for LogicalTypeMut<'_> {}

// SAFETY:
// - `LogicalTypeMut` does not perform any shared mutation.
unsafe impl Sync for LogicalTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for LogicalTypeMut<'msg> {
  type Proxied = LogicalType;
  fn as_view(&self) -> ::protobuf::View<'_, LogicalType> {
    LogicalTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogicalTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LogicalType>
  where
      'msg: 'shorter {
    LogicalTypeView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for LogicalTypeMut<'msg> {
  type MutProxied = LogicalType;
  fn as_mut(&mut self) -> LogicalTypeMut<'msg> {
    LogicalTypeMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LogicalTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> LogicalTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LogicalType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LogicalType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LogicalTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LogicalTypeMut<'_> {
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

  // representation: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_representation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_representation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn representation_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.representation(), self.has_representation())
  }
  pub fn representation(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn representation_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_representation(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // argument_type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_argument_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_argument_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn argument_type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.argument_type(), self.has_argument_type())
  }
  pub fn argument_type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn argument_type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_argument_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // argument: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_argument(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_argument(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn argument_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.argument(), self.has_argument())
  }
  pub fn argument(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn argument_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_argument(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl LogicalType

impl ::std::ops::Drop for LogicalType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LogicalType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LogicalType {
  type Proxied = Self;
  fn as_view(&self) -> LogicalTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LogicalType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LogicalTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LogicalType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__LogicalType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogicalType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogicalType {
  type Msg = LogicalType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalType {
  type Msg = LogicalType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogicalTypeMut<'_> {
  type Msg = LogicalType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypeMut<'_> {
  type Msg = LogicalType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypeView<'_> {
  type Msg = LogicalType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogicalTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__LogicalTypes_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LogicalTypes {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LogicalTypes>
}

impl ::protobuf::Message for LogicalTypes {}

impl ::std::default::Default for LogicalTypes {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LogicalTypes {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LogicalTypes` is `Sync` because it does not implement interior mutability.
//    Neither does `LogicalTypesMut`.
unsafe impl Sync for LogicalTypes {}

// SAFETY:
// - `LogicalTypes` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for LogicalTypes {}

impl ::protobuf::Proxied for LogicalTypes {
  type View<'msg> = LogicalTypesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LogicalTypes {}

impl ::protobuf::MutProxied for LogicalTypes {
  type Mut<'msg> = LogicalTypesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LogicalTypesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalTypes>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogicalTypesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LogicalTypesView<'msg> {
  type Message = LogicalTypes;
}

impl ::std::fmt::Debug for LogicalTypesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LogicalTypesView<'_> {
  fn default() -> LogicalTypesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalTypes>> for LogicalTypesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalTypes>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogicalTypesView<'msg> {

  pub fn to_owned(&self) -> LogicalTypes {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `LogicalTypesView` is `Sync` because it does not support mutation.
unsafe impl Sync for LogicalTypesView<'_> {}

// SAFETY:
// - `LogicalTypesView` is `Send` because while its alive a `LogicalTypesMut` cannot.
// - `LogicalTypesView` does not use thread-local data.
unsafe impl Send for LogicalTypesView<'_> {}

impl<'msg> ::protobuf::AsView for LogicalTypesView<'msg> {
  type Proxied = LogicalTypes;
  fn as_view(&self) -> ::protobuf::View<'msg, LogicalTypes> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogicalTypesView<'msg> {
  fn into_view<'shorter>(self) -> LogicalTypesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LogicalTypes> for LogicalTypesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogicalTypes {
    let mut dst = LogicalTypes::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LogicalTypes> for LogicalTypesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogicalTypes {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for LogicalTypes {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LogicalTypesView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LogicalTypesMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LogicalTypesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypes>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogicalTypesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LogicalTypesMut<'msg> {
  type Message = LogicalTypes;
}

impl ::std::fmt::Debug for LogicalTypesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypes>> for LogicalTypesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypes>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogicalTypesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypes> {
    self.inner
  }

  pub fn to_owned(&self) -> LogicalTypes {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `LogicalTypesMut` does not perform any shared mutation.
unsafe impl Send for LogicalTypesMut<'_> {}

// SAFETY:
// - `LogicalTypesMut` does not perform any shared mutation.
unsafe impl Sync for LogicalTypesMut<'_> {}

impl<'msg> ::protobuf::AsView for LogicalTypesMut<'msg> {
  type Proxied = LogicalTypes;
  fn as_view(&self) -> ::protobuf::View<'_, LogicalTypes> {
    LogicalTypesView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogicalTypesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LogicalTypes>
  where
      'msg: 'shorter {
    LogicalTypesView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for LogicalTypesMut<'msg> {
  type MutProxied = LogicalTypes;
  fn as_mut(&mut self) -> LogicalTypesMut<'msg> {
    LogicalTypesMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LogicalTypesMut<'msg> {
  fn into_mut<'shorter>(self) -> LogicalTypesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LogicalTypes {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LogicalTypes> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LogicalTypesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LogicalTypesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl LogicalTypes

impl ::std::ops::Drop for LogicalTypes {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LogicalTypes {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LogicalTypes {
  type Proxied = Self;
  fn as_view(&self) -> LogicalTypesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LogicalTypes {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LogicalTypesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LogicalTypes {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__LogicalTypes_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__LogicalTypes_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__LogicalTypes_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogicalTypes {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogicalTypes {
  type Msg = LogicalTypes;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypes> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypes {
  type Msg = LogicalTypes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypes> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogicalTypesMut<'_> {
  type Msg = LogicalTypes;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypes> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypesMut<'_> {
  type Msg = LogicalTypes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypes> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypesView<'_> {
  type Msg = LogicalTypes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypes> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogicalTypesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod logical_types {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enum(i32);

#[allow(non_upper_case_globals)]
impl Enum {
  pub const PythonCallable: Enum = Enum(0);
  pub const MicrosInstant: Enum = Enum(1);
  pub const MillisInstant: Enum = Enum(2);
  pub const Decimal: Enum = Enum(3);
  pub const FixedBytes: Enum = Enum(4);
  pub const VarBytes: Enum = Enum(5);
  pub const FixedChar: Enum = Enum(6);
  pub const VarChar: Enum = Enum(7);
  pub const Date: Enum = Enum(8);
  pub const Timestamp: Enum = Enum(9);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "PythonCallable",
      1 => "MicrosInstant",
      2 => "MillisInstant",
      3 => "Decimal",
      4 => "FixedBytes",
      5 => "VarBytes",
      6 => "FixedChar",
      7 => "VarChar",
      8 => "Date",
      9 => "Timestamp",
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
    matches!(value, 0|1|2|3|4|5|6|7|8|9)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Enum {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


}  // pub mod logical_types


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__Option_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Option {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Option>
}

impl ::protobuf::Message for Option {}

impl ::std::default::Default for Option {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Option {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Option` is `Sync` because it does not implement interior mutability.
//    Neither does `OptionMut`.
unsafe impl Sync for Option {}

// SAFETY:
// - `Option` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Option {}

impl ::protobuf::Proxied for Option {
  type View<'msg> = OptionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Option {}

impl ::protobuf::MutProxied for Option {
  type Mut<'msg> = OptionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OptionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Option>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OptionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OptionView<'msg> {
  type Message = Option;
}

impl ::std::fmt::Debug for OptionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OptionView<'_> {
  fn default() -> OptionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Option>> for OptionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Option>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OptionView<'msg> {

  pub fn to_owned(&self) -> Option {
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

  // type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn type_opt(self) -> ::protobuf::Optional<super::FieldTypeView<'msg>> {
        ::protobuf::Optional::new(self.r#type(), self.has_type())
  }
  pub fn r#type(self) -> super::FieldTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn value_opt(self) -> ::protobuf::Optional<super::FieldValueView<'msg>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(self) -> super::FieldValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }

}

// SAFETY:
// - `OptionView` is `Sync` because it does not support mutation.
unsafe impl Sync for OptionView<'_> {}

// SAFETY:
// - `OptionView` is `Send` because while its alive a `OptionMut` cannot.
// - `OptionView` does not use thread-local data.
unsafe impl Send for OptionView<'_> {}

impl<'msg> ::protobuf::AsView for OptionView<'msg> {
  type Proxied = Option;
  fn as_view(&self) -> ::protobuf::View<'msg, Option> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OptionView<'msg> {
  fn into_view<'shorter>(self) -> OptionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Option> for OptionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Option {
    let mut dst = Option::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Option> for OptionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Option {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Option {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for OptionView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for OptionMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OptionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Option>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OptionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OptionMut<'msg> {
  type Message = Option;
}

impl ::std::fmt::Debug for OptionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Option>> for OptionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Option>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OptionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Option> {
    self.inner
  }

  pub fn to_owned(&self) -> Option {
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

  // type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.r#type(), self.has_type())
  }
  pub fn r#type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn value_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn value_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

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
// - `OptionMut` does not perform any shared mutation.
unsafe impl Send for OptionMut<'_> {}

// SAFETY:
// - `OptionMut` does not perform any shared mutation.
unsafe impl Sync for OptionMut<'_> {}

impl<'msg> ::protobuf::AsView for OptionMut<'msg> {
  type Proxied = Option;
  fn as_view(&self) -> ::protobuf::View<'_, Option> {
    OptionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OptionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Option>
  where
      'msg: 'shorter {
    OptionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for OptionMut<'msg> {
  type MutProxied = Option;
  fn as_mut(&mut self) -> OptionMut<'msg> {
    OptionMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OptionMut<'msg> {
  fn into_mut<'shorter>(self) -> OptionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Option {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Option> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OptionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OptionMut<'_> {
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

  // type: optional message org.apache.beam.model.pipeline.v1.FieldType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn type_opt(&self) -> ::protobuf::Optional<super::FieldTypeView<'_>> {
        ::protobuf::Optional::new(self.r#type(), self.has_type())
  }
  pub fn r#type(&self) -> super::FieldTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldTypeView::default())
  }
  pub fn type_mut(&mut self) -> super::FieldTypeMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn value_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn value_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl Option

impl ::std::ops::Drop for Option {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Option {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Option {
  type Proxied = Self;
  fn as_view(&self) -> OptionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Option {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OptionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Option {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__Option_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Option {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Option {
  type Msg = Option;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Option> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Option {
  type Msg = Option;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Option> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OptionMut<'_> {
  type Msg = Option;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Option> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OptionMut<'_> {
  type Msg = Option;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Option> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OptionView<'_> {
  type Msg = Option;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Option> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OptionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__Row_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Row {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Row>
}

impl ::protobuf::Message for Row {}

impl ::std::default::Default for Row {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Row {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Row` is `Sync` because it does not implement interior mutability.
//    Neither does `RowMut`.
unsafe impl Sync for Row {}

// SAFETY:
// - `Row` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Row {}

impl ::protobuf::Proxied for Row {
  type View<'msg> = RowView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Row {}

impl ::protobuf::MutProxied for Row {
  type Mut<'msg> = RowMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RowView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Row>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RowView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RowView<'msg> {
  type Message = Row;
}

impl ::std::fmt::Debug for RowView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RowView<'_> {
  fn default() -> RowView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Row>> for RowView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Row>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RowView<'msg> {

  pub fn to_owned(&self) -> Row {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // values: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn values(self) -> ::protobuf::RepeatedView<'msg, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RowView` is `Sync` because it does not support mutation.
unsafe impl Sync for RowView<'_> {}

// SAFETY:
// - `RowView` is `Send` because while its alive a `RowMut` cannot.
// - `RowView` does not use thread-local data.
unsafe impl Send for RowView<'_> {}

impl<'msg> ::protobuf::AsView for RowView<'msg> {
  type Proxied = Row;
  fn as_view(&self) -> ::protobuf::View<'msg, Row> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RowView<'msg> {
  fn into_view<'shorter>(self) -> RowView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Row> for RowView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Row {
    let mut dst = Row::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Row> for RowMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Row {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Row {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RowView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RowMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RowMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Row>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RowMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RowMut<'msg> {
  type Message = Row;
}

impl ::std::fmt::Debug for RowMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Row>> for RowMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Row>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RowMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Row> {
    self.inner
  }

  pub fn to_owned(&self) -> Row {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // values: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn values(&self) -> ::protobuf::RepeatedView<'_, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn values_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FieldValue> {
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
  pub fn set_values(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FieldValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `RowMut` does not perform any shared mutation.
unsafe impl Send for RowMut<'_> {}

// SAFETY:
// - `RowMut` does not perform any shared mutation.
unsafe impl Sync for RowMut<'_> {}

impl<'msg> ::protobuf::AsView for RowMut<'msg> {
  type Proxied = Row;
  fn as_view(&self) -> ::protobuf::View<'_, Row> {
    RowView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RowMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Row>
  where
      'msg: 'shorter {
    RowView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RowMut<'msg> {
  type MutProxied = Row;
  fn as_mut(&mut self) -> RowMut<'msg> {
    RowMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RowMut<'msg> {
  fn into_mut<'shorter>(self) -> RowMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Row {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Row> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RowView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RowMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // values: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn values(&self) -> ::protobuf::RepeatedView<'_, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn values_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FieldValue> {
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
  pub fn set_values(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FieldValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Row

impl ::std::ops::Drop for Row {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Row {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Row {
  type Proxied = Self;
  fn as_view(&self) -> RowView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Row {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RowMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Row {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayTypeValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__Row_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Row {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Row {
  type Msg = Row;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Row> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Row {
  type Msg = Row;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Row> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RowMut<'_> {
  type Msg = Row;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Row> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RowMut<'_> {
  type Msg = Row;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Row> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RowView<'_> {
  type Msg = Row;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Row> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RowMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__FieldValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FieldValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FieldValue>
}

impl ::protobuf::Message for FieldValue {}

impl ::std::default::Default for FieldValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FieldValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FieldValue` is `Sync` because it does not implement interior mutability.
//    Neither does `FieldValueMut`.
unsafe impl Sync for FieldValue {}

// SAFETY:
// - `FieldValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for FieldValue {}

impl ::protobuf::Proxied for FieldValue {
  type View<'msg> = FieldValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FieldValue {}

impl ::protobuf::MutProxied for FieldValue {
  type Mut<'msg> = FieldValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FieldValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FieldValueView<'msg> {
  type Message = FieldValue;
}

impl ::std::fmt::Debug for FieldValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FieldValueView<'_> {
  fn default() -> FieldValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FieldValue>> for FieldValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldValueView<'msg> {

  pub fn to_owned(&self) -> FieldValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // atomic_value: optional message org.apache.beam.model.pipeline.v1.AtomicTypeValue
  pub fn has_atomic_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn atomic_value_opt(self) -> ::protobuf::Optional<super::AtomicTypeValueView<'msg>> {
        ::protobuf::Optional::new(self.atomic_value(), self.has_atomic_value())
  }
  pub fn atomic_value(self) -> super::AtomicTypeValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AtomicTypeValueView::default())
  }

  // array_value: optional message org.apache.beam.model.pipeline.v1.ArrayTypeValue
  pub fn has_array_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn array_value_opt(self) -> ::protobuf::Optional<super::ArrayTypeValueView<'msg>> {
        ::protobuf::Optional::new(self.array_value(), self.has_array_value())
  }
  pub fn array_value(self) -> super::ArrayTypeValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArrayTypeValueView::default())
  }

  // iterable_value: optional message org.apache.beam.model.pipeline.v1.IterableTypeValue
  pub fn has_iterable_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn iterable_value_opt(self) -> ::protobuf::Optional<super::IterableTypeValueView<'msg>> {
        ::protobuf::Optional::new(self.iterable_value(), self.has_iterable_value())
  }
  pub fn iterable_value(self) -> super::IterableTypeValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::IterableTypeValueView::default())
  }

  // map_value: optional message org.apache.beam.model.pipeline.v1.MapTypeValue
  pub fn has_map_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn map_value_opt(self) -> ::protobuf::Optional<super::MapTypeValueView<'msg>> {
        ::protobuf::Optional::new(self.map_value(), self.has_map_value())
  }
  pub fn map_value(self) -> super::MapTypeValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapTypeValueView::default())
  }

  // row_value: optional message org.apache.beam.model.pipeline.v1.Row
  pub fn has_row_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn row_value_opt(self) -> ::protobuf::Optional<super::RowView<'msg>> {
        ::protobuf::Optional::new(self.row_value(), self.has_row_value())
  }
  pub fn row_value(self) -> super::RowView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RowView::default())
  }

  // logical_type_value: optional message org.apache.beam.model.pipeline.v1.LogicalTypeValue
  pub fn has_logical_type_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn logical_type_value_opt(self) -> ::protobuf::Optional<super::LogicalTypeValueView<'msg>> {
        ::protobuf::Optional::new(self.logical_type_value(), self.has_logical_type_value())
  }
  pub fn logical_type_value(self) -> super::LogicalTypeValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogicalTypeValueView::default())
  }

  pub fn field_value(self) -> super::field_value::FieldValueOneof<'msg> {
    match self.field_value_case() {
      super::field_value::FieldValueCase::AtomicValue =>
          super::field_value::FieldValueOneof::AtomicValue(self.atomic_value()),
      super::field_value::FieldValueCase::ArrayValue =>
          super::field_value::FieldValueOneof::ArrayValue(self.array_value()),
      super::field_value::FieldValueCase::IterableValue =>
          super::field_value::FieldValueOneof::IterableValue(self.iterable_value()),
      super::field_value::FieldValueCase::MapValue =>
          super::field_value::FieldValueOneof::MapValue(self.map_value()),
      super::field_value::FieldValueCase::RowValue =>
          super::field_value::FieldValueOneof::RowValue(self.row_value()),
      super::field_value::FieldValueCase::LogicalTypeValue =>
          super::field_value::FieldValueOneof::LogicalTypeValue(self.logical_type_value()),
      _ => super::field_value::FieldValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn field_value_case(self) -> super::field_value::FieldValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::field_value::FieldValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FieldValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for FieldValueView<'_> {}

// SAFETY:
// - `FieldValueView` is `Send` because while its alive a `FieldValueMut` cannot.
// - `FieldValueView` does not use thread-local data.
unsafe impl Send for FieldValueView<'_> {}

impl<'msg> ::protobuf::AsView for FieldValueView<'msg> {
  type Proxied = FieldValue;
  fn as_view(&self) -> ::protobuf::View<'msg, FieldValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldValueView<'msg> {
  fn into_view<'shorter>(self) -> FieldValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldValue> for FieldValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldValue {
    let mut dst = FieldValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldValue> for FieldValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for FieldValue {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FieldValueView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for FieldValueMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FieldValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FieldValueMut<'msg> {
  type Message = FieldValue;
}

impl ::std::fmt::Debug for FieldValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FieldValue>> for FieldValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldValue> {
    self.inner
  }

  pub fn to_owned(&self) -> FieldValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // atomic_value: optional message org.apache.beam.model.pipeline.v1.AtomicTypeValue
  pub fn has_atomic_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_atomic_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn atomic_value_opt(&self) -> ::protobuf::Optional<super::AtomicTypeValueView<'_>> {
        ::protobuf::Optional::new(self.atomic_value(), self.has_atomic_value())
  }
  pub fn atomic_value(&self) -> super::AtomicTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AtomicTypeValueView::default())
  }
  pub fn atomic_value_mut(&mut self) -> super::AtomicTypeValueMut<'_> {
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
  pub fn set_atomic_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::AtomicTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // array_value: optional message org.apache.beam.model.pipeline.v1.ArrayTypeValue
  pub fn has_array_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_array_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn array_value_opt(&self) -> ::protobuf::Optional<super::ArrayTypeValueView<'_>> {
        ::protobuf::Optional::new(self.array_value(), self.has_array_value())
  }
  pub fn array_value(&self) -> super::ArrayTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArrayTypeValueView::default())
  }
  pub fn array_value_mut(&mut self) -> super::ArrayTypeValueMut<'_> {
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
  pub fn set_array_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArrayTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // iterable_value: optional message org.apache.beam.model.pipeline.v1.IterableTypeValue
  pub fn has_iterable_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_iterable_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn iterable_value_opt(&self) -> ::protobuf::Optional<super::IterableTypeValueView<'_>> {
        ::protobuf::Optional::new(self.iterable_value(), self.has_iterable_value())
  }
  pub fn iterable_value(&self) -> super::IterableTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::IterableTypeValueView::default())
  }
  pub fn iterable_value_mut(&mut self) -> super::IterableTypeValueMut<'_> {
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
  pub fn set_iterable_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::IterableTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // map_value: optional message org.apache.beam.model.pipeline.v1.MapTypeValue
  pub fn has_map_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_map_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn map_value_opt(&self) -> ::protobuf::Optional<super::MapTypeValueView<'_>> {
        ::protobuf::Optional::new(self.map_value(), self.has_map_value())
  }
  pub fn map_value(&self) -> super::MapTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapTypeValueView::default())
  }
  pub fn map_value_mut(&mut self) -> super::MapTypeValueMut<'_> {
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
  pub fn set_map_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::MapTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // row_value: optional message org.apache.beam.model.pipeline.v1.Row
  pub fn has_row_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_row_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn row_value_opt(&self) -> ::protobuf::Optional<super::RowView<'_>> {
        ::protobuf::Optional::new(self.row_value(), self.has_row_value())
  }
  pub fn row_value(&self) -> super::RowView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RowView::default())
  }
  pub fn row_value_mut(&mut self) -> super::RowMut<'_> {
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
  pub fn set_row_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::Row>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // logical_type_value: optional message org.apache.beam.model.pipeline.v1.LogicalTypeValue
  pub fn has_logical_type_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_logical_type_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn logical_type_value_opt(&self) -> ::protobuf::Optional<super::LogicalTypeValueView<'_>> {
        ::protobuf::Optional::new(self.logical_type_value(), self.has_logical_type_value())
  }
  pub fn logical_type_value(&self) -> super::LogicalTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogicalTypeValueView::default())
  }
  pub fn logical_type_value_mut(&mut self) -> super::LogicalTypeValueMut<'_> {
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
  pub fn set_logical_type_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::LogicalTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  pub fn field_value(&self) -> super::field_value::FieldValueOneof<'_> {
    match &self.field_value_case() {
      super::field_value::FieldValueCase::AtomicValue =>
          super::field_value::FieldValueOneof::AtomicValue(self.atomic_value()),
      super::field_value::FieldValueCase::ArrayValue =>
          super::field_value::FieldValueOneof::ArrayValue(self.array_value()),
      super::field_value::FieldValueCase::IterableValue =>
          super::field_value::FieldValueOneof::IterableValue(self.iterable_value()),
      super::field_value::FieldValueCase::MapValue =>
          super::field_value::FieldValueOneof::MapValue(self.map_value()),
      super::field_value::FieldValueCase::RowValue =>
          super::field_value::FieldValueOneof::RowValue(self.row_value()),
      super::field_value::FieldValueCase::LogicalTypeValue =>
          super::field_value::FieldValueOneof::LogicalTypeValue(self.logical_type_value()),
      _ => super::field_value::FieldValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn field_value_case(&self) -> super::field_value::FieldValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::field_value::FieldValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FieldValueMut` does not perform any shared mutation.
unsafe impl Send for FieldValueMut<'_> {}

// SAFETY:
// - `FieldValueMut` does not perform any shared mutation.
unsafe impl Sync for FieldValueMut<'_> {}

impl<'msg> ::protobuf::AsView for FieldValueMut<'msg> {
  type Proxied = FieldValue;
  fn as_view(&self) -> ::protobuf::View<'_, FieldValue> {
    FieldValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FieldValue>
  where
      'msg: 'shorter {
    FieldValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for FieldValueMut<'msg> {
  type MutProxied = FieldValue;
  fn as_mut(&mut self) -> FieldValueMut<'msg> {
    FieldValueMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FieldValueMut<'msg> {
  fn into_mut<'shorter>(self) -> FieldValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FieldValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FieldValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FieldValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FieldValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // atomic_value: optional message org.apache.beam.model.pipeline.v1.AtomicTypeValue
  pub fn has_atomic_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_atomic_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn atomic_value_opt(&self) -> ::protobuf::Optional<super::AtomicTypeValueView<'_>> {
        ::protobuf::Optional::new(self.atomic_value(), self.has_atomic_value())
  }
  pub fn atomic_value(&self) -> super::AtomicTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AtomicTypeValueView::default())
  }
  pub fn atomic_value_mut(&mut self) -> super::AtomicTypeValueMut<'_> {
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
  pub fn set_atomic_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::AtomicTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // array_value: optional message org.apache.beam.model.pipeline.v1.ArrayTypeValue
  pub fn has_array_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_array_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn array_value_opt(&self) -> ::protobuf::Optional<super::ArrayTypeValueView<'_>> {
        ::protobuf::Optional::new(self.array_value(), self.has_array_value())
  }
  pub fn array_value(&self) -> super::ArrayTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ArrayTypeValueView::default())
  }
  pub fn array_value_mut(&mut self) -> super::ArrayTypeValueMut<'_> {
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
  pub fn set_array_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::ArrayTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // iterable_value: optional message org.apache.beam.model.pipeline.v1.IterableTypeValue
  pub fn has_iterable_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_iterable_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn iterable_value_opt(&self) -> ::protobuf::Optional<super::IterableTypeValueView<'_>> {
        ::protobuf::Optional::new(self.iterable_value(), self.has_iterable_value())
  }
  pub fn iterable_value(&self) -> super::IterableTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::IterableTypeValueView::default())
  }
  pub fn iterable_value_mut(&mut self) -> super::IterableTypeValueMut<'_> {
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
  pub fn set_iterable_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::IterableTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // map_value: optional message org.apache.beam.model.pipeline.v1.MapTypeValue
  pub fn has_map_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_map_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn map_value_opt(&self) -> ::protobuf::Optional<super::MapTypeValueView<'_>> {
        ::protobuf::Optional::new(self.map_value(), self.has_map_value())
  }
  pub fn map_value(&self) -> super::MapTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MapTypeValueView::default())
  }
  pub fn map_value_mut(&mut self) -> super::MapTypeValueMut<'_> {
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
  pub fn set_map_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::MapTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // row_value: optional message org.apache.beam.model.pipeline.v1.Row
  pub fn has_row_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_row_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn row_value_opt(&self) -> ::protobuf::Optional<super::RowView<'_>> {
        ::protobuf::Optional::new(self.row_value(), self.has_row_value())
  }
  pub fn row_value(&self) -> super::RowView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RowView::default())
  }
  pub fn row_value_mut(&mut self) -> super::RowMut<'_> {
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
  pub fn set_row_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::Row>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // logical_type_value: optional message org.apache.beam.model.pipeline.v1.LogicalTypeValue
  pub fn has_logical_type_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_logical_type_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn logical_type_value_opt(&self) -> ::protobuf::Optional<super::LogicalTypeValueView<'_>> {
        ::protobuf::Optional::new(self.logical_type_value(), self.has_logical_type_value())
  }
  pub fn logical_type_value(&self) -> super::LogicalTypeValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogicalTypeValueView::default())
  }
  pub fn logical_type_value_mut(&mut self) -> super::LogicalTypeValueMut<'_> {
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
  pub fn set_logical_type_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::LogicalTypeValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  pub fn field_value(&self) -> super::field_value::FieldValueOneof<'_> {
    match &self.field_value_case() {
      super::field_value::FieldValueCase::AtomicValue =>
          super::field_value::FieldValueOneof::AtomicValue(self.atomic_value()),
      super::field_value::FieldValueCase::ArrayValue =>
          super::field_value::FieldValueOneof::ArrayValue(self.array_value()),
      super::field_value::FieldValueCase::IterableValue =>
          super::field_value::FieldValueOneof::IterableValue(self.iterable_value()),
      super::field_value::FieldValueCase::MapValue =>
          super::field_value::FieldValueOneof::MapValue(self.map_value()),
      super::field_value::FieldValueCase::RowValue =>
          super::field_value::FieldValueOneof::RowValue(self.row_value()),
      super::field_value::FieldValueCase::LogicalTypeValue =>
          super::field_value::FieldValueOneof::LogicalTypeValue(self.logical_type_value()),
      _ => super::field_value::FieldValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn field_value_case(&self) -> super::field_value::FieldValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::field_value::FieldValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FieldValue

impl ::std::ops::Drop for FieldValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FieldValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FieldValue {
  type Proxied = Self;
  fn as_view(&self) -> FieldValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FieldValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FieldValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FieldValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayTypeValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldValue {
  type Msg = FieldValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldValue {
  type Msg = FieldValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldValueMut<'_> {
  type Msg = FieldValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldValueMut<'_> {
  type Msg = FieldValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldValueView<'_> {
  type Msg = FieldValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod field_value {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum FieldValueOneof<'msg> {
  AtomicValue(::protobuf::View<'msg, super::super::AtomicTypeValue>) = 1,
  ArrayValue(::protobuf::View<'msg, super::super::ArrayTypeValue>) = 2,
  IterableValue(::protobuf::View<'msg, super::super::IterableTypeValue>) = 3,
  MapValue(::protobuf::View<'msg, super::super::MapTypeValue>) = 4,
  RowValue(::protobuf::View<'msg, super::super::Row>) = 5,
  LogicalTypeValue(::protobuf::View<'msg, super::super::LogicalTypeValue>) = 6,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum FieldValueCase {
  AtomicValue = 1,
  ArrayValue = 2,
  IterableValue = 3,
  MapValue = 4,
  RowValue = 5,
  LogicalTypeValue = 6,

  not_set = 0
}

impl FieldValueCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<FieldValueCase> {
    match v {
      0 => Some(FieldValueCase::not_set),
      1 => Some(FieldValueCase::AtomicValue),
      2 => Some(FieldValueCase::ArrayValue),
      3 => Some(FieldValueCase::IterableValue),
      4 => Some(FieldValueCase::MapValue),
      5 => Some(FieldValueCase::RowValue),
      6 => Some(FieldValueCase::LogicalTypeValue),
      _ => None
    }
  }
}
}  // pub mod field_value


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__AtomicTypeValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AtomicTypeValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AtomicTypeValue>
}

impl ::protobuf::Message for AtomicTypeValue {}

impl ::std::default::Default for AtomicTypeValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AtomicTypeValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AtomicTypeValue` is `Sync` because it does not implement interior mutability.
//    Neither does `AtomicTypeValueMut`.
unsafe impl Sync for AtomicTypeValue {}

// SAFETY:
// - `AtomicTypeValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AtomicTypeValue {}

impl ::protobuf::Proxied for AtomicTypeValue {
  type View<'msg> = AtomicTypeValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AtomicTypeValue {}

impl ::protobuf::MutProxied for AtomicTypeValue {
  type Mut<'msg> = AtomicTypeValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AtomicTypeValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AtomicTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AtomicTypeValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AtomicTypeValueView<'msg> {
  type Message = AtomicTypeValue;
}

impl ::std::fmt::Debug for AtomicTypeValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AtomicTypeValueView<'_> {
  fn default() -> AtomicTypeValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AtomicTypeValue>> for AtomicTypeValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AtomicTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AtomicTypeValueView<'msg> {

  pub fn to_owned(&self) -> AtomicTypeValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // byte: optional int32
  pub fn has_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn byte_opt(self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.byte(), self.has_byte())
  }
  pub fn byte(self) -> i32 {
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

  // int16: optional int32
  pub fn has_int16(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn int16_opt(self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.int16(), self.has_int16())
  }
  pub fn int16(self) -> i32 {
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

  // int32: optional int32
  pub fn has_int32(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn int32_opt(self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.int32(), self.has_int32())
  }
  pub fn int32(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // int64: optional int64
  pub fn has_int64(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn int64_opt(self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.int64(), self.has_int64())
  }
  pub fn int64(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // float: optional float
  pub fn has_float(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn float_opt(self) -> ::protobuf::Optional<f32> {
        ::protobuf::Optional::new(self.float(), self.has_float())
  }
  pub fn float(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // double: optional double
  pub fn has_double(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn double_opt(self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.double(), self.has_double())
  }
  pub fn double(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // string: optional string
  pub fn has_string(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn string_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.string(), self.has_string())
  }
  pub fn string(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // boolean: optional bool
  pub fn has_boolean(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn boolean_opt(self) -> ::protobuf::Optional<bool> {
        ::protobuf::Optional::new(self.boolean(), self.has_boolean())
  }
  pub fn boolean(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

  // bytes: optional bytes
  pub fn has_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn bytes_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.bytes(), self.has_bytes())
  }
  pub fn bytes(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  pub fn value(self) -> super::atomic_type_value::ValueOneof<'msg> {
    match self.value_case() {
      super::atomic_type_value::ValueCase::Byte =>
          super::atomic_type_value::ValueOneof::Byte(self.byte()),
      super::atomic_type_value::ValueCase::Int16 =>
          super::atomic_type_value::ValueOneof::Int16(self.int16()),
      super::atomic_type_value::ValueCase::Int32 =>
          super::atomic_type_value::ValueOneof::Int32(self.int32()),
      super::atomic_type_value::ValueCase::Int64 =>
          super::atomic_type_value::ValueOneof::Int64(self.int64()),
      super::atomic_type_value::ValueCase::Float =>
          super::atomic_type_value::ValueOneof::Float(self.float()),
      super::atomic_type_value::ValueCase::Double =>
          super::atomic_type_value::ValueOneof::Double(self.double()),
      super::atomic_type_value::ValueCase::String =>
          super::atomic_type_value::ValueOneof::String(self.string()),
      super::atomic_type_value::ValueCase::Boolean =>
          super::atomic_type_value::ValueOneof::Boolean(self.boolean()),
      super::atomic_type_value::ValueCase::Bytes =>
          super::atomic_type_value::ValueOneof::Bytes(self.bytes()),
      _ => super::atomic_type_value::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(self) -> super::atomic_type_value::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::atomic_type_value::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AtomicTypeValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for AtomicTypeValueView<'_> {}

// SAFETY:
// - `AtomicTypeValueView` is `Send` because while its alive a `AtomicTypeValueMut` cannot.
// - `AtomicTypeValueView` does not use thread-local data.
unsafe impl Send for AtomicTypeValueView<'_> {}

impl<'msg> ::protobuf::AsView for AtomicTypeValueView<'msg> {
  type Proxied = AtomicTypeValue;
  fn as_view(&self) -> ::protobuf::View<'msg, AtomicTypeValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AtomicTypeValueView<'msg> {
  fn into_view<'shorter>(self) -> AtomicTypeValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AtomicTypeValue> for AtomicTypeValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AtomicTypeValue {
    let mut dst = AtomicTypeValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AtomicTypeValue> for AtomicTypeValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AtomicTypeValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AtomicTypeValue {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AtomicTypeValueView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AtomicTypeValueMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AtomicTypeValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AtomicTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AtomicTypeValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AtomicTypeValueMut<'msg> {
  type Message = AtomicTypeValue;
}

impl ::std::fmt::Debug for AtomicTypeValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AtomicTypeValue>> for AtomicTypeValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AtomicTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AtomicTypeValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AtomicTypeValue> {
    self.inner
  }

  pub fn to_owned(&self) -> AtomicTypeValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // byte: optional int32
  pub fn has_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn byte_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.byte(), self.has_byte())
  }
  pub fn byte(&self) -> i32 {
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
  pub fn set_byte(&mut self, val: i32) {
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

  // int16: optional int32
  pub fn has_int16(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_int16(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn int16_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.int16(), self.has_int16())
  }
  pub fn int16(&self) -> i32 {
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
  pub fn set_int16(&mut self, val: i32) {
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

  // int32: optional int32
  pub fn has_int32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_int32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn int32_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.int32(), self.has_int32())
  }
  pub fn int32(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int32(&mut self, val: i32) {
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

  // int64: optional int64
  pub fn has_int64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_int64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn int64_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.int64(), self.has_int64())
  }
  pub fn int64(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int64(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // float: optional float
  pub fn has_float(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_float(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn float_opt(&self) -> ::protobuf::Optional<f32> {
        ::protobuf::Optional::new(self.float(), self.has_float())
  }
  pub fn float(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_float(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

  // double: optional double
  pub fn has_double(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_double(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn double_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.double(), self.has_double())
  }
  pub fn double(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_double(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // string: optional string
  pub fn has_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn string_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.string(), self.has_string())
  }
  pub fn string(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_string(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // boolean: optional bool
  pub fn has_boolean(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_boolean(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn boolean_opt(&self) -> ::protobuf::Optional<bool> {
        ::protobuf::Optional::new(self.boolean(), self.has_boolean())
  }
  pub fn boolean(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_boolean(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // bytes: optional bytes
  pub fn has_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn bytes_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.bytes(), self.has_bytes())
  }
  pub fn bytes(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_bytes(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  pub fn value(&self) -> super::atomic_type_value::ValueOneof<'_> {
    match &self.value_case() {
      super::atomic_type_value::ValueCase::Byte =>
          super::atomic_type_value::ValueOneof::Byte(self.byte()),
      super::atomic_type_value::ValueCase::Int16 =>
          super::atomic_type_value::ValueOneof::Int16(self.int16()),
      super::atomic_type_value::ValueCase::Int32 =>
          super::atomic_type_value::ValueOneof::Int32(self.int32()),
      super::atomic_type_value::ValueCase::Int64 =>
          super::atomic_type_value::ValueOneof::Int64(self.int64()),
      super::atomic_type_value::ValueCase::Float =>
          super::atomic_type_value::ValueOneof::Float(self.float()),
      super::atomic_type_value::ValueCase::Double =>
          super::atomic_type_value::ValueOneof::Double(self.double()),
      super::atomic_type_value::ValueCase::String =>
          super::atomic_type_value::ValueOneof::String(self.string()),
      super::atomic_type_value::ValueCase::Boolean =>
          super::atomic_type_value::ValueOneof::Boolean(self.boolean()),
      super::atomic_type_value::ValueCase::Bytes =>
          super::atomic_type_value::ValueOneof::Bytes(self.bytes()),
      _ => super::atomic_type_value::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(&self) -> super::atomic_type_value::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::atomic_type_value::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AtomicTypeValueMut` does not perform any shared mutation.
unsafe impl Send for AtomicTypeValueMut<'_> {}

// SAFETY:
// - `AtomicTypeValueMut` does not perform any shared mutation.
unsafe impl Sync for AtomicTypeValueMut<'_> {}

impl<'msg> ::protobuf::AsView for AtomicTypeValueMut<'msg> {
  type Proxied = AtomicTypeValue;
  fn as_view(&self) -> ::protobuf::View<'_, AtomicTypeValue> {
    AtomicTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AtomicTypeValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AtomicTypeValue>
  where
      'msg: 'shorter {
    AtomicTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AtomicTypeValueMut<'msg> {
  type MutProxied = AtomicTypeValue;
  fn as_mut(&mut self) -> AtomicTypeValueMut<'msg> {
    AtomicTypeValueMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AtomicTypeValueMut<'msg> {
  fn into_mut<'shorter>(self) -> AtomicTypeValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AtomicTypeValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AtomicTypeValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AtomicTypeValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AtomicTypeValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // byte: optional int32
  pub fn has_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn byte_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.byte(), self.has_byte())
  }
  pub fn byte(&self) -> i32 {
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
  pub fn set_byte(&mut self, val: i32) {
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

  // int16: optional int32
  pub fn has_int16(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_int16(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn int16_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.int16(), self.has_int16())
  }
  pub fn int16(&self) -> i32 {
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
  pub fn set_int16(&mut self, val: i32) {
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

  // int32: optional int32
  pub fn has_int32(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_int32(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn int32_opt(&self) -> ::protobuf::Optional<i32> {
        ::protobuf::Optional::new(self.int32(), self.has_int32())
  }
  pub fn int32(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int32(&mut self, val: i32) {
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

  // int64: optional int64
  pub fn has_int64(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_int64(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn int64_opt(&self) -> ::protobuf::Optional<i64> {
        ::protobuf::Optional::new(self.int64(), self.has_int64())
  }
  pub fn int64(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int64(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // float: optional float
  pub fn has_float(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_float(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn float_opt(&self) -> ::protobuf::Optional<f32> {
        ::protobuf::Optional::new(self.float(), self.has_float())
  }
  pub fn float(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_float(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

  // double: optional double
  pub fn has_double(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_double(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn double_opt(&self) -> ::protobuf::Optional<f64> {
        ::protobuf::Optional::new(self.double(), self.has_double())
  }
  pub fn double(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_double(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // string: optional string
  pub fn has_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn string_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.string(), self.has_string())
  }
  pub fn string(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_string(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // boolean: optional bool
  pub fn has_boolean(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_boolean(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn boolean_opt(&self) -> ::protobuf::Optional<bool> {
        ::protobuf::Optional::new(self.boolean(), self.has_boolean())
  }
  pub fn boolean(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_boolean(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // bytes: optional bytes
  pub fn has_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn bytes_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.bytes(), self.has_bytes())
  }
  pub fn bytes(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_bytes(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  pub fn value(&self) -> super::atomic_type_value::ValueOneof<'_> {
    match &self.value_case() {
      super::atomic_type_value::ValueCase::Byte =>
          super::atomic_type_value::ValueOneof::Byte(self.byte()),
      super::atomic_type_value::ValueCase::Int16 =>
          super::atomic_type_value::ValueOneof::Int16(self.int16()),
      super::atomic_type_value::ValueCase::Int32 =>
          super::atomic_type_value::ValueOneof::Int32(self.int32()),
      super::atomic_type_value::ValueCase::Int64 =>
          super::atomic_type_value::ValueOneof::Int64(self.int64()),
      super::atomic_type_value::ValueCase::Float =>
          super::atomic_type_value::ValueOneof::Float(self.float()),
      super::atomic_type_value::ValueCase::Double =>
          super::atomic_type_value::ValueOneof::Double(self.double()),
      super::atomic_type_value::ValueCase::String =>
          super::atomic_type_value::ValueOneof::String(self.string()),
      super::atomic_type_value::ValueCase::Boolean =>
          super::atomic_type_value::ValueOneof::Boolean(self.boolean()),
      super::atomic_type_value::ValueCase::Bytes =>
          super::atomic_type_value::ValueOneof::Bytes(self.bytes()),
      _ => super::atomic_type_value::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(&self) -> super::atomic_type_value::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::atomic_type_value::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl AtomicTypeValue

impl ::std::ops::Drop for AtomicTypeValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AtomicTypeValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AtomicTypeValue {
  type Proxied = Self;
  fn as_view(&self) -> AtomicTypeValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AtomicTypeValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AtomicTypeValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AtomicTypeValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__AtomicTypeValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(((+! 1T/0^!|#|$|%|&|(|)|*|+");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__AtomicTypeValue_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__AtomicTypeValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AtomicTypeValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AtomicTypeValue {
  type Msg = AtomicTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AtomicTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AtomicTypeValue {
  type Msg = AtomicTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AtomicTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AtomicTypeValueMut<'_> {
  type Msg = AtomicTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AtomicTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AtomicTypeValueMut<'_> {
  type Msg = AtomicTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AtomicTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AtomicTypeValueView<'_> {
  type Msg = AtomicTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AtomicTypeValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AtomicTypeValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod atomic_type_value {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ValueOneof<'msg> {
  Byte(i32) = 1,
  Int16(i32) = 2,
  Int32(i32) = 3,
  Int64(i64) = 4,
  Float(f32) = 5,
  Double(f64) = 6,
  String(&'msg ::protobuf::ProtoStr) = 7,
  Boolean(bool) = 8,
  Bytes(&'msg [u8]) = 9,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ValueCase {
  Byte = 1,
  Int16 = 2,
  Int32 = 3,
  Int64 = 4,
  Float = 5,
  Double = 6,
  String = 7,
  Boolean = 8,
  Bytes = 9,

  not_set = 0
}

impl ValueCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ValueCase> {
    match v {
      0 => Some(ValueCase::not_set),
      1 => Some(ValueCase::Byte),
      2 => Some(ValueCase::Int16),
      3 => Some(ValueCase::Int32),
      4 => Some(ValueCase::Int64),
      5 => Some(ValueCase::Float),
      6 => Some(ValueCase::Double),
      7 => Some(ValueCase::String),
      8 => Some(ValueCase::Boolean),
      9 => Some(ValueCase::Bytes),
      _ => None
    }
  }
}
}  // pub mod atomic_type_value


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__ArrayTypeValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ArrayTypeValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ArrayTypeValue>
}

impl ::protobuf::Message for ArrayTypeValue {}

impl ::std::default::Default for ArrayTypeValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ArrayTypeValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ArrayTypeValue` is `Sync` because it does not implement interior mutability.
//    Neither does `ArrayTypeValueMut`.
unsafe impl Sync for ArrayTypeValue {}

// SAFETY:
// - `ArrayTypeValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ArrayTypeValue {}

impl ::protobuf::Proxied for ArrayTypeValue {
  type View<'msg> = ArrayTypeValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ArrayTypeValue {}

impl ::protobuf::MutProxied for ArrayTypeValue {
  type Mut<'msg> = ArrayTypeValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArrayTypeValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArrayTypeValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ArrayTypeValueView<'msg> {
  type Message = ArrayTypeValue;
}

impl ::std::fmt::Debug for ArrayTypeValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ArrayTypeValueView<'_> {
  fn default() -> ArrayTypeValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayTypeValue>> for ArrayTypeValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ArrayTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArrayTypeValueView<'msg> {

  pub fn to_owned(&self) -> ArrayTypeValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // element: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn element(self) -> ::protobuf::RepeatedView<'msg, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ArrayTypeValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArrayTypeValueView<'_> {}

// SAFETY:
// - `ArrayTypeValueView` is `Send` because while its alive a `ArrayTypeValueMut` cannot.
// - `ArrayTypeValueView` does not use thread-local data.
unsafe impl Send for ArrayTypeValueView<'_> {}

impl<'msg> ::protobuf::AsView for ArrayTypeValueView<'msg> {
  type Proxied = ArrayTypeValue;
  fn as_view(&self) -> ::protobuf::View<'msg, ArrayTypeValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArrayTypeValueView<'msg> {
  fn into_view<'shorter>(self) -> ArrayTypeValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ArrayTypeValue> for ArrayTypeValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArrayTypeValue {
    let mut dst = ArrayTypeValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ArrayTypeValue> for ArrayTypeValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ArrayTypeValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ArrayTypeValue {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArrayTypeValueView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ArrayTypeValueMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArrayTypeValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ArrayTypeValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ArrayTypeValueMut<'msg> {
  type Message = ArrayTypeValue;
}

impl ::std::fmt::Debug for ArrayTypeValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayTypeValue>> for ArrayTypeValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ArrayTypeValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ArrayTypeValue> {
    self.inner
  }

  pub fn to_owned(&self) -> ArrayTypeValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // element: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn element(&self) -> ::protobuf::RepeatedView<'_, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn element_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FieldValue> {
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
  pub fn set_element(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FieldValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ArrayTypeValueMut` does not perform any shared mutation.
unsafe impl Send for ArrayTypeValueMut<'_> {}

// SAFETY:
// - `ArrayTypeValueMut` does not perform any shared mutation.
unsafe impl Sync for ArrayTypeValueMut<'_> {}

impl<'msg> ::protobuf::AsView for ArrayTypeValueMut<'msg> {
  type Proxied = ArrayTypeValue;
  fn as_view(&self) -> ::protobuf::View<'_, ArrayTypeValue> {
    ArrayTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ArrayTypeValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ArrayTypeValue>
  where
      'msg: 'shorter {
    ArrayTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ArrayTypeValueMut<'msg> {
  type MutProxied = ArrayTypeValue;
  fn as_mut(&mut self) -> ArrayTypeValueMut<'msg> {
    ArrayTypeValueMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ArrayTypeValueMut<'msg> {
  fn into_mut<'shorter>(self) -> ArrayTypeValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ArrayTypeValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ArrayTypeValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ArrayTypeValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ArrayTypeValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // element: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn element(&self) -> ::protobuf::RepeatedView<'_, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn element_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FieldValue> {
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
  pub fn set_element(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FieldValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ArrayTypeValue

impl ::std::ops::Drop for ArrayTypeValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ArrayTypeValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ArrayTypeValue {
  type Proxied = Self;
  fn as_view(&self) -> ArrayTypeValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ArrayTypeValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArrayTypeValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArrayTypeValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__ArrayTypeValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333333^!|#|$|%|&|(");
        super::org__apache__beam__model__pipeline__v1__IterableTypeValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::org__apache__beam__model__pipeline__v1__LogicalTypeValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        super::org__apache__beam__model__pipeline__v1__MapTypeEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        super::org__apache__beam__model__pipeline__v1__MapTypeValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::org__apache__beam__model__pipeline__v1__Row_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__ArrayTypeValue_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0, &[<super::AtomicTypeValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::org__apache__beam__model__pipeline__v1__ArrayTypeValue_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__IterableTypeValue_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__MapTypeValue_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__Row_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__LogicalTypeValue_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__IterableTypeValue_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__LogicalTypeValue_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MapTypeEntry_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0,
            super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__MapTypeValue_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__MapTypeEntry_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__Row_msg_init.0, &[super::org__apache__beam__model__pipeline__v1__FieldValue_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__ArrayTypeValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArrayTypeValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArrayTypeValue {
  type Msg = ArrayTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayTypeValue {
  type Msg = ArrayTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ArrayTypeValueMut<'_> {
  type Msg = ArrayTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayTypeValueMut<'_> {
  type Msg = ArrayTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ArrayTypeValueView<'_> {
  type Msg = ArrayTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ArrayTypeValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ArrayTypeValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__IterableTypeValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct IterableTypeValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<IterableTypeValue>
}

impl ::protobuf::Message for IterableTypeValue {}

impl ::std::default::Default for IterableTypeValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for IterableTypeValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `IterableTypeValue` is `Sync` because it does not implement interior mutability.
//    Neither does `IterableTypeValueMut`.
unsafe impl Sync for IterableTypeValue {}

// SAFETY:
// - `IterableTypeValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IterableTypeValue {}

impl ::protobuf::Proxied for IterableTypeValue {
  type View<'msg> = IterableTypeValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for IterableTypeValue {}

impl ::protobuf::MutProxied for IterableTypeValue {
  type Mut<'msg> = IterableTypeValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IterableTypeValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IterableTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IterableTypeValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for IterableTypeValueView<'msg> {
  type Message = IterableTypeValue;
}

impl ::std::fmt::Debug for IterableTypeValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for IterableTypeValueView<'_> {
  fn default() -> IterableTypeValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, IterableTypeValue>> for IterableTypeValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IterableTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IterableTypeValueView<'msg> {

  pub fn to_owned(&self) -> IterableTypeValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // element: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn element(self) -> ::protobuf::RepeatedView<'msg, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `IterableTypeValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for IterableTypeValueView<'_> {}

// SAFETY:
// - `IterableTypeValueView` is `Send` because while its alive a `IterableTypeValueMut` cannot.
// - `IterableTypeValueView` does not use thread-local data.
unsafe impl Send for IterableTypeValueView<'_> {}

impl<'msg> ::protobuf::AsView for IterableTypeValueView<'msg> {
  type Proxied = IterableTypeValue;
  fn as_view(&self) -> ::protobuf::View<'msg, IterableTypeValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IterableTypeValueView<'msg> {
  fn into_view<'shorter>(self) -> IterableTypeValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<IterableTypeValue> for IterableTypeValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IterableTypeValue {
    let mut dst = IterableTypeValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<IterableTypeValue> for IterableTypeValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IterableTypeValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for IterableTypeValue {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for IterableTypeValueView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for IterableTypeValueMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IterableTypeValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IterableTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IterableTypeValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for IterableTypeValueMut<'msg> {
  type Message = IterableTypeValue;
}

impl ::std::fmt::Debug for IterableTypeValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, IterableTypeValue>> for IterableTypeValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IterableTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IterableTypeValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, IterableTypeValue> {
    self.inner
  }

  pub fn to_owned(&self) -> IterableTypeValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // element: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn element(&self) -> ::protobuf::RepeatedView<'_, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn element_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FieldValue> {
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
  pub fn set_element(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FieldValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `IterableTypeValueMut` does not perform any shared mutation.
unsafe impl Send for IterableTypeValueMut<'_> {}

// SAFETY:
// - `IterableTypeValueMut` does not perform any shared mutation.
unsafe impl Sync for IterableTypeValueMut<'_> {}

impl<'msg> ::protobuf::AsView for IterableTypeValueMut<'msg> {
  type Proxied = IterableTypeValue;
  fn as_view(&self) -> ::protobuf::View<'_, IterableTypeValue> {
    IterableTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IterableTypeValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, IterableTypeValue>
  where
      'msg: 'shorter {
    IterableTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for IterableTypeValueMut<'msg> {
  type MutProxied = IterableTypeValue;
  fn as_mut(&mut self) -> IterableTypeValueMut<'msg> {
    IterableTypeValueMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for IterableTypeValueMut<'msg> {
  fn into_mut<'shorter>(self) -> IterableTypeValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl IterableTypeValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, IterableTypeValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> IterableTypeValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> IterableTypeValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // element: repeated message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn element(&self) -> ::protobuf::RepeatedView<'_, super::FieldValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FieldValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn element_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FieldValue> {
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
  pub fn set_element(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FieldValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl IterableTypeValue

impl ::std::ops::Drop for IterableTypeValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for IterableTypeValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for IterableTypeValue {
  type Proxied = Self;
  fn as_view(&self) -> IterableTypeValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for IterableTypeValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> IterableTypeValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for IterableTypeValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayTypeValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__IterableTypeValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IterableTypeValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IterableTypeValue {
  type Msg = IterableTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IterableTypeValue {
  type Msg = IterableTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IterableTypeValueMut<'_> {
  type Msg = IterableTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IterableTypeValueMut<'_> {
  type Msg = IterableTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IterableTypeValueView<'_> {
  type Msg = IterableTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IterableTypeValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IterableTypeValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MapTypeValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MapTypeValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MapTypeValue>
}

impl ::protobuf::Message for MapTypeValue {}

impl ::std::default::Default for MapTypeValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MapTypeValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MapTypeValue` is `Sync` because it does not implement interior mutability.
//    Neither does `MapTypeValueMut`.
unsafe impl Sync for MapTypeValue {}

// SAFETY:
// - `MapTypeValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MapTypeValue {}

impl ::protobuf::Proxied for MapTypeValue {
  type View<'msg> = MapTypeValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MapTypeValue {}

impl ::protobuf::MutProxied for MapTypeValue {
  type Mut<'msg> = MapTypeValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MapTypeValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MapTypeValueView<'msg> {
  type Message = MapTypeValue;
}

impl ::std::fmt::Debug for MapTypeValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MapTypeValueView<'_> {
  fn default() -> MapTypeValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MapTypeValue>> for MapTypeValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeValueView<'msg> {

  pub fn to_owned(&self) -> MapTypeValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entries: repeated message org.apache.beam.model.pipeline.v1.MapTypeEntry
  pub fn entries(self) -> ::protobuf::RepeatedView<'msg, super::MapTypeEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MapTypeEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MapTypeValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for MapTypeValueView<'_> {}

// SAFETY:
// - `MapTypeValueView` is `Send` because while its alive a `MapTypeValueMut` cannot.
// - `MapTypeValueView` does not use thread-local data.
unsafe impl Send for MapTypeValueView<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeValueView<'msg> {
  type Proxied = MapTypeValue;
  fn as_view(&self) -> ::protobuf::View<'msg, MapTypeValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeValueView<'msg> {
  fn into_view<'shorter>(self) -> MapTypeValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MapTypeValue> for MapTypeValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapTypeValue {
    let mut dst = MapTypeValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MapTypeValue> for MapTypeValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapTypeValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MapTypeValue {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MapTypeValueView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MapTypeValueMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MapTypeValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MapTypeValueMut<'msg> {
  type Message = MapTypeValue;
}

impl ::std::fmt::Debug for MapTypeValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeValue>> for MapTypeValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeValue> {
    self.inner
  }

  pub fn to_owned(&self) -> MapTypeValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // entries: repeated message org.apache.beam.model.pipeline.v1.MapTypeEntry
  pub fn entries(&self) -> ::protobuf::RepeatedView<'_, super::MapTypeEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MapTypeEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::MapTypeEntry> {
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
  pub fn set_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::MapTypeEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `MapTypeValueMut` does not perform any shared mutation.
unsafe impl Send for MapTypeValueMut<'_> {}

// SAFETY:
// - `MapTypeValueMut` does not perform any shared mutation.
unsafe impl Sync for MapTypeValueMut<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeValueMut<'msg> {
  type Proxied = MapTypeValue;
  fn as_view(&self) -> ::protobuf::View<'_, MapTypeValue> {
    MapTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MapTypeValue>
  where
      'msg: 'shorter {
    MapTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MapTypeValueMut<'msg> {
  type MutProxied = MapTypeValue;
  fn as_mut(&mut self) -> MapTypeValueMut<'msg> {
    MapTypeValueMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MapTypeValueMut<'msg> {
  fn into_mut<'shorter>(self) -> MapTypeValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MapTypeValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MapTypeValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MapTypeValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MapTypeValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // entries: repeated message org.apache.beam.model.pipeline.v1.MapTypeEntry
  pub fn entries(&self) -> ::protobuf::RepeatedView<'_, super::MapTypeEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::MapTypeEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::MapTypeEntry> {
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
  pub fn set_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::MapTypeEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl MapTypeValue

impl ::std::ops::Drop for MapTypeValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MapTypeValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MapTypeValue {
  type Proxied = Self;
  fn as_view(&self) -> MapTypeValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MapTypeValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MapTypeValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MapTypeValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayTypeValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MapTypeValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapTypeValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapTypeValue {
  type Msg = MapTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeValue {
  type Msg = MapTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapTypeValueMut<'_> {
  type Msg = MapTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeValueMut<'_> {
  type Msg = MapTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeValueView<'_> {
  type Msg = MapTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapTypeValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__MapTypeEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MapTypeEntry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MapTypeEntry>
}

impl ::protobuf::Message for MapTypeEntry {}

impl ::std::default::Default for MapTypeEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MapTypeEntry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MapTypeEntry` is `Sync` because it does not implement interior mutability.
//    Neither does `MapTypeEntryMut`.
unsafe impl Sync for MapTypeEntry {}

// SAFETY:
// - `MapTypeEntry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MapTypeEntry {}

impl ::protobuf::Proxied for MapTypeEntry {
  type View<'msg> = MapTypeEntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MapTypeEntry {}

impl ::protobuf::MutProxied for MapTypeEntry {
  type Mut<'msg> = MapTypeEntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MapTypeEntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapTypeEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeEntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MapTypeEntryView<'msg> {
  type Message = MapTypeEntry;
}

impl ::std::fmt::Debug for MapTypeEntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MapTypeEntryView<'_> {
  fn default() -> MapTypeEntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MapTypeEntry>> for MapTypeEntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MapTypeEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeEntryView<'msg> {

  pub fn to_owned(&self) -> MapTypeEntry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn key_opt(self) -> ::protobuf::Optional<super::FieldValueView<'msg>> {
        ::protobuf::Optional::new(self.key(), self.has_key())
  }
  pub fn key(self) -> super::FieldValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_opt(self) -> ::protobuf::Optional<super::FieldValueView<'msg>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(self) -> super::FieldValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }

}

// SAFETY:
// - `MapTypeEntryView` is `Sync` because it does not support mutation.
unsafe impl Sync for MapTypeEntryView<'_> {}

// SAFETY:
// - `MapTypeEntryView` is `Send` because while its alive a `MapTypeEntryMut` cannot.
// - `MapTypeEntryView` does not use thread-local data.
unsafe impl Send for MapTypeEntryView<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeEntryView<'msg> {
  type Proxied = MapTypeEntry;
  fn as_view(&self) -> ::protobuf::View<'msg, MapTypeEntry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeEntryView<'msg> {
  fn into_view<'shorter>(self) -> MapTypeEntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MapTypeEntry> for MapTypeEntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapTypeEntry {
    let mut dst = MapTypeEntry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MapTypeEntry> for MapTypeEntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MapTypeEntry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MapTypeEntry {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MapTypeEntryView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MapTypeEntryMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MapTypeEntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MapTypeEntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MapTypeEntryMut<'msg> {
  type Message = MapTypeEntry;
}

impl ::std::fmt::Debug for MapTypeEntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeEntry>> for MapTypeEntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MapTypeEntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MapTypeEntry> {
    self.inner
  }

  pub fn to_owned(&self) -> MapTypeEntry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.key(), self.has_key())
  }
  pub fn key(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn key_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_key(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn value_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

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
// - `MapTypeEntryMut` does not perform any shared mutation.
unsafe impl Send for MapTypeEntryMut<'_> {}

// SAFETY:
// - `MapTypeEntryMut` does not perform any shared mutation.
unsafe impl Sync for MapTypeEntryMut<'_> {}

impl<'msg> ::protobuf::AsView for MapTypeEntryMut<'msg> {
  type Proxied = MapTypeEntry;
  fn as_view(&self) -> ::protobuf::View<'_, MapTypeEntry> {
    MapTypeEntryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MapTypeEntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MapTypeEntry>
  where
      'msg: 'shorter {
    MapTypeEntryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MapTypeEntryMut<'msg> {
  type MutProxied = MapTypeEntry;
  fn as_mut(&mut self) -> MapTypeEntryMut<'msg> {
    MapTypeEntryMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MapTypeEntryMut<'msg> {
  fn into_mut<'shorter>(self) -> MapTypeEntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MapTypeEntry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MapTypeEntry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MapTypeEntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MapTypeEntryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.key(), self.has_key())
  }
  pub fn key(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn key_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_key(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn value_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl MapTypeEntry

impl ::std::ops::Drop for MapTypeEntry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MapTypeEntry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MapTypeEntry {
  type Proxied = Self;
  fn as_view(&self) -> MapTypeEntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MapTypeEntry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MapTypeEntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MapTypeEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayTypeValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__MapTypeEntry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapTypeEntry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapTypeEntry {
  type Msg = MapTypeEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeEntry {
  type Msg = MapTypeEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MapTypeEntryMut<'_> {
  type Msg = MapTypeEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeEntryMut<'_> {
  type Msg = MapTypeEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MapTypeEntryView<'_> {
  type Msg = MapTypeEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MapTypeEntry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MapTypeEntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__LogicalTypeValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LogicalTypeValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LogicalTypeValue>
}

impl ::protobuf::Message for LogicalTypeValue {}

impl ::std::default::Default for LogicalTypeValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LogicalTypeValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LogicalTypeValue` is `Sync` because it does not implement interior mutability.
//    Neither does `LogicalTypeValueMut`.
unsafe impl Sync for LogicalTypeValue {}

// SAFETY:
// - `LogicalTypeValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for LogicalTypeValue {}

impl ::protobuf::Proxied for LogicalTypeValue {
  type View<'msg> = LogicalTypeValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LogicalTypeValue {}

impl ::protobuf::MutProxied for LogicalTypeValue {
  type Mut<'msg> = LogicalTypeValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LogicalTypeValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogicalTypeValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LogicalTypeValueView<'msg> {
  type Message = LogicalTypeValue;
}

impl ::std::fmt::Debug for LogicalTypeValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LogicalTypeValueView<'_> {
  fn default() -> LogicalTypeValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalTypeValue>> for LogicalTypeValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogicalTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogicalTypeValueView<'msg> {

  pub fn to_owned(&self) -> LogicalTypeValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn value_opt(self) -> ::protobuf::Optional<super::FieldValueView<'msg>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(self) -> super::FieldValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }

}

// SAFETY:
// - `LogicalTypeValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for LogicalTypeValueView<'_> {}

// SAFETY:
// - `LogicalTypeValueView` is `Send` because while its alive a `LogicalTypeValueMut` cannot.
// - `LogicalTypeValueView` does not use thread-local data.
unsafe impl Send for LogicalTypeValueView<'_> {}

impl<'msg> ::protobuf::AsView for LogicalTypeValueView<'msg> {
  type Proxied = LogicalTypeValue;
  fn as_view(&self) -> ::protobuf::View<'msg, LogicalTypeValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogicalTypeValueView<'msg> {
  fn into_view<'shorter>(self) -> LogicalTypeValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LogicalTypeValue> for LogicalTypeValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogicalTypeValue {
    let mut dst = LogicalTypeValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LogicalTypeValue> for LogicalTypeValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogicalTypeValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for LogicalTypeValue {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LogicalTypeValueView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for LogicalTypeValueMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LogicalTypeValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypeValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogicalTypeValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LogicalTypeValueMut<'msg> {
  type Message = LogicalTypeValue;
}

impl ::std::fmt::Debug for LogicalTypeValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypeValue>> for LogicalTypeValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypeValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogicalTypeValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LogicalTypeValue> {
    self.inner
  }

  pub fn to_owned(&self) -> LogicalTypeValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn value_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn value_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

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
// - `LogicalTypeValueMut` does not perform any shared mutation.
unsafe impl Send for LogicalTypeValueMut<'_> {}

// SAFETY:
// - `LogicalTypeValueMut` does not perform any shared mutation.
unsafe impl Sync for LogicalTypeValueMut<'_> {}

impl<'msg> ::protobuf::AsView for LogicalTypeValueMut<'msg> {
  type Proxied = LogicalTypeValue;
  fn as_view(&self) -> ::protobuf::View<'_, LogicalTypeValue> {
    LogicalTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogicalTypeValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LogicalTypeValue>
  where
      'msg: 'shorter {
    LogicalTypeValueView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for LogicalTypeValueMut<'msg> {
  type MutProxied = LogicalTypeValue;
  fn as_mut(&mut self) -> LogicalTypeValueMut<'msg> {
    LogicalTypeValueMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LogicalTypeValueMut<'msg> {
  fn into_mut<'shorter>(self) -> LogicalTypeValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LogicalTypeValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LogicalTypeValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LogicalTypeValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LogicalTypeValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // value: optional message org.apache.beam.model.pipeline.v1.FieldValue
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn value_opt(&self) -> ::protobuf::Optional<super::FieldValueView<'_>> {
        ::protobuf::Optional::new(self.value(), self.has_value())
  }
  pub fn value(&self) -> super::FieldValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FieldValueView::default())
  }
  pub fn value_mut(&mut self) -> super::FieldValueMut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<super::FieldValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl LogicalTypeValue

impl ::std::ops::Drop for LogicalTypeValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LogicalTypeValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LogicalTypeValue {
  type Proxied = Self;
  fn as_view(&self) -> LogicalTypeValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LogicalTypeValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LogicalTypeValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LogicalTypeValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ArrayTypeValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__LogicalTypeValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogicalTypeValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogicalTypeValue {
  type Msg = LogicalTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypeValue {
  type Msg = LogicalTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogicalTypeValueMut<'_> {
  type Msg = LogicalTypeValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypeValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypeValueMut<'_> {
  type Msg = LogicalTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypeValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogicalTypeValueView<'_> {
  type Msg = LogicalTypeValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogicalTypeValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogicalTypeValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__SchemaCoderPayload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SchemaCoderPayload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SchemaCoderPayload>
}

impl ::protobuf::Message for SchemaCoderPayload {}

impl ::std::default::Default for SchemaCoderPayload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SchemaCoderPayload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SchemaCoderPayload` is `Sync` because it does not implement interior mutability.
//    Neither does `SchemaCoderPayloadMut`.
unsafe impl Sync for SchemaCoderPayload {}

// SAFETY:
// - `SchemaCoderPayload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SchemaCoderPayload {}

impl ::protobuf::Proxied for SchemaCoderPayload {
  type View<'msg> = SchemaCoderPayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SchemaCoderPayload {}

impl ::protobuf::MutProxied for SchemaCoderPayload {
  type Mut<'msg> = SchemaCoderPayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SchemaCoderPayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaCoderPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaCoderPayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SchemaCoderPayloadView<'msg> {
  type Message = SchemaCoderPayload;
}

impl ::std::fmt::Debug for SchemaCoderPayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SchemaCoderPayloadView<'_> {
  fn default() -> SchemaCoderPayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaCoderPayload>> for SchemaCoderPayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaCoderPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaCoderPayloadView<'msg> {

  pub fn to_owned(&self) -> SchemaCoderPayload {
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

  // to_row_fn: optional message org.apache.beam.model.pipeline.v1.FunctionSpec
  pub fn has_to_row_fn(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn to_row_fn_opt(self) -> ::protobuf::Optional<super::FunctionSpecView<'msg>> {
        ::protobuf::Optional::new(self.to_row_fn(), self.has_to_row_fn())
  }
  pub fn to_row_fn(self) -> super::FunctionSpecView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FunctionSpecView::default())
  }

  // from_row_fn: optional message org.apache.beam.model.pipeline.v1.FunctionSpec
  pub fn has_from_row_fn(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn from_row_fn_opt(self) -> ::protobuf::Optional<super::FunctionSpecView<'msg>> {
        ::protobuf::Optional::new(self.from_row_fn(), self.has_from_row_fn())
  }
  pub fn from_row_fn(self) -> super::FunctionSpecView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FunctionSpecView::default())
  }

  // additional_coder_infos: repeated message org.apache.beam.model.pipeline.v1.SchemaCoderPayload.AdditionalCoderInfo
  pub fn additional_coder_infos(self) -> ::protobuf::RepeatedView<'msg, super::schema_coder_payload::AdditionalCoderInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::schema_coder_payload::AdditionalCoderInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SchemaCoderPayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for SchemaCoderPayloadView<'_> {}

// SAFETY:
// - `SchemaCoderPayloadView` is `Send` because while its alive a `SchemaCoderPayloadMut` cannot.
// - `SchemaCoderPayloadView` does not use thread-local data.
unsafe impl Send for SchemaCoderPayloadView<'_> {}

impl<'msg> ::protobuf::AsView for SchemaCoderPayloadView<'msg> {
  type Proxied = SchemaCoderPayload;
  fn as_view(&self) -> ::protobuf::View<'msg, SchemaCoderPayload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaCoderPayloadView<'msg> {
  fn into_view<'shorter>(self) -> SchemaCoderPayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaCoderPayload> for SchemaCoderPayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaCoderPayload {
    let mut dst = SchemaCoderPayload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaCoderPayload> for SchemaCoderPayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaCoderPayload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SchemaCoderPayload {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaCoderPayloadView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaCoderPayloadMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SchemaCoderPayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaCoderPayload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaCoderPayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SchemaCoderPayloadMut<'msg> {
  type Message = SchemaCoderPayload;
}

impl ::std::fmt::Debug for SchemaCoderPayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaCoderPayload>> for SchemaCoderPayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaCoderPayload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaCoderPayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaCoderPayload> {
    self.inner
  }

  pub fn to_owned(&self) -> SchemaCoderPayload {
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

  // to_row_fn: optional message org.apache.beam.model.pipeline.v1.FunctionSpec
  pub fn has_to_row_fn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_to_row_fn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn to_row_fn_opt(&self) -> ::protobuf::Optional<super::FunctionSpecView<'_>> {
        ::protobuf::Optional::new(self.to_row_fn(), self.has_to_row_fn())
  }
  pub fn to_row_fn(&self) -> super::FunctionSpecView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FunctionSpecView::default())
  }
  pub fn to_row_fn_mut(&mut self) -> super::FunctionSpecMut<'_> {
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
  pub fn set_to_row_fn(&mut self,
    val: impl ::protobuf::IntoProxied<super::FunctionSpec>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // from_row_fn: optional message org.apache.beam.model.pipeline.v1.FunctionSpec
  pub fn has_from_row_fn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_from_row_fn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn from_row_fn_opt(&self) -> ::protobuf::Optional<super::FunctionSpecView<'_>> {
        ::protobuf::Optional::new(self.from_row_fn(), self.has_from_row_fn())
  }
  pub fn from_row_fn(&self) -> super::FunctionSpecView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FunctionSpecView::default())
  }
  pub fn from_row_fn_mut(&mut self) -> super::FunctionSpecMut<'_> {
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
  pub fn set_from_row_fn(&mut self,
    val: impl ::protobuf::IntoProxied<super::FunctionSpec>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // additional_coder_infos: repeated message org.apache.beam.model.pipeline.v1.SchemaCoderPayload.AdditionalCoderInfo
  pub fn additional_coder_infos(&self) -> ::protobuf::RepeatedView<'_, super::schema_coder_payload::AdditionalCoderInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::schema_coder_payload::AdditionalCoderInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_coder_infos_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::schema_coder_payload::AdditionalCoderInfo> {
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
  pub fn set_additional_coder_infos(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::schema_coder_payload::AdditionalCoderInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `SchemaCoderPayloadMut` does not perform any shared mutation.
unsafe impl Send for SchemaCoderPayloadMut<'_> {}

// SAFETY:
// - `SchemaCoderPayloadMut` does not perform any shared mutation.
unsafe impl Sync for SchemaCoderPayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for SchemaCoderPayloadMut<'msg> {
  type Proxied = SchemaCoderPayload;
  fn as_view(&self) -> ::protobuf::View<'_, SchemaCoderPayload> {
    SchemaCoderPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaCoderPayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SchemaCoderPayload>
  where
      'msg: 'shorter {
    SchemaCoderPayloadView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SchemaCoderPayloadMut<'msg> {
  type MutProxied = SchemaCoderPayload;
  fn as_mut(&mut self) -> SchemaCoderPayloadMut<'msg> {
    SchemaCoderPayloadMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SchemaCoderPayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> SchemaCoderPayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SchemaCoderPayload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SchemaCoderPayload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SchemaCoderPayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SchemaCoderPayloadMut<'_> {
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

  // to_row_fn: optional message org.apache.beam.model.pipeline.v1.FunctionSpec
  pub fn has_to_row_fn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_to_row_fn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn to_row_fn_opt(&self) -> ::protobuf::Optional<super::FunctionSpecView<'_>> {
        ::protobuf::Optional::new(self.to_row_fn(), self.has_to_row_fn())
  }
  pub fn to_row_fn(&self) -> super::FunctionSpecView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FunctionSpecView::default())
  }
  pub fn to_row_fn_mut(&mut self) -> super::FunctionSpecMut<'_> {
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
  pub fn set_to_row_fn(&mut self,
    val: impl ::protobuf::IntoProxied<super::FunctionSpec>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // from_row_fn: optional message org.apache.beam.model.pipeline.v1.FunctionSpec
  pub fn has_from_row_fn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_from_row_fn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn from_row_fn_opt(&self) -> ::protobuf::Optional<super::FunctionSpecView<'_>> {
        ::protobuf::Optional::new(self.from_row_fn(), self.has_from_row_fn())
  }
  pub fn from_row_fn(&self) -> super::FunctionSpecView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FunctionSpecView::default())
  }
  pub fn from_row_fn_mut(&mut self) -> super::FunctionSpecMut<'_> {
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
  pub fn set_from_row_fn(&mut self,
    val: impl ::protobuf::IntoProxied<super::FunctionSpec>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // additional_coder_infos: repeated message org.apache.beam.model.pipeline.v1.SchemaCoderPayload.AdditionalCoderInfo
  pub fn additional_coder_infos(&self) -> ::protobuf::RepeatedView<'_, super::schema_coder_payload::AdditionalCoderInfo> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::schema_coder_payload::AdditionalCoderInfo>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_coder_infos_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::schema_coder_payload::AdditionalCoderInfo> {
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
  pub fn set_additional_coder_infos(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::schema_coder_payload::AdditionalCoderInfo>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl SchemaCoderPayload

impl ::std::ops::Drop for SchemaCoderPayload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SchemaCoderPayload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SchemaCoderPayload {
  type Proxied = Self;
  fn as_view(&self) -> SchemaCoderPayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SchemaCoderPayload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SchemaCoderPayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SchemaCoderPayload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::org__apache__beam__model__pipeline__v1__SchemaCoderPayload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::org__apache__beam__model__pipeline__v1__SchemaCoderPayload_msg_init.0, &[<super::Schema as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::FunctionSpec as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::FunctionSpec as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::schema_coder_payload::AdditionalCoderInfo as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::org__apache__beam__model__pipeline__v1__SchemaCoderPayload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaCoderPayload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaCoderPayload {
  type Msg = SchemaCoderPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaCoderPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaCoderPayload {
  type Msg = SchemaCoderPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaCoderPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaCoderPayloadMut<'_> {
  type Msg = SchemaCoderPayload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaCoderPayload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaCoderPayloadMut<'_> {
  type Msg = SchemaCoderPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaCoderPayload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaCoderPayloadView<'_> {
  type Msg = SchemaCoderPayload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaCoderPayload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaCoderPayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod schema_coder_payload {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut org__apache__beam__model__pipeline__v1__SchemaCoderPayload__AdditionalCoderInfo_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AdditionalCoderInfo {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AdditionalCoderInfo>
}

impl ::protobuf::Message for AdditionalCoderInfo {}

impl ::std::default::Default for AdditionalCoderInfo {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AdditionalCoderInfo {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AdditionalCoderInfo` is `Sync` because it does not implement interior mutability.
//    Neither does `AdditionalCoderInfoMut`.
unsafe impl Sync for AdditionalCoderInfo {}

// SAFETY:
// - `AdditionalCoderInfo` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AdditionalCoderInfo {}

impl ::protobuf::Proxied for AdditionalCoderInfo {
  type View<'msg> = AdditionalCoderInfoView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AdditionalCoderInfo {}

impl ::protobuf::MutProxied for AdditionalCoderInfo {
  type Mut<'msg> = AdditionalCoderInfoMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AdditionalCoderInfoView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdditionalCoderInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdditionalCoderInfoView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AdditionalCoderInfoView<'msg> {
  type Message = AdditionalCoderInfo;
}

impl ::std::fmt::Debug for AdditionalCoderInfoView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AdditionalCoderInfoView<'_> {
  fn default() -> AdditionalCoderInfoView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AdditionalCoderInfo>> for AdditionalCoderInfoView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdditionalCoderInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdditionalCoderInfoView<'msg> {

  pub fn to_owned(&self) -> AdditionalCoderInfo {
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
// - `AdditionalCoderInfoView` is `Sync` because it does not support mutation.
unsafe impl Sync for AdditionalCoderInfoView<'_> {}

// SAFETY:
// - `AdditionalCoderInfoView` is `Send` because while its alive a `AdditionalCoderInfoMut` cannot.
// - `AdditionalCoderInfoView` does not use thread-local data.
unsafe impl Send for AdditionalCoderInfoView<'_> {}

impl<'msg> ::protobuf::AsView for AdditionalCoderInfoView<'msg> {
  type Proxied = AdditionalCoderInfo;
  fn as_view(&self) -> ::protobuf::View<'msg, AdditionalCoderInfo> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdditionalCoderInfoView<'msg> {
  fn into_view<'shorter>(self) -> AdditionalCoderInfoView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AdditionalCoderInfo> for AdditionalCoderInfoView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdditionalCoderInfo {
    let mut dst = AdditionalCoderInfo::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AdditionalCoderInfo> for AdditionalCoderInfoMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdditionalCoderInfo {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AdditionalCoderInfo {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AdditionalCoderInfoView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AdditionalCoderInfoMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AdditionalCoderInfoMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalCoderInfo>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdditionalCoderInfoMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AdditionalCoderInfoMut<'msg> {
  type Message = AdditionalCoderInfo;
}

impl ::std::fmt::Debug for AdditionalCoderInfoMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalCoderInfo>> for AdditionalCoderInfoMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalCoderInfo>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdditionalCoderInfoMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalCoderInfo> {
    self.inner
  }

  pub fn to_owned(&self) -> AdditionalCoderInfo {
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
// - `AdditionalCoderInfoMut` does not perform any shared mutation.
unsafe impl Send for AdditionalCoderInfoMut<'_> {}

// SAFETY:
// - `AdditionalCoderInfoMut` does not perform any shared mutation.
unsafe impl Sync for AdditionalCoderInfoMut<'_> {}

impl<'msg> ::protobuf::AsView for AdditionalCoderInfoMut<'msg> {
  type Proxied = AdditionalCoderInfo;
  fn as_view(&self) -> ::protobuf::View<'_, AdditionalCoderInfo> {
    AdditionalCoderInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdditionalCoderInfoMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AdditionalCoderInfo>
  where
      'msg: 'shorter {
    AdditionalCoderInfoView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AdditionalCoderInfoMut<'msg> {
  type MutProxied = AdditionalCoderInfo;
  fn as_mut(&mut self) -> AdditionalCoderInfoMut<'msg> {
    AdditionalCoderInfoMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AdditionalCoderInfoMut<'msg> {
  fn into_mut<'shorter>(self) -> AdditionalCoderInfoMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AdditionalCoderInfo {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AdditionalCoderInfo> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AdditionalCoderInfoView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AdditionalCoderInfoMut<'_> {
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

}  // impl AdditionalCoderInfo

impl ::std::ops::Drop for AdditionalCoderInfo {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AdditionalCoderInfo {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AdditionalCoderInfo {
  type Proxied = Self;
  fn as_view(&self) -> AdditionalCoderInfoView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AdditionalCoderInfo {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AdditionalCoderInfoMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AdditionalCoderInfo {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::schema_coder_payload::org__apache__beam__model__pipeline__v1__SchemaCoderPayload__AdditionalCoderInfo_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::schema_coder_payload::org__apache__beam__model__pipeline__v1__SchemaCoderPayload__AdditionalCoderInfo_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::schema_coder_payload::org__apache__beam__model__pipeline__v1__SchemaCoderPayload__AdditionalCoderInfo_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdditionalCoderInfo {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdditionalCoderInfo {
  type Msg = AdditionalCoderInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalCoderInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdditionalCoderInfo {
  type Msg = AdditionalCoderInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalCoderInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdditionalCoderInfoMut<'_> {
  type Msg = AdditionalCoderInfo;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalCoderInfo> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdditionalCoderInfoMut<'_> {
  type Msg = AdditionalCoderInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalCoderInfo> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdditionalCoderInfoView<'_> {
  type Msg = AdditionalCoderInfo;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalCoderInfo> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdditionalCoderInfoMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod schema_coder_payload


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AtomicType(i32);

#[allow(non_upper_case_globals)]
impl AtomicType {
  pub const Unspecified: AtomicType = AtomicType(0);
  pub const Byte: AtomicType = AtomicType(1);
  pub const Int16: AtomicType = AtomicType(2);
  pub const Int32: AtomicType = AtomicType(3);
  pub const Int64: AtomicType = AtomicType(4);
  pub const Float: AtomicType = AtomicType(5);
  pub const Double: AtomicType = AtomicType(6);
  pub const String: AtomicType = AtomicType(7);
  pub const Boolean: AtomicType = AtomicType(8);
  pub const Bytes: AtomicType = AtomicType(9);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Byte",
      2 => "Int16",
      3 => "Int32",
      4 => "Int64",
      5 => "Float",
      6 => "Double",
      7 => "String",
      8 => "Boolean",
      9 => "Bytes",
      _ => return None
    })
  }
}

impl ::std::convert::From<AtomicType> for i32 {
  fn from(val: AtomicType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for AtomicType {
  fn from(val: i32) -> AtomicType {
    Self(val)
  }
}

impl ::std::default::Default for AtomicType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for AtomicType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "AtomicType::{}", constant_name)
    } else {
      write!(f, "AtomicType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for AtomicType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for AtomicType {}

impl ::protobuf::Proxied for AtomicType {
  type View<'a> = AtomicType;
}

impl ::protobuf::AsView for AtomicType {
  type Proxied = AtomicType;

  fn as_view(&self) -> AtomicType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AtomicType {
  fn into_view<'shorter>(self) -> AtomicType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for AtomicType {
  const NAME: &'static str = "AtomicType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AtomicType {
    type Tag = ::protobuf::__internal::runtime::EnumTag;
}


