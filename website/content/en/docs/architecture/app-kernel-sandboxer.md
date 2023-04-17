---
date: 2023-04-17
draft: false
lastmod: 2023-04-17
linktitle: App Kernel Sandboxer
menu:
    docs:
        parent: architecture
        weight: 3
title: App Kernel Sandboxer
toc: true
type: docs
---

The app kernel sandbox launches a KVM virtual machine and a guest kernel, without any application-level hypervisor or Linux kernel. This allows for customized optimization to speed up startup procedure, reduce memory overheads, and improve IO and network performance. Examples of such app kernel sandboxes include [gVisor](https://gvisor.dev/) and [Quark](https://github.com/QuarkContainer/Quark).

Quark is an application kernel sandbox that utilizes its own hypervisor named `QVisor` and a customized kernel called `QKernel`. With customized modifications to these components, Quark can achieve significant performance.

The `quark-sandboxer` of app kernel sandboxer starts `Qvisor` and an app kernel named `Qkernel`. Whenever containerd needs to start a container in the sandbox, the `quark-task` in `QVisor` will call `Qkernel` to launch a new container. All containers within the same pod will be running within the same process.

![vmm-arch](/img/docs/architecture/quark-arch.png)

*Please note that only Quark is currently supported.*