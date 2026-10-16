#!/usr/bin/env python3
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements. See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""Fetch checksum-pinned generation tools. Not used by ordinary Cargo builds."""
import hashlib
import io
from pathlib import Path
import platform
import urllib.request
import zipfile

if platform.system() != "Linux" or platform.machine() != "x86_64":
    raise SystemExit("this bootstrap supports Linux x86_64; see README for manual upstream builds")

destination = Path(__file__).resolve().parents[1] / ".tools"
for directory, url, checksum in [
    ("protoc", "https://github.com/protocolbuffers/protobuf/releases/download/v34.0/protoc-34.0-linux-x86_64.zip", "e9a91b6fcfe4177ec2cd35fc8f15c1e811fa0ecdef9372755cd6d3513d5faaab"),
    ("grpc", "https://github.com/grpc/grpc-rust/releases/download/protoc-gen-rust-grpc-v0.9.0/protoc-gen-rust-grpc-0.9.0-linux-x86_64.zip", "c0e93891f39f8188c23d2026af972f07547066fb3c2973f26148f9b4d52c5320"),
]:
    binary = "protoc" if directory == "protoc" else "protoc-gen-rust-grpc"
    target = destination / directory
    marker = target / "archive.sha256"
    if marker.exists() and marker.read_text().strip() == checksum and (target / "bin" / binary).is_file():
        continue
    data = urllib.request.urlopen(url, timeout=120).read()
    if hashlib.sha256(data).hexdigest() != checksum:
        raise SystemExit("checksum mismatch: " + url)
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        for name in archive.namelist():
            if not (target / name).resolve().is_relative_to(target.resolve()):
                raise SystemExit("invalid archive member: " + name)
        archive.extractall(target)
    (target / "bin" / binary).chmod(0o755)
    marker.write_text(checksum + "\n")
print(destination)
