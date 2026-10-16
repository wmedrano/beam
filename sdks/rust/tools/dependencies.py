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

"""Record locked dependency license metadata, including opt-in probe features."""
import argparse
import csv
import io
import json
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--check", action="store_true")
args = parser.parse_args()
sdk = Path(__file__).resolve().parents[1]
metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--all-features", "--format-version=1"], cwd=sdk))
contents = io.StringIO()
writer = csv.writer(contents, lineterminator="\n")
writer.writerow(["name", "version", "SPDX_license", "repository"])
for package in sorted(metadata["packages"], key=lambda p: (p["name"], p["version"])):
    if package["id"] in metadata["workspace_members"]:
        continue
    if not package.get("license"):
        raise SystemExit("missing license metadata: " + package["id"])
    writer.writerow([package["name"], package["version"], package["license"], package.get("repository") or ""])
header = "\n".join(Path(__file__).read_text().splitlines()[1:15]) + "\n\n"
expected = header + contents.getvalue()
destination = sdk / "DEPENDENCIES.csv"
if args.check:
    if destination.read_text() != expected:
        raise SystemExit("dependency inventory differs; run tools/dependencies.py")
else:
    destination.write_text(expected)
print("Dependency metadata inventory " + ("checked" if args.check else "generated"))
