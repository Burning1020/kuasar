---
date: 2023-04-17
draft: false
lastmod: 2023-04-17
linktitle: Build
menu:
  docs:
    parent: developer guide
    weight: 1
title: Build
toc: true
type: docs
---

## Prerequisites

### 1. OS

+ The minimum versions of Linux distributions supported by Kuasar are *Ubuntu 22.04* or *CentOS 8* or openEuler 23.03.

+ Please also note that Quark requires a Linux kernel version >= 5.15.

### 2. Sandbox

+ MicroVM: To launch a microVM-based sandbox, a hypervisor must be installed on the host.
  + It is recommended to install Cloud Hypervisor by default. You can find Cloud Hypervisor installation instructions [here](https://github.com/cloud-hypervisor/cloud-hypervisor/blob/main/docs/building.md).
  + If you want to run kuasar with iSulad container engine and StratoVirt hypervisor, you can refer to this guide [how-to-run-kuasar-with-isulad-and-stratovirt](docs/vmm/how-to-run-kuasar-with-isulad-and-stratovirt.md).
+ Quark: To use Quark, please refer to the installation instructions [here](https://chat.openai.com/chat/docs/quark/README.md).
+ WasmEdge: To start WebAssembly sandboxes, you need to install WasmEdge. Instructions for installing WasmEdge can be found in [install.html](https://wasmedge.org/book/en/quick_start/install.html).


### 3. containerd

Kuasar sandboxers are external plugins of containerd, so both containerd and its CRI plugin are required in order to manage the sandboxes and containers.

We offer two ways to interact Kuasar with containerd:

+ **EXPERIMENTAL in containerd 2.0 milestone**: If you desire the full experience of Kuasar, please install [containerd under kuasar-io organization](https://github.com/kuasar-io/kuasar/blob/main/docs/containerd.md). Rest assured that our containerd is built based on the official v1.7.0, so there is no need to worry about missing any functionalities.

+ If the compatibility is a real concern, you need to install official containerd v1.7.0 with an extra [kuasar-shim](https://github.com/kuasar-io/kuasar/tree/main/shim) for request forwarding, see [here](https://github.com/kuasar-io/kuasar/blob/main/docs/shim/README.md). However, it's possible that this way may be deprecated in the future as containerd 2.0 evolves.

### 4. crictl

Since Kuasar is built on top of the Sandbox API, which has already been integrated into the CRI of containerd, it makes sense to experience Kuasar from the CRI level.

`crictl` is a debug CLI for CRI. To install it, please see [here](https://github.com/kubernetes-sigs/cri-tools/blob/master/docs/crictl.md#install-crictl)

### 5. virtiofsd

MicroVMs like Cloud Hypervisor needs a virtiofsd to share the directories on the host. Please refer to [virtiofsd guide](https://gitlab.com/virtio-fs/virtiofsd#building-from-sources).


## Downloading source

Rust 1.67 or higher version is required to compile Kuasar.

```shell
git clone https://github.com/kuasar-io/kuasar.git
```

## Compiling Kuasar

```bash
cd kuasar
make all
make install
```

## Start Kuasar

Launch the sandboxers by the following commands:

+ For vmm: `nohup vmm-sandboxer --listen /run/vmm-sandboxer.sock --dir /run/kuasar-vmm &`
+ For quark: `nohup quark-sandboxer --listen /run/quark-sandboxer.sock --dir /var/lib/kuasar-quark &`
+ For wasm: `nohup wasm-sandboxer --listen /run/wasm-sandboxer.sock --dir /run/kuasar-wasm &`

## Start Container

Since Kuasar is a low-level container runtime, all interactions should be done via CRI in containerd, such as crictl or Kubernetes. We use crictl as examples:

+ For vmm and quark, run the following [scripts](https://github.com/kuasar-io/kuasar/blob/main/examples/run_example_container.sh):

  `./run_example_container.sh vmm` or `./run_example_container.sh quark`

+ For wasm: Wasm container needs its own container image so our script has to build and import the container image at first. Run the following [scripts](https://github.com/kuasar-io/kuasar/blob/main/examples/run_example_wasm_container.sh):

  `./run_example_wasm_container.sh`
