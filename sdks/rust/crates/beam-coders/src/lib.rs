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

//! Internal, bounded Beam wire codecs. These are element encodings, independent
//! of protobuf RPC messages. Limits are checked before allocating from lengths.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Context {
    Outer,
    Nested,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Coder {
    Bytes,
    Utf8,
    Bool,
    VarInt,
    Double,
    Kv(Box<Coder>, Box<Coder>),
    Iterable(Box<Coder>),
    Nullable(Box<Coder>),
    LengthPrefix(Box<Coder>),
    GlobalWindow,
    IntervalWindow,
    Windowed(Box<Coder>, Box<Coder>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bytes(Vec<u8>),
    String(String),
    Bool(bool),
    I64(i64),
    F64(f64),
    Kv(Box<Value>, Box<Value>),
    Iterable(Vec<Value>),
    Nullable(Option<Box<Value>>),
    GlobalWindow,
    IntervalWindow { end: i64, span: i64 },
    Windowed(Box<WindowedValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WindowedValue {
    pub value: Value,
    pub timestamp: i64,
    pub windows: Vec<Value>,
    pub pane: Pane,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timing {
    Early,
    OnTime,
    Late,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pane {
    pub is_first: bool,
    pub is_last: bool,
    pub timing: Timing,
    pub index: i64,
    pub on_time_index: i64,
}

impl Default for Pane {
    fn default() -> Self {
        Self {
            is_first: true,
            is_last: true,
            timing: Timing::Unknown,
            index: 0,
            on_time_index: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub bytes: usize,
    pub items: usize,
    pub depth: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            bytes: 64 * 1024 * 1024,
            items: 1_000_000,
            depth: 64,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NeedMore,
    Invalid(&'static str),
    Limit(&'static str),
    TypeMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NeedMore => write!(f, "truncated Beam element"),
            Self::Invalid(reason) => write!(f, "invalid Beam encoding: {reason}"),
            Self::Limit(reason) => write!(f, "Beam coder resource limit: {reason}"),
            Self::TypeMismatch => write!(f, "value does not match its Beam coder"),
        }
    }
}
impl std::error::Error for Error {}

struct Input<'a> {
    bytes: &'a [u8],
    offset: usize,
    limits: Limits,
    items: usize,
}

impl<'a> Input<'a> {
    fn take(&mut self, length: usize) -> Result<&'a [u8], Error> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(Error::Invalid("length overflow"))?;
        if end > self.limits.bytes {
            return Err(Error::Limit("element bytes"));
        }
        let bytes = self.bytes.get(self.offset..end).ok_or(Error::NeedMore)?;
        self.offset = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }
    fn uvarint(&mut self) -> Result<u64, Error> {
        let mut value = 0;
        for shift in (0..70).step_by(7) {
            let byte = self.byte()?;
            if shift == 63 && byte > 1 {
                return Err(Error::Invalid("varint exceeds 64 bits"));
            }
            value |= u64::from(byte & 127) << shift;
            if byte & 128 == 0 {
                return Ok(value);
            }
        }
        Err(Error::Invalid("varint exceeds 10 bytes"))
    }
    fn length(&mut self) -> Result<usize, Error> {
        let value = self.uvarint()?;
        let value = usize::try_from(value).map_err(|_| Error::Invalid("length exceeds usize"))?;
        if value > self.limits.bytes {
            return Err(Error::Limit("length"));
        }
        Ok(value)
    }
    fn count(&mut self, count: usize) -> Result<(), Error> {
        self.items = self
            .items
            .checked_add(count)
            .ok_or(Error::Limit("item count overflow"))?;
        if self.items > self.limits.items {
            return Err(Error::Limit("collection items"));
        }
        Ok(())
    }
    fn timestamp(&mut self) -> Result<i64, Error> {
        Ok((u64::from_be_bytes(self.take(8)?.try_into().unwrap()) ^ (1 << 63)) as i64)
    }
}

fn varint(mut value: u64, out: &mut Vec<u8>) {
    while value >= 128 {
        out.push(value as u8 | 128);
        value >>= 7;
    }
    out.push(value as u8);
}

fn timestamp(value: i64, out: &mut Vec<u8>) {
    out.extend_from_slice(&((value as u64) ^ (1 << 63)).to_be_bytes());
}

impl Coder {
    pub fn signature(&self) -> String {
        let (name, children): (&str, Vec<&Coder>) = match self {
            Self::Bytes => ("bytes", vec![]),
            Self::Utf8 => ("string_utf8", vec![]),
            Self::Bool => ("bool", vec![]),
            Self::VarInt => ("varint", vec![]),
            Self::Double => ("double", vec![]),
            Self::Kv(k, v) => ("kv", vec![k, v]),
            Self::Windowed(v, w) => ("windowed_value", vec![v, w]),
            Self::Iterable(c) => ("iterable", vec![c]),
            Self::Nullable(c) => ("nullable", vec![c]),
            Self::LengthPrefix(c) => ("length_prefix", vec![c]),
            Self::GlobalWindow => ("global_window", vec![]),
            Self::IntervalWindow => ("interval_window", vec![]),
        };
        let mut signature = format!("beam:coder:{name}:v1");
        for child in children {
            let child = child.signature();
            signature.push_str(&format!(";{}:{child}", child.len()));
        }
        signature
    }
    /// Canonical encoding is a separate property from encodability. Arbitrary
    /// protobufs, floats and windowed values cannot silently become key coders.
    pub fn deterministic_key(&self) -> bool {
        match self {
            Self::Bytes | Self::Utf8 | Self::Bool | Self::VarInt => true,
            Self::Kv(k, v) => k.deterministic_key() && v.deterministic_key(),
            Self::Nullable(c) | Self::LengthPrefix(c) => c.deterministic_key(),
            _ => false,
        }
    }

    pub fn encode(&self, value: &Value, context: Context) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        self.write(value, context, &mut bytes, 64)?;
        Ok(bytes)
    }

    pub fn decode(&self, bytes: &[u8], context: Context, limits: Limits) -> Result<Value, Error> {
        let (value, consumed) = self.decode_prefix(bytes, context, limits)?;
        if consumed != bytes.len() {
            return Err(Error::Invalid("trailing bytes"));
        }
        Ok(value)
    }

    /// A truncated prefix returns NeedMore. Callers retain the bytes and retry
    /// with the next data frame; no decoder state or allocations are retained.
    pub fn decode_prefix(
        &self,
        bytes: &[u8],
        context: Context,
        limits: Limits,
    ) -> Result<(Value, usize), Error> {
        let mut input = Input {
            bytes,
            offset: 0,
            limits,
            items: 0,
        };
        let value = self.read(&mut input, context, limits.depth)?;
        Ok((value, input.offset))
    }

    fn write(
        &self,
        value: &Value,
        context: Context,
        out: &mut Vec<u8>,
        depth: usize,
    ) -> Result<(), Error> {
        if depth == 0 {
            return Err(Error::Limit("coder depth"));
        }
        match (self, value) {
            (Self::Bytes, Value::Bytes(bytes)) => {
                if context == Context::Nested {
                    varint(bytes.len() as u64, out);
                }
                out.extend_from_slice(bytes);
            }
            (Self::Utf8, Value::String(string)) => {
                if context == Context::Nested {
                    varint(string.len() as u64, out);
                }
                out.extend_from_slice(string.as_bytes());
            }
            (Self::Bool, Value::Bool(boolean)) => out.push(u8::from(*boolean)),
            (Self::VarInt, Value::I64(integer)) => varint(*integer as u64, out),
            (Self::Double, Value::F64(number)) => out.extend_from_slice(&number.to_be_bytes()),
            (Self::Kv(key, val), Value::Kv(k, v)) => {
                key.write(k, Context::Nested, out, depth - 1)?;
                val.write(v, context, out, depth - 1)?;
            }
            (Self::Iterable(coder), Value::Iterable(items)) => {
                let length =
                    i32::try_from(items.len()).map_err(|_| Error::Limit("iterable length"))?;
                out.extend_from_slice(&length.to_be_bytes());
                for item in items {
                    coder.write(item, Context::Nested, out, depth - 1)?;
                }
            }
            (Self::Nullable(_), Value::Nullable(None)) => out.push(0),
            (Self::Nullable(coder), Value::Nullable(Some(value))) => {
                out.push(1);
                coder.write(value, context, out, depth - 1)?;
            }
            (Self::LengthPrefix(coder), value) => {
                let mut encoded = Vec::new();
                coder.write(value, Context::Outer, &mut encoded, depth - 1)?;
                varint(encoded.len() as u64, out);
                out.extend(encoded);
            }
            (Self::GlobalWindow, Value::GlobalWindow) => {}
            (Self::IntervalWindow, Value::IntervalWindow { end, span }) => {
                if *span < 0 {
                    return Err(Error::Invalid("negative window span"));
                }
                end.checked_sub(*span)
                    .ok_or(Error::Invalid("window start overflow"))?;
                timestamp(*end, out);
                varint(*span as u64, out);
            }
            (Self::Windowed(coder, window), Value::Windowed(value)) => {
                timestamp(value.timestamp, out);
                let length =
                    i32::try_from(value.windows.len()).map_err(|_| Error::Limit("window count"))?;
                out.extend_from_slice(&length.to_be_bytes());
                for win in &value.windows {
                    window.write(win, Context::Nested, out, depth - 1)?;
                }
                value.pane.write(out)?;
                coder.write(&value.value, context, out, depth - 1)?;
            }
            _ => return Err(Error::TypeMismatch),
        }
        Ok(())
    }

    fn read(&self, input: &mut Input<'_>, context: Context, depth: usize) -> Result<Value, Error> {
        if depth == 0 {
            return Err(Error::Limit("coder depth"));
        }
        match self {
            Self::Bytes | Self::Utf8 => {
                let length = if context == Context::Nested {
                    input.length()?
                } else {
                    input.bytes.len() - input.offset
                };
                let bytes = input.take(length)?;
                if *self == Self::Bytes {
                    Ok(Value::Bytes(bytes.to_vec()))
                } else {
                    Ok(Value::String(
                        std::str::from_utf8(bytes)
                            .map_err(|_| Error::Invalid("UTF-8"))?
                            .to_owned(),
                    ))
                }
            }
            Self::Bool => match input.byte()? {
                0 => Ok(Value::Bool(false)),
                1 => Ok(Value::Bool(true)),
                _ => Err(Error::Invalid("boolean")),
            },
            Self::VarInt => Ok(Value::I64(input.uvarint()? as i64)),
            Self::Double => Ok(Value::F64(f64::from_be_bytes(
                input.take(8)?.try_into().unwrap(),
            ))),
            Self::Kv(k, v) => Ok(Value::Kv(
                Box::new(k.read(input, Context::Nested, depth - 1)?),
                Box::new(v.read(input, context, depth - 1)?),
            )),
            Self::Nullable(coder) => match input.byte()? {
                0 => Ok(Value::Nullable(None)),
                1 => Ok(Value::Nullable(Some(Box::new(coder.read(
                    input,
                    context,
                    depth - 1,
                )?)))),
                _ => Err(Error::Invalid("nullable presence")),
            },
            Self::LengthPrefix(coder) => {
                let length = input.length()?;
                let bytes = input.take(length)?;
                let mut nested = Input {
                    bytes,
                    offset: 0,
                    limits: input.limits,
                    items: input.items,
                };
                let result = coder.read(&mut nested, Context::Outer, depth - 1)?;
                input.items = nested.items;
                if nested.offset != length {
                    return Err(Error::Invalid("length-prefix trailing bytes"));
                }
                Ok(result)
            }
            Self::Iterable(coder) => {
                let length = i32::from_be_bytes(input.take(4)?.try_into().unwrap());
                if length < -1 {
                    return Err(Error::Invalid("negative iterable length"));
                }
                let mut items = Vec::new();
                if length >= 0 {
                    input.count(length as usize)?;
                    for _ in 0..length {
                        items.push(coder.read(input, Context::Nested, depth - 1)?);
                    }
                } else {
                    loop {
                        let count = input.uvarint()? as i64;
                        if count == 0 {
                            break;
                        }
                        if count < 0 {
                            return Err(Error::Invalid("state-backed iterables are unsupported"));
                        }
                        let count =
                            usize::try_from(count).map_err(|_| Error::Limit("block count"))?;
                        input.count(count)?;
                        for _ in 0..count {
                            items.push(coder.read(input, Context::Nested, depth - 1)?);
                        }
                    }
                }
                Ok(Value::Iterable(items))
            }
            Self::GlobalWindow => Ok(Value::GlobalWindow),
            Self::IntervalWindow => {
                let end = input.timestamp()?;
                let span = i64::try_from(input.uvarint()?)
                    .map_err(|_| Error::Invalid("window span exceeds i64"))?;
                end.checked_sub(span)
                    .ok_or(Error::Invalid("window start overflow"))?;
                Ok(Value::IntervalWindow { end, span })
            }
            Self::Windowed(coder, window) => {
                let timestamp = input.timestamp()?;
                let count = i32::from_be_bytes(input.take(4)?.try_into().unwrap());
                if count < 0 {
                    return Err(Error::Invalid("negative window count"));
                }
                input.count(count as usize)?;
                let mut windows = Vec::new();
                for _ in 0..count {
                    windows.push(window.read(input, Context::Nested, depth - 1)?);
                }
                let pane = Pane::read(input)?;
                let value = coder.read(input, context, depth - 1)?;
                Ok(Value::Windowed(Box::new(WindowedValue {
                    value,
                    timestamp,
                    windows,
                    pane,
                })))
            }
        }
    }
}

impl Pane {
    fn write(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        if self.index < 0
            || (self.timing == Timing::Early && self.on_time_index != -1)
            || (self.timing != Timing::Early && self.on_time_index < 0)
        {
            return Err(Error::Invalid("pane indices"));
        }
        let timing = match self.timing {
            Timing::Early => 0,
            Timing::OnTime => 1,
            Timing::Late => 2,
            Timing::Unknown => 3,
        };
        let byte = u8::from(self.is_first) | (u8::from(self.is_last) << 1) | timing << 2;
        if self.index == 0 && self.on_time_index == 0 {
            out.push(byte);
        } else if self.index == self.on_time_index || self.timing == Timing::Early {
            out.push(byte | 16);
            varint(self.index as u64, out);
        } else {
            out.push(byte | 32);
            varint(self.index as u64, out);
            varint(self.on_time_index as u64, out);
        }
        Ok(())
    }

    fn read(input: &mut Input<'_>) -> Result<Self, Error> {
        let byte = input.byte()?;
        let timing = match byte >> 2 & 3 {
            0 => Timing::Early,
            1 => Timing::OnTime,
            2 => Timing::Late,
            _ => Timing::Unknown,
        };
        let (index, on_time_index) = match byte >> 4 {
            0 => (0, if timing == Timing::Early { -1 } else { 0 }),
            1 => {
                let i = input.uvarint()? as i64;
                (i, if timing == Timing::Early { -1 } else { i })
            }
            2 => (input.uvarint()? as i64, input.uvarint()? as i64),
            _ => return Err(Error::Invalid("pane encoding tag")),
        };
        if index < 0
            || (timing != Timing::Early && on_time_index < 0)
            || (timing == Timing::Early && on_time_index != -1)
        {
            return Err(Error::Invalid("pane indices"));
        }
        Ok(Self {
            is_first: byte & 1 != 0,
            is_last: byte & 2 != 0,
            timing,
            index,
            on_time_index,
        })
    }
}
