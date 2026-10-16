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

use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

struct Process(Child);
impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn wait(child: &mut Child, limit: Duration) -> std::process::ExitStatus {
    let deadline = Instant::now() + limit;
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            return status;
        }
        assert!(Instant::now() < deadline, "runner job timed out");
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
#[ignore = "requires checkout Prism and Rust example binaries; run tools/test-prism.sh"]
fn bounded_process_acceptance() {
    let prism = std::env::var("BEAM_RUST_PRISM").expect("set BEAM_RUST_PRISM");
    let wordcount = std::env::var("BEAM_RUST_WORDCOUNT").expect("set BEAM_RUST_WORDCOUNT");
    let acceptance = std::env::var("BEAM_RUST_ACCEPTANCE").expect("set BEAM_RUST_ACCEPTANCE");
    let directory = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let log_path = directory.path().join("prism.log");
    let log = File::create(&log_path).unwrap();
    let mut runner = Process(
        Command::new(prism)
            .args([
                "--job_port",
                &address.port().to_string(),
                "--serve_http=false",
            ])
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    while TcpStream::connect(address).is_err() {
        assert!(
            runner.0.try_wait().unwrap().is_none(),
            "Prism exited: {}",
            std::fs::read_to_string(&log_path).unwrap()
        );
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
    for (case, success, diagnostic) in [
        ("", true, "pipeline completed"),
        ("large", true, "pipeline completed"),
        ("hot_group", true, "pipeline completed"),
        ("partial_group", true, "pipeline completed"),
        ("finish_emit", true, "pipeline completed"),
        ("slow", true, "pipeline completed"),
        ("fail", false, "requested user failure"),
        ("panic", false, "processor discarded"),
        ("identity", false, "identity mismatch"),
        ("assert", false, "assertion failed"),
        ("empty_assert", false, "assertion failed"),
        ("multiplicity", false, "assertion failed"),
        ("config", false, "invalid utf-8"),
        ("factory_panic", false, "processor discarded"),
        ("setup_fail", false, "requested setup failure"),
        ("setup_panic", false, "processor discarded"),
        ("finish_fail", false, "requested finish failure"),
    ] {
        let runner_offset = std::fs::metadata(&log_path).unwrap().len() as usize;
        let start = Instant::now();
        let driver_log = directory.path().join(format!("driver-{case}.log"));
        let log = File::create(&driver_log).unwrap();
        let mut driver = Process(
            Command::new(if case.is_empty() {
                &wordcount
            } else {
                &acceptance
            })
            .args([
                format!("--endpoint={address}"),
                format!("--case={case}"),
                "--print_counters=true".to_owned(),
            ])
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .spawn()
            .unwrap(),
        );
        let status = wait(&mut driver.0, Duration::from_secs(30));
        let driver_text = std::fs::read_to_string(driver_log).unwrap();
        let runner_log = std::fs::read_to_string(&log_path).unwrap();
        let runner_text = &runner_log[runner_offset..];
        assert_eq!(
            status.success(),
            success,
            "case={case}\n{driver_text}\n{runner_text}"
        );
        assert!(
            driver_text.contains(diagnostic) || runner_text.contains(diagnostic),
            "case={case}: missing {diagnostic}\n{driver_text}\n{runner_text}"
        );
        assert!(
            runner.0.try_wait().unwrap().is_none(),
            "Prism crashed for {case}\n{runner_text}"
        );
        if success {
            let word_counters: Vec<i64> = driver_text
                .lines()
                .filter(|line| {
                    line.contains("committed=true") && line.contains("\"NAME\": \"words\"")
                })
                .map(|line| line.rsplit("value=").next().unwrap().parse().unwrap())
                .collect();
            assert_eq!(
                word_counters,
                if case.is_empty() { vec![6] } else { vec![6, 6] },
                "{driver_text}"
            );
        }
        eprintln!(
            "Prism PROCESS case {case:?}: expected success={success}, elapsed={:?}",
            start.elapsed()
        );
    }
    let text_io = std::env::var("BEAM_RUST_TEXT_IO").expect("set BEAM_RUST_TEXT_IO");
    for case in [
        "roundtrip",
        "empty_file",
        "missing",
        "invalid_utf8",
        "bad_output",
    ] {
        let success = case == "roundtrip" || case == "empty_file";
        let input = directory.path().join(format!("text-{case}.txt"));
        let output = directory.path().join(format!("text-{case}-output"));
        if case != "missing" {
            std::fs::write(
                &input,
                if case == "empty_file" {
                    &b""[..]
                } else if case == "invalid_utf8" {
                    &b"bad\xff\n"[..]
                } else {
                    "red blue\r\n\ncafé\nlast".as_bytes()
                },
            )
            .unwrap();
        }
        if case == "bad_output" {
            std::fs::write(&output, b"a file, not a directory").unwrap();
        }
        let log = File::create(directory.path().join(format!("text-{case}.log"))).unwrap();
        let mut driver = Process(
            Command::new(&text_io)
                .args([
                    format!("--endpoint={address}"),
                    format!("--input={}", input.display()),
                    format!("--output={}", output.display()),
                    "--job_name=rust-native-text".to_owned(),
                ])
                .stdout(log.try_clone().unwrap())
                .stderr(log)
                .spawn()
                .unwrap(),
        );
        let status = wait(&mut driver.0, Duration::from_secs(30));
        let log =
            std::fs::read_to_string(directory.path().join(format!("text-{case}.log"))).unwrap();
        assert_eq!(status.success(), success, "text case={case}: {log}");
        if success {
            let mut actual = Vec::new();
            let mut shard_count = 0;
            for shard in std::fs::read_dir(&output).unwrap() {
                shard_count += 1;
                let shard = shard.unwrap();
                assert!(shard.file_name().to_str().unwrap().starts_with("part-"));
                actual.extend(
                    std::fs::read_to_string(shard.path())
                        .unwrap()
                        .lines()
                        .map(str::to_owned),
                );
            }
            actual.sort();
            assert!(shard_count > 0);
            assert_eq!(
                actual,
                if case == "empty_file" {
                    vec![]
                } else {
                    vec!["", "café", "last", "red blue"]
                }
            );
        } else {
            assert!(
                log.contains("Enum::Failed"),
                "expected worker failure: {log}"
            );
        }
        assert!(
            runner.0.try_wait().unwrap().is_none(),
            "Prism crashed in text case {case}"
        );
        eprintln!(
            "Prism PROCESS text I/O case {case}: expected success={}",
            success
        );
    }
    let log = File::create(directory.path().join("cancel.log")).unwrap();
    let mut cancel = Process(
        Command::new(acceptance)
            .args([
                format!("--endpoint={address}"),
                "--case=slow".to_owned(),
                "--wait_ms=100".to_owned(),
            ])
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .spawn()
            .unwrap(),
    );
    assert!(!wait(&mut cancel.0, Duration::from_secs(10)).success());
    let output = std::fs::read_to_string(directory.path().join("cancel.log")).unwrap();
    assert!(output.contains("cancellation requested"), "{output}");
    let job_id = output
        .lines()
        .find_map(|line| line.strip_prefix("submitted job "))
        .expect("driver job ID")
        .to_owned();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        tokio::time::timeout(Duration::from_secs(5), async {
            let channel = tonic::transport::Endpoint::from_shared(format!("http://{address}"))
                .unwrap()
                .connect()
                .await
                .unwrap();
            let mut jobs = beam_model::job_service_client::JobServiceClient::new(channel);
            loop {
                let mut request = beam_model::GetJobStateRequest::new();
                request.set_job_id(&job_id);
                if jobs.get_state(request).await.unwrap().get_ref().state()
                    == beam_model::job_state::Enum::Cancelled
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .expect("cancel acknowledgement did not reach terminal CANCELLED");
    });
    assert!(runner.0.try_wait().unwrap().is_none());
}
