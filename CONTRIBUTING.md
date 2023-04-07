# Welcome
Welcome to kuasar!

### Main components

The table below lists the core parts of the project:

| Component              | Description                                                                              |
|------------------------|------------------------------------------------------------------------------------------|
| [vmm](vmm)             | Source code of vmm sandbox, including vmm-sandboxer, vmm-task and some building scripts. |
| [quark](quark)         | Quark sandboxer that can run a container by quark.                                       |
| [wasm](wasm)           | Wasm sandboxer that can run a container by WebAssembly runtime.                          |
| [shim](shim)           | An optional workaround of inactive with containerd                                       |
| [documentations](docs) | Documentations of all sandboxer architecture.                                            |
| [tests](tests)         | Benchmark tests and e2e tests directory.                                                 |
| [examples](examples)   | Examples of how to run a container via Kuasar sandboxers.                                |
