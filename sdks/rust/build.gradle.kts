/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

plugins { base }

description = "Apache Beam :: SDKs :: Experimental Rust"

val cargoFmt = tasks.register<Exec>("cargoFmt") {
  workingDir(projectDir)
  commandLine("cargo", "fmt", "--all", "--", "--check")
}
val cargoClippy = tasks.register<Exec>("cargoClippy") {
  workingDir(projectDir)
  commandLine("cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings")
}
val cargoTest = tasks.register<Exec>("cargoTest") {
  workingDir(projectDir)
  commandLine("cargo", "test", "--workspace", "--locked")
}
tasks.register("preCommit") { dependsOn(cargoFmt, cargoClippy, cargoTest) }
tasks.named("check") { dependsOn("preCommit") }
tasks.register<Exec>("protocolTest") {
  workingDir(projectDir)
  commandLine("bash", "tools/test-interop.sh")
}
tasks.register<Exec>("prismTest") {
  workingDir(projectDir)
  commandLine("bash", "tools/test-prism.sh")
}
tasks.register<Exec>("packageTest") {
  workingDir(projectDir)
  commandLine("python3", "tools/test-packages.py")
}
tasks.register<Exec>("regenerationCheck") {
  workingDir(projectDir)
  commandLine("python3", "tools/generate.py", "--check",
    "--protoc", ".tools/protoc/bin/protoc",
    "--plugin", ".tools/grpc/bin/protoc-gen-rust-grpc",
    "--include", ".tools/protoc/include")
}
