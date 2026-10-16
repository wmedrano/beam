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

use crate::{Coder, Context, Error, Limits, Value};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Clone, Debug)]
pub struct SpoolBudget(Arc<Budget>);
#[derive(Debug)]
struct Budget {
    maximum: usize,
    used: AtomicUsize,
}
impl SpoolBudget {
    pub fn new(maximum: usize) -> Self {
        Self(Arc::new(Budget {
            maximum,
            used: AtomicUsize::new(0),
        }))
    }
    pub fn used(&self) -> usize {
        self.0.used.load(Ordering::Relaxed)
    }
    pub(crate) fn maximum(&self) -> usize {
        self.0.maximum
    }
    fn reserve(&self, bytes: usize) -> Result<(), Error> {
        self.0
            .used
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |used| {
                used.checked_add(bytes).filter(|sum| *sum <= self.0.maximum)
            })
            .map(|_| ())
            .map_err(|_| Error::Limit("group spool disk budget"))
    }
    fn release(&self, bytes: usize) {
        self.0.used.fetch_sub(bytes, Ordering::Relaxed);
    }
}

struct Storage {
    file: Mutex<File>,
    bytes: usize,
    budget: SpoolBudget,
}
impl Drop for Storage {
    fn drop(&mut self) {
        self.budget.release(self.bytes);
    }
}

/// Immutable, shared storage. Each iterator has its own cursor, so fan-out and
/// partially consumed iterators never change another consumer's position.
#[derive(Clone)]
pub struct Iterable {
    storage: Arc<Storage>,
    coder: Coder,
    count: usize,
    limits: Limits,
}
impl std::fmt::Debug for Iterable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Iterable")
            .field("count", &self.count)
            .field("bytes", &self.storage.bytes)
            .finish()
    }
}
impl PartialEq for Iterable {
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.storage, &other.storage) {
            return true;
        }
        self.count == other.count
            && self
                .iter()
                .unwrap()
                .zip(other.iter().unwrap())
                .all(|(a, b)| matches!((a, b), (Ok(a), Ok(b)) if a == b))
    }
}
impl Iterable {
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn iter(&self) -> Result<Values<'_>, Error> {
        Ok(Values::Spool(SpoolValues {
            reader: BufReader::new(Cursor {
                storage: self.storage.clone(),
                offset: 0,
            }),
            remaining: self.count,
            coder: self.coder.clone(),
            limits: self.limits,
        }))
    }
}

struct Cursor {
    storage: Arc<Storage>,
    offset: u64,
}
impl Read for Cursor {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let mut file = self
            .storage
            .file
            .lock()
            .map_err(|_| std::io::Error::other("poisoned group spool"))?;
        file.seek(SeekFrom::Start(self.offset))?;
        let count = file.read(buffer)?;
        self.offset += count as u64;
        Ok(count)
    }
}

pub struct SpoolValues {
    reader: BufReader<Cursor>,
    remaining: usize,
    coder: Coder,
    limits: Limits,
}
pub enum Values<'a> {
    Memory(std::slice::Iter<'a, Value>),
    Spool(SpoolValues),
}
impl Iterator for Values<'_> {
    type Item = Result<Value, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Memory(values) => values.next().cloned().map(Ok),
            Self::Spool(values) => {
                if values.remaining == 0 {
                    return None;
                }
                values.remaining -= 1;
                let result = (|| {
                    let mut length = [0; 8];
                    values.reader.read_exact(&mut length).map_err(io)?;
                    let length = usize::try_from(u64::from_be_bytes(length))
                        .map_err(|_| Error::Limit("group item length"))?;
                    if length > values.limits.bytes {
                        return Err(Error::Limit("group item bytes"));
                    }
                    let mut bytes = vec![0; length];
                    values.reader.read_exact(&mut bytes).map_err(io)?;
                    values.coder.decode(&bytes, Context::Nested, values.limits)
                })();
                if result.is_err() {
                    values.remaining = 0;
                }
                Some(result)
            }
        }
    }
}
impl Value {
    pub fn iterable(&self) -> Result<Values<'_>, Error> {
        match self {
            Self::Iterable(values) => Ok(Values::Memory(values.iter())),
            Self::Spool(values) => values.iter(),
            _ => Err(Error::TypeMismatch),
        }
    }
}

fn io(error: std::io::Error) -> Error {
    Error::Io(error.to_string())
}

pub(crate) struct Builder {
    file: Option<BufWriter<File>>,
    bytes: usize,
    budget: SpoolBudget,
    coder: Coder,
    count: usize,
    limits: Limits,
}
impl Drop for Builder {
    fn drop(&mut self) {
        self.budget.release(self.bytes);
    }
}
impl Builder {
    pub fn new(file: File, coder: Coder, budget: SpoolBudget, limits: Limits) -> Self {
        Self {
            file: Some(BufWriter::new(file)),
            bytes: 0,
            budget,
            coder,
            count: 0,
            limits,
        }
    }
    pub fn push(&mut self, value: &Value) -> Result<(), Error> {
        let bytes = self.coder.encode(value, Context::Nested)?;
        if bytes.len() > self.limits.bytes {
            return Err(Error::Limit("group item bytes"));
        }
        let charged = bytes
            .len()
            .checked_add(8)
            .ok_or(Error::Limit("group size"))?;
        self.budget.reserve(charged)?;
        self.bytes += charged;
        let file = self.file.as_mut().unwrap();
        file.write_all(&(bytes.len() as u64).to_be_bytes())
            .map_err(io)?;
        file.write_all(&bytes).map_err(io)?;
        self.count += 1;
        Ok(())
    }
    pub fn finish(mut self) -> Result<Iterable, Error> {
        let file = self
            .file
            .take()
            .unwrap()
            .into_inner()
            .map_err(|e| io(e.into_error()))?;
        let storage = Arc::new(Storage {
            file: Mutex::new(file),
            bytes: self.bytes,
            budget: self.budget.clone(),
        });
        self.bytes = 0;
        Ok(Iterable {
            storage,
            coder: self.coder.clone(),
            count: self.count,
            limits: self.limits,
        })
    }
}
