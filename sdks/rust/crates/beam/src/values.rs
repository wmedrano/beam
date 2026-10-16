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

use crate::{Result, error};
use beam_coders::{Coder, Value, ValueRef, ValueView};

/// A stable Beam coder contract. Protocol and RPC message types are not
/// implicit element coders; applications must choose an interoperable encoding.
/// Borrowed views must agree with the declared coder and owned conversions.
pub trait BeamValue: ValueView + Send + Sync + 'static {
    #[doc(hidden)]
    fn coder() -> Coder;
    #[doc(hidden)]
    fn into_value(self) -> Value;
    #[doc(hidden)]
    fn from_ref(value: ValueRef<'_>) -> Result<Self>
    where
        Self: Sized;
    #[doc(hidden)]
    fn from_value(value: &Value) -> Result<Self>
    where
        Self: Sized,
    {
        Self::from_ref(value.value_ref())
    }
}

/// A borrowed representation compatible with the declared output type.
/// Exact element types, `str` for `String`, and `[u8]` for `Bytes` are supported.
/// Custom implementations must preserve the output type's coder contract.
pub trait EmitValue<T: BeamValue>: ValueView {}
impl<T: BeamValue> EmitValue<T> for T {}
impl EmitValue<String> for str {}
impl EmitValue<Bytes> for [u8] {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bytes(pub Vec<u8>);
impl ValueView for Bytes {
    fn value_ref(&self) -> ValueRef<'_> {
        ValueRef::Bytes(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kv<K, V> {
    pub key: K,
    pub value: V,
}
impl<K, V> Kv<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}
impl<K: ValueView, V: ValueView> ValueView for Kv<K, V> {
    fn value_ref(&self) -> ValueRef<'_> {
        ValueRef::Kv(&self.key, &self.value)
    }
}

macro_rules! primitive {
    ($rust:ty, $coder:ident, $variant:ident) => {
        impl BeamValue for $rust {
            fn coder() -> Coder {
                Coder::$coder
            }
            fn into_value(self) -> Value {
                Value::$variant(self)
            }
            fn from_ref(value: ValueRef<'_>) -> Result<Self> {
                if let ValueRef::$variant(value) = value {
                    Ok(value)
                } else {
                    Err(error("element/coder contract mismatch"))
                }
            }
        }
    };
}
primitive!(i64, VarInt, I64);
primitive!(bool, Bool, Bool);
primitive!(f64, Double, F64);

impl BeamValue for String {
    fn coder() -> Coder {
        Coder::Utf8
    }
    fn into_value(self) -> Value {
        Value::String(self)
    }
    fn from_ref(value: ValueRef<'_>) -> Result<Self> {
        if let ValueRef::String(value) = value {
            Ok(value.to_owned())
        } else {
            Err(error("expected string"))
        }
    }
}

impl BeamValue for Bytes {
    fn coder() -> Coder {
        Coder::Bytes
    }
    fn into_value(self) -> Value {
        Value::Bytes(self.0)
    }
    fn from_ref(value: ValueRef<'_>) -> Result<Self> {
        if let ValueRef::Bytes(value) = value {
            Ok(Self(value.to_vec()))
        } else {
            Err(error("expected bytes"))
        }
    }
}

impl<K: BeamValue, V: BeamValue> BeamValue for Kv<K, V> {
    fn coder() -> Coder {
        Coder::Kv(Box::new(K::coder()), Box::new(V::coder()))
    }
    fn into_value(self) -> Value {
        Value::Kv(
            Box::new(self.key.into_value()),
            Box::new(self.value.into_value()),
        )
    }
    fn from_ref(value: ValueRef<'_>) -> Result<Self> {
        if let ValueRef::Kv(k, v) = value {
            Ok(Self::new(
                K::from_ref(k.value_ref())?,
                V::from_ref(v.value_ref())?,
            ))
        } else {
            Err(error("expected key/value"))
        }
    }
}

impl<T: BeamValue> BeamValue for Vec<T> {
    fn coder() -> Coder {
        Coder::Iterable(Box::new(T::coder()))
    }
    fn into_value(self) -> Value {
        Value::Iterable(self.into_iter().map(T::into_value).collect())
    }
    fn from_ref(value: ValueRef<'_>) -> Result<Self> {
        let mut values = Vec::new();
        value.try_for_each::<crate::Error>(|item| {
            values.push(T::from_ref(item)?);
            Ok(())
        })?;
        Ok(values)
    }
}

/// Reiterable grouped values. Each item is decoded on demand and can fail.
/// Partial consumption never affects another group or another iterator.
pub struct Grouped<T> {
    value: Value,
    marker: std::marker::PhantomData<T>,
}
impl<T> Clone for Grouped<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: std::marker::PhantomData,
        }
    }
}
impl<T> ValueView for Grouped<T> {
    fn value_ref(&self) -> ValueRef<'_> {
        self.value.value_ref()
    }
}
impl<T: BeamValue> Grouped<T> {
    pub fn iter(&self) -> Result<impl Iterator<Item = Result<T>> + '_> {
        Ok(self.value.iterable()?.map(|item| T::from_value(&item?)))
    }
}
impl<T: BeamValue> BeamValue for Grouped<T> {
    fn coder() -> Coder {
        Coder::Iterable(Box::new(T::coder()))
    }
    fn into_value(self) -> Value {
        self.value
    }
    fn from_ref(value: ValueRef<'_>) -> Result<Self> {
        value.iterable_len()?;
        Ok(Self {
            value: value.to_owned()?,
            marker: std::marker::PhantomData,
        })
    }
}

impl<T: BeamValue> BeamValue for Option<T> {
    fn coder() -> Coder {
        Coder::Nullable(Box::new(T::coder()))
    }
    fn into_value(self) -> Value {
        Value::Nullable(self.map(|value| Box::new(value.into_value())))
    }
    fn from_ref(value: ValueRef<'_>) -> Result<Self> {
        match value {
            ValueRef::Nullable(None) => Ok(None),
            ValueRef::Nullable(Some(value)) => Ok(Some(T::from_ref(value.value_ref())?)),
            _ => Err(error("expected nullable")),
        }
    }
}
