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

"""Verify exact Cargo tarballs outside Beam, with unpublished sibling patches."""
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile

SDK = Path(__file__).resolve().parents[1]
metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--format-version=1", "--no-deps"], cwd=SDK))
target = Path(metadata["target_directory"])
subprocess.run(["cargo", "package", "--workspace", "--locked", "--allow-dirty", "--no-verify", "--exclude-lockfile"], cwd=SDK, check=True)
with tempfile.TemporaryDirectory(prefix="beam-rust-packages-") as temporary:
    root = Path(temporary)
    if root.is_relative_to(SDK.parents[1]):
        raise SystemExit("package verification directory must be outside Beam")
    packages = [package for package in metadata["packages"] if package["id"] in metadata["workspace_members"]]
    members = []
    patches = []
    for package in packages:
        name = package["name"] + "-" + package["version"]
        archive = target / "package" / (name + ".crate")
        with tarfile.open(archive) as tar:
            tar.extractall(root, filter="data")
        members.append(name)
        patches.append(f'{package["name"]} = {{ path = {json.dumps(name)} }}\n')
        for required in ("LICENSE", "NOTICE", "src/lib.rs"):
            if not (root / name / required).is_file():
                raise SystemExit(f"package {name} lacks {required}")
    # Crates are not published during development. Patches resolve only the
    # extracted siblings; dependencies and source are otherwise unmodified.
    (root / "Cargo.toml").write_text('[workspace]\nresolver = "3"\nmembers = ' + json.dumps(members + ["consumer"]) + '\n[patch.crates-io]\n' + ''.join(patches))
    consumer = root / "consumer/src"
    consumer.mkdir(parents=True)
    facade = next(package for package in packages if package["name"] == "apache-beam")
    facade_directory = facade["name"] + "-" + facade["version"]
    (consumer.parent / "Cargo.toml").write_text('[package]\nname="beam-package-consumer"\nversion="0.0.0"\nedition="2024"\n[dependencies]\napache-beam={path=' + json.dumps("../" + facade_directory) + '}\n')
    (consumer / "main.rs").write_text('fn main() { let app = apache_beam::Application::new("package-smoke"); let mut p = app.pipeline().unwrap(); let input = p.create("Input", vec![1_i64, 1, 2]).unwrap(); p.assert_contents("Expected", &input, vec![1, 2, 1]).unwrap(); }\n')
    toolchain = subprocess.check_output(["rustup", "show", "active-toolchain"], cwd=SDK, text=True).split()[0]
    environment = dict(os.environ, CARGO_TARGET_DIR=str(target / "package-validation"), RUSTUP_TOOLCHAIN=toolchain, PROTOC="/unavailable-during-package-verification")
    subprocess.run(["cargo", "generate-lockfile", "--offline"], cwd=root, env=environment, check=True)
    # Cargo archives normalize source mtimes. A shared target directory can
    # otherwise reuse an old unpublished crate with the same version/mtime.
    # Keep third-party build caches, but always rebuild the extracted sources.
    clean = ["cargo", "clean", "--locked", "--offline"]
    for package in packages:
        clean.extend(["-p", package["name"]])
    clean.extend(["-p", "beam-package-consumer"])
    subprocess.run(clean, cwd=root, env=environment, check=True)
    subprocess.run(["cargo", "test", "--workspace", "--locked", "--offline"], cwd=root, env=environment, check=True)
    subprocess.run(["cargo", "run", "-p", "beam-package-consumer", "--locked", "--offline"], cwd=root, env=environment, check=True)
print("Packaged crates and a native consumer passed outside the Beam checkout")
