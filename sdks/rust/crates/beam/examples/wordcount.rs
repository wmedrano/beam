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

use apache_beam::{Application, DoFn, Kv, ProcessContext, Result, SumI64};

struct SplitWords;
impl DoFn for SplitWords {
    type Input = String;
    type Output = Kv<String, i64>;
    type Config = ();

    fn from_config(_: ()) -> Result<Self> {
        Ok(Self)
    }

    fn process(
        &mut self,
        line: &String,
        context: &mut ProcessContext<'_, Self::Output>,
    ) -> Result<()> {
        for word in line.split_whitespace() {
            context.counter("words", 1)?;
            context.emit(Kv::new(word.to_owned(), 1))?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Application::new("beam-rust-wordcount-v1")
        .register_dofn::<SplitWords>("example:split_words:v1")?
        .run_from_env(|pipeline| {
            let lines = pipeline.create(
                "Lines",
                vec![
                    "red blue red".to_owned(),
                    "blue green".to_owned(),
                    "red".to_owned(),
                ],
            )?;
            let words = pipeline.par_do::<SplitWords>("Split words", &lines, ())?;
            let counts = pipeline.combine_per_key("Count words", &words, SumI64)?;
            pipeline.assert_contents(
                "Expected counts",
                &counts,
                vec![
                    Kv::new("red".to_owned(), 3),
                    Kv::new("blue".to_owned(), 2),
                    Kv::new("green".to_owned(), 1),
                ],
            )
        })
}
