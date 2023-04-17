---
date: 2023-04-17
draft: false
lastmod: 2023-04-17
linktitle: MicroVM Sandboxer
menu:
    docs:
        parent: architecture
        weight: 2
title: MicroVM Sandboxer
toc: true
type: docs
---

In the microVM sandbox scenario, the VM process provides complete virtual machines and Linux kernels based on open-source VMMs such as [Cloud Hypervisor](https://www.cloudhypervisor.org/), [StratoVirt](https://gitee.com/openeuler/stratovirt), [Firecracker](https://firecracker-microvm.github.io/) and [QEMU](https://www.qemu.org/). Hence, the `vmm-sandboxer` of MicroVM sandboxer is responsible for launching VMs and calling APIs, and the `vmm-task`, as the init process in VMs, plays the role of running container processes. The container IO can be exported via vsock or uds.

The microVM sandboxer avoids the necessity of running shim process on the host, bringing about a cleaner and more manageable architecture with only one process per pod.

![vmm-arch](/img/docs/architecture/vmm-arch.png)

*Please note that only Cloud Hypervisor, StratoVirt and QEMU are supported currently.*