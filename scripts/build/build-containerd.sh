#!/bin/bash
# Copyright 2022 The Kuasar Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -e

git clone -b v0.2.0-kuasar https://github.com/kuasar-io/containerd.git
mkdir bin && make -C containerd bin/containerd && mv containerd/bin/containerd bin

tee bin/config.toml > /dev/null <<EOF
version = 3

[plugins.'io.containerd.cri.v1.runtime']
disable_apparmor = true

[plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes.kuasar-runc]
runtime_type = "io.containerd.kuasar-runc.v1"
sandboxer = "runc"

[plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes.kuasar-vmm]
runtime_type = "io.containerd.kuasar-vmm.v1"
sandboxer = "vmm"
io_type = "streaming"

[plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes.kuasar-quark]
runtime_type = "io.containerd.kuasar-quark.v1"
sandboxer = "quark"

[plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes.kuasar-wasm]
runtime_type = "io.containerd.kuasar-wasm.v1"
sandboxer = "wasm"

[proxy_plugins.vmm]
type = "sandbox"
address = "/run/vmm-sandboxer.sock"

[proxy_plugins.quark]
type = "sandbox"
address = "/run/quark-sandboxer.sock"

[proxy_plugins.wasm]
type = "sandbox"
address = "/run/wasm-sandboxer.sock"

[proxy_plugins.runc]
type = "sandbox"
address = "/run/runc-sandboxer.sock"
EOF
