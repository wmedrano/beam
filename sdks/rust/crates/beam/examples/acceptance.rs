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

use apache_beam::{
    Application, BeamConfig, Bytes, DoFn, Grouped, Kv, ProcessContext, Result, SumI64,
};

struct Mode(String);
impl BeamConfig for Mode {
    const CONTRACT: &'static str = "example:lifecycle_config:v1";
    const VERSION: u32 = 1;
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(if self.0 == "config" {
            vec![255]
        } else {
            self.0.as_bytes().to_vec()
        })
    }
    fn decode(bytes: &[u8]) -> Result<Self> {
        Ok(Self(std::str::from_utf8(bytes)?.to_owned()))
    }
}
struct Lifecycle {
    mode: String,
    phase: u8,
}
impl DoFn for Lifecycle {
    type Input = i64;
    type Output = i64;
    type Config = Mode;
    fn from_config(mode: Mode) -> Result<Self> {
        assert!(mode.0 != "factory_panic", "requested factory panic");
        Ok(Self {
            mode: mode.0,
            phase: 0,
        })
    }
    fn setup(&mut self) -> Result<()> {
        assert!(self.mode != "setup_panic", "requested setup panic");
        if self.mode == "setup_fail" {
            return Err(std::io::Error::other("requested setup failure").into());
        }
        assert_eq!(self.phase, 0);
        self.phase = 1;
        Ok(())
    }
    fn start_bundle(&mut self) -> Result<()> {
        assert_eq!(self.phase, 1);
        self.phase = 2;
        Ok(())
    }
    fn process(&mut self, value: &i64, context: &mut ProcessContext<'_, i64>) -> Result<()> {
        assert_eq!(self.phase, 2);
        if self.mode == "slow" {
            let until = std::time::Instant::now() + std::time::Duration::from_secs(2);
            let mut value = 1_u64;
            while std::time::Instant::now() < until {
                value =
                    std::hint::black_box(value.wrapping_mul(6364136223846793005).wrapping_add(1));
            }
        }
        context.emit(*value)
    }
    fn finish_bundle(&mut self, _context: &mut apache_beam::FinishContext<'_, i64>) -> Result<()> {
        assert_eq!(self.phase, 2);
        self.phase = 3;
        if self.mode == "finish_fail" {
            return Err(std::io::Error::other("requested finish failure").into());
        }
        Ok(())
    }
}

struct GenerateBytes;
impl DoFn for GenerateBytes {
    type Input = i64;
    type Output = Kv<i64, Bytes>;
    type Config = ();
    fn from_config(_: ()) -> Result<Self> {
        Ok(Self)
    }
    fn process(
        &mut self,
        length: &i64,
        context: &mut ProcessContext<'_, Self::Output>,
    ) -> Result<()> {
        context.emit(Kv::new(0, Bytes(vec![42; usize::try_from(*length)?])))
    }
}
struct ByteLengths;
impl DoFn for ByteLengths {
    type Input = Kv<i64, Grouped<Bytes>>;
    type Output = i64;
    type Config = ();
    fn from_config(_: ()) -> Result<Self> {
        Ok(Self)
    }
    fn process(
        &mut self,
        value: &Self::Input,
        context: &mut ProcessContext<'_, i64>,
    ) -> Result<()> {
        for bytes in value.value.iter()? {
            let bytes = bytes?;
            assert!(bytes.0.iter().all(|byte| *byte == 42));
            context.emit(i64::try_from(bytes.0.len())?)?;
        }
        Ok(())
    }
}

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
        if line == "__fail__" {
            return Err(std::io::Error::other("requested user failure").into());
        }
        assert_ne!(line, "__panic__", "requested user panic");
        for word in line.split_whitespace() {
            context.counter("words", 1)?;
            context.emit(Kv::new(word.to_owned(), 1))?;
        }
        Ok(())
    }
}

struct BufferedSum(i64);
impl DoFn for BufferedSum {
    type Input = i64;
    type Output = i64;
    type Config = ();
    fn from_config(_: ()) -> Result<Self> {
        Ok(Self(0))
    }
    fn process(&mut self, value: &i64, _: &mut ProcessContext<'_, i64>) -> Result<()> {
        self.0 = self.0.wrapping_add(*value);
        Ok(())
    }
    fn finish_bundle(&mut self, context: &mut apache_beam::FinishContext<'_, i64>) -> Result<()> {
        context.log("Emitting buffered sum during finish")?;
        context.emit(self.0, 1234)
    }
}

struct FirstInGroup;
impl DoFn for FirstInGroup {
    type Input = Kv<i64, Grouped<i64>>;
    type Output = i64;
    type Config = ();
    fn from_config(_: ()) -> Result<Self> {
        Ok(Self)
    }
    fn process(
        &mut self,
        input: &Self::Input,
        context: &mut ProcessContext<'_, i64>,
    ) -> Result<()> {
        let mut values = input.value.iter()?;
        assert_eq!(values.next().transpose()?, Some(input.key));
        // Deliberately leave the remainder unread.
        context.emit(input.key)
    }
}

fn main() -> Result<()> {
    let case = std::env::args()
        .find_map(|arg| arg.strip_prefix("--case=").map(str::to_owned))
        .unwrap_or_default();
    let identity = if case == "identity" {
        "wrong-worker-build"
    } else {
        "beam-rust-acceptance-v1"
    };
    Application::new(identity)
        .register_dofn::<SplitWords>("example:split_words:v1")?
        .register_dofn::<Lifecycle>("example:lifecycle:v1")?
        .register_dofn::<GenerateBytes>("example:generate_bytes:v1")?
        .register_dofn::<ByteLengths>("example:byte_lengths:v1")?
        .register_dofn::<BufferedSum>("example:buffered_sum:v1")?
        .register_dofn::<FirstInGroup>("example:first_in_group:v1")?
        .run_from_env(|p| {
            let lines = if ["fail", "panic"].contains(&case.as_str()) {
                vec![format!("__{case}__")]
            } else {
                vec![
                    "red blue red".to_owned(),
                    "blue green".to_owned(),
                    "red".to_owned(),
                ]
            };
            let lines = p.create("Lines", lines)?;
            let words = p.par_do::<SplitWords>("Split", &lines, ())?;
            let counts = p.combine_per_key("Count", &words, SumI64)?;
            let mut expected = vec![
                Kv::new("red".to_owned(), 3),
                Kv::new("blue".to_owned(), 2),
                Kv::new("green".to_owned(), 1),
            ];
            if case == "assert" {
                expected.push(Kv::new("missing".to_owned(), 1));
            }
            p.assert_contents("Counts", &counts, expected)?;

            let words_again = p.par_do::<SplitWords>("Split again", &lines, ())?;
            let both = p.flatten("Fanout flattened", &[words, words_again])?;
            let counts = p.combine_per_key("Count twice", &both, SumI64)?;
            p.assert_contents(
                "Doubled counts",
                &counts,
                vec![
                    Kv::new("red".to_owned(), 6),
                    Kv::new("blue".to_owned(), 4),
                    Kv::new("green".to_owned(), 2),
                ],
            )?;

            let empty = p.create::<String>("Empty", vec![])?;
            p.assert_contents(
                "Empty assertion",
                &empty,
                if case == "empty_assert" {
                    vec!["missing".to_owned()]
                } else {
                    vec![]
                },
            )?;
            let repeated = p.create("Repeated values", vec![1_i64, 1, 2])?;
            p.assert_contents(
                "Multiplicity",
                &repeated,
                if case == "multiplicity" {
                    vec![1, 2]
                } else {
                    vec![2, 1, 1]
                },
            )?;
            let overflow = p.create(
                "Wrapping input",
                vec![
                    Kv::new("k".to_owned(), i64::MAX),
                    Kv::new("k".to_owned(), 1),
                    Kv::new("k".to_owned(), i64::MIN),
                ],
            )?;
            let overflow = p.combine_per_key("Wrapping sum", &overflow, SumI64)?;
            p.assert_contents(
                "Wrapping assertion",
                &overflow,
                vec![Kv::new("k".to_owned(), 0)],
            )?;
            let lifecycle_input = p.create("Lifecycle input", vec![17_i64])?;
            let lifecycle =
                p.par_do::<Lifecycle>("Lifecycle", &lifecycle_input, Mode(case.clone()))?;
            p.assert_contents("Lifecycle result", &lifecycle, vec![17])?;
            if case == "large" || case == "hot_group" {
                let count = if case == "hot_group" { 40 } else { 3 };
                let lengths = p.create("Byte lengths", vec![2_i64 * 1024 * 1024; count])?;
                let bytes = p.par_do::<GenerateBytes>("Generate bytes", &lengths, ())?;
                let grouped = p.group_by_key("Shuffle large elements", &bytes)?;
                let lengths = p.par_do::<ByteLengths>("Verify bytes", &grouped, ())?;
                p.assert_contents(
                    "Transferred lengths",
                    &lengths,
                    vec![2_i64 * 1024 * 1024; count],
                )?;
            }
            if case == "partial_group" {
                let input = p.create(
                    "Grouped integers",
                    (0..3)
                        .flat_map(|key| std::iter::repeat_n(Kv::new(key, key), 100))
                        .collect(),
                )?;
                let grouped = p.group_by_key("Group integers", &input)?;
                let first = p.par_do::<FirstInGroup>("Read only one value", &grouped, ())?;
                p.assert_contents("Partial consumption result", &first, vec![0, 1, 2])?;
            }
            if case == "finish_emit" {
                let input = p.create("Buffered integers", vec![1_i64, 2, 3])?;
                let first = p.par_do::<BufferedSum>("Buffer upstream", &input, ())?;
                let second = p.par_do::<BufferedSum>("Buffer downstream", &first, ())?;
                p.assert_contents("Finish result", &second, vec![6])?;
            }
            Ok(())
        })
}
