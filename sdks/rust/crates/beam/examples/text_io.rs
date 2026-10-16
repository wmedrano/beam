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

use apache_beam::{Application, Result};

fn argument(name: &str) -> Result<String> {
    std::env::args()
        .find_map(|arg| arg.strip_prefix(&format!("--{name}=")).map(str::to_owned))
        .ok_or_else(|| std::io::Error::other(format!("requires --{name}=path")).into())
}

fn main() -> Result<()> {
    Application::new("beam-rust-text-io-v1").run_from_env(|pipeline| {
        let lines = pipeline.read_text("Read UTF-8 lines", vec![argument("input")?])?;
        pipeline.write_text("Write UTF-8 shards", &lines, &argument("output")?)?;
        Ok(())
    })
}
