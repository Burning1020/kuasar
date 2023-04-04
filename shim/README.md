# What is this?

Shim is a kind of CRI runtime V2 backend, it has connected to containerd and kuasar sandboxer process,
playing a role of "proxy". To be specific, it will forward Sandbox API to sandboxer and forward Task API
to low to tasker in low level indirectly, and try best not handle the original request. It was only created for linking
up containerd v1.7.0 and kuasar sandboxer, so maybe it will be **deprecated** in the future.

![](../docs/shim/images/arch.png)

# Usage
This directory supply two shims, `containerd-shim-kuasar-vmm-v2` and `containerd-shim-kuasar-wasm-v2`.
The former one will talk to vmm-sandboxer and the later one will talk to wasm-sandboxer.

## Build
You need build it from source before use it:
```shell
cd kuasar/shim
make all
make install
```

## Configure Containerd
After that, containerd should know this runtime. Modify containerd's configuration toml file on the host,
the default path is `/etc/containerd/config.toml`:

For vmm:
```toml
        [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.kuasar-vmm]
          runtime_type = "io.containerd.kuasar-vmm.v2"
          sandbox_mode = "shim"
```

For wasm:

```toml
        [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.kuasar-wasm]
          runtime_type = "io.containerd.kuasar-wasm.v2"
          sandbox_mode = "shim"
```

Please Make sure `runtime_type` and `sandbox_mode` is correct.

## Run the container

Use `kuasar-vmm` or `kuasar-wasm` as the CRI runtime type to run a container by shim.

