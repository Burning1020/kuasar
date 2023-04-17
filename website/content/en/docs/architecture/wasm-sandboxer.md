---
date: 2023-04-17
draft: false
lastmod: 2023-04-17
linktitle: Wasm Sandboxer
menu:
    docs:
        parent: architecture
        weight: 4
title: Wasm Sandboxer
toc: true
type: docs
---

The wasm sandbox, such as [WasmEdge](https://wasmedge.org/) or [Wasmtime](https://wasmtime.dev/), is incredibly lightweight, but it may have constraints for some applications at present. The `wasm-sandboxer` and `wasm-task` launch containers within a WebAssembly runtime. Whenever containerd needs to start a container in the sandbox, the `wasm-task` will fork a new process, start a new WasmEdge runtime, and run the Wasm code inside it. All containers within the same pod will share the same Namespace/Cgroup resources with the `wasm-task` process.

![vmm-arch](/img/docs/architecture/wasm-arch.png)

*Please note that only WasmEdge is currently supported.*