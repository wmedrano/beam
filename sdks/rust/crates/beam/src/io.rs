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

//! Bounded UTF-8 text I/O for the shared filesystem of local PROCESS workers.
//! Output files are atomic per bundle, with unique names. Runner retries can
//! produce duplicate shards; this prototype makes no exactly-once I/O claim.

use crate::{BeamConfig, DoFn, FinishContext, ProcessContext, Result, error};
use std::io::{BufRead, BufReader, Read, Write};

impl BeamConfig for String {
    const CONTRACT: &'static str = "beam:rust:config:utf8:v1";
    const VERSION: u32 = 1;
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
    fn decode(bytes: &[u8]) -> Result<Self> {
        Ok(std::str::from_utf8(bytes)?.to_owned())
    }
}

pub(crate) struct ReadText;
impl DoFn for ReadText {
    type Input = String;
    type Output = String;
    type Config = ();
    fn from_config(_: ()) -> Result<Self> {
        Ok(Self)
    }
    fn process(&mut self, path: &String, context: &mut ProcessContext<'_, String>) -> Result<()> {
        let mut reader = BufReader::new(std::fs::File::open(path)?);
        let mut line = Vec::new();
        loop {
            line.clear();
            let maximum = beam_coders::Limits::default().bytes;
            let read = (&mut reader)
                .take(maximum as u64 + 1)
                .read_until(b'\n', &mut line)?;
            if read == 0 {
                break;
            }
            if read > maximum {
                return Err(error(format!(
                    "text line exceeds {maximum} bytes in {path}"
                )));
            }
            if line.last() == Some(&b'\n') {
                line.pop();
                if line.last() == Some(&b'\r') {
                    line.pop();
                }
            }
            context.emit(std::str::from_utf8(&line)?)?;
        }
        Ok(())
    }
}

pub(crate) struct WriteText {
    directory: String,
    file: Option<tempfile::NamedTempFile>,
}
impl DoFn for WriteText {
    type Input = String;
    type Output = String;
    type Config = String;
    fn from_config(directory: String) -> Result<Self> {
        Ok(Self {
            directory,
            file: None,
        })
    }
    fn setup(&mut self) -> Result<()> {
        std::fs::create_dir_all(&self.directory)?;
        self.file = Some(tempfile::NamedTempFile::new_in(&self.directory)?);
        Ok(())
    }
    fn process(&mut self, line: &String, _: &mut ProcessContext<'_, String>) -> Result<()> {
        if line.contains(['\r', '\n']) {
            return Err(error("WriteText requires one line per element"));
        }
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| error("text sink is not initialized"))?;
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }
    fn finish_bundle(&mut self, context: &mut FinishContext<'_, String>) -> Result<()> {
        let mut file = self
            .file
            .take()
            .ok_or_else(|| error("text sink already finished"))?;
        file.flush()?;
        let name = format!(
            "part-{}.txt",
            file.path().file_name().unwrap().to_string_lossy()
        );
        let destination = std::path::Path::new(&self.directory).join(name);
        file.persist_noclobber(&destination)
            .map_err(|failure| failure.error)?;
        context.log("Committed local text shard")?;
        context.emit(destination.to_string_lossy().as_ref(), 0)
    }
}
