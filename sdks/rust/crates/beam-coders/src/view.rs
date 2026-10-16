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

//! Synchronous borrowed element views. No view retains its source's storage.

use crate::{Error, Iterable, Pane, Value, WindowedValue};
use std::ops::ControlFlow;

/// An element representation that can be inspected without taking ownership.
pub trait ValueView {
    fn value_ref(&self) -> ValueRef<'_>;
}

impl ValueView for str {
    fn value_ref(&self) -> ValueRef<'_> {
        ValueRef::String(self)
    }
}
impl ValueView for String {
    fn value_ref(&self) -> ValueRef<'_> {
        self.as_str().value_ref()
    }
}
impl ValueView for [u8] {
    fn value_ref(&self) -> ValueRef<'_> {
        ValueRef::Bytes(self)
    }
}
macro_rules! scalar {
    ($ty:ty, $variant:ident) => {
        impl ValueView for $ty {
            fn value_ref(&self) -> ValueRef<'_> {
                ValueRef::$variant(*self)
            }
        }
    };
}
scalar!(bool, Bool);
scalar!(i64, I64);
scalar!(f64, F64);

impl<T: ValueView> ValueView for Vec<T> {
    fn value_ref(&self) -> ValueRef<'_> {
        ValueRef::Iterable(self)
    }
}
impl<T: ValueView> ValueView for Option<T> {
    fn value_ref(&self) -> ValueRef<'_> {
        ValueRef::Nullable(self.as_ref().map(|v| v as &dyn ValueView))
    }
}

/// Reiterable in-memory elements, visited without allocating an owned tree.
/// A visitor can stop early; errors reading the underlying storage propagate.
/// Implementations visit exactly `len()` elements unless the visitor returns
/// `Break`, and finish each callback before producing the next element.
pub trait IterableView {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn visit(&self, visitor: &mut dyn FnMut(ValueRef<'_>) -> ControlFlow<()>) -> Result<(), Error>;
}

impl<T: ValueView> IterableView for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
    fn visit(&self, visitor: &mut dyn FnMut(ValueRef<'_>) -> ControlFlow<()>) -> Result<(), Error> {
        for item in self {
            if visitor(item.value_ref()).is_break() {
                break;
            }
        }
        Ok(())
    }
}

/// An element view whose composite children and payloads are borrowed;
/// disk-backed groups retain their existing independent-iterator behavior.
#[derive(Clone, Copy)]
pub enum ValueRef<'a> {
    Bytes(&'a [u8]),
    String(&'a str),
    Bool(bool),
    I64(i64),
    F64(f64),
    Kv(&'a dyn ValueView, &'a dyn ValueView),
    Iterable(&'a dyn IterableView),
    Spool(&'a Iterable),
    Nullable(Option<&'a dyn ValueView>),
    GlobalWindow,
    IntervalWindow {
        end: i64,
        span: i64,
    },
    Windowed {
        value: &'a dyn ValueView,
        timestamp: i64,
        windows: &'a [Value],
        pane: Pane,
    },
}

impl ValueView for ValueRef<'_> {
    fn value_ref(&self) -> ValueRef<'_> {
        *self
    }
}

impl ValueView for Value {
    fn value_ref(&self) -> ValueRef<'_> {
        match self {
            Self::Bytes(v) => ValueRef::Bytes(v),
            Self::String(v) => ValueRef::String(v),
            Self::Bool(v) => ValueRef::Bool(*v),
            Self::I64(v) => ValueRef::I64(*v),
            Self::F64(v) => ValueRef::F64(*v),
            Self::Kv(k, v) => ValueRef::Kv(k.as_ref(), v.as_ref()),
            Self::Iterable(v) => ValueRef::Iterable(v),
            Self::Spool(v) => ValueRef::Spool(v),
            Self::Nullable(v) => ValueRef::Nullable(v.as_deref().map(|v| v as &dyn ValueView)),
            Self::GlobalWindow => ValueRef::GlobalWindow,
            Self::IntervalWindow { end, span } => ValueRef::IntervalWindow {
                end: *end,
                span: *span,
            },
            Self::Windowed(v) => v.value_ref(),
        }
    }
}

impl ValueView for WindowedValue {
    fn value_ref(&self) -> ValueRef<'_> {
        ValueRef::Windowed {
            value: &self.value,
            timestamp: self.timestamp,
            windows: &self.windows,
            pane: self.pane,
        }
    }
}

impl ValueRef<'_> {
    pub fn iterable_len(self) -> Result<usize, Error> {
        match self {
            Self::Iterable(v) => Ok(v.len()),
            Self::Spool(v) => Ok(v.len()),
            _ => Err(Error::TypeMismatch),
        }
    }

    /// Visit borrowed items, preserving either codec or caller-specific errors.
    /// Spool items are decoded one at a time and live only during the callback.
    pub fn try_for_each<E: From<Error>>(
        self,
        mut visitor: impl FnMut(ValueRef<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        match self {
            Self::Iterable(items) => {
                let mut result = Ok(());
                items.visit(&mut |item| match visitor(item) {
                    Ok(()) => ControlFlow::Continue(()),
                    Err(error) => {
                        result = Err(error);
                        ControlFlow::Break(())
                    }
                })?;
                result
            }
            Self::Spool(items) => {
                for item in items.iter()? {
                    visitor(item?.value_ref())?;
                }
                Ok(())
            }
            _ => Err(Error::TypeMismatch.into()),
        }
    }

    /// Materialize only where ownership is required, retaining shared spools.
    pub fn to_owned(self) -> Result<Value, Error> {
        Ok(match self {
            Self::Bytes(v) => Value::Bytes(v.to_vec()),
            Self::String(v) => Value::String(v.to_owned()),
            Self::Bool(v) => Value::Bool(v),
            Self::I64(v) => Value::I64(v),
            Self::F64(v) => Value::F64(v),
            Self::Kv(k, v) => Value::Kv(
                Box::new(k.value_ref().to_owned()?),
                Box::new(v.value_ref().to_owned()?),
            ),
            Self::Iterable(_) => {
                let mut items = Vec::new();
                self.try_for_each::<Error>(|v| {
                    items.push(v.to_owned()?);
                    Ok(())
                })?;
                Value::Iterable(items)
            }
            Self::Spool(v) => Value::Spool(v.clone()),
            Self::Nullable(v) => Value::Nullable(
                v.map(|v| v.value_ref().to_owned().map(Box::new))
                    .transpose()?,
            ),
            Self::GlobalWindow => Value::GlobalWindow,
            Self::IntervalWindow { end, span } => Value::IntervalWindow { end, span },
            Self::Windowed {
                value,
                timestamp,
                windows,
                pane,
            } => Value::Windowed(Box::new(WindowedValue {
                value: value.value_ref().to_owned()?,
                timestamp,
                windows: windows.to_vec(),
                pane,
            })),
        })
    }
}
