HYPERVISOR ?= cloud_hypervisor
GUESTOS_IMAGE ?= centos
WASI_RUNTIME ?= wasmedge
KERNEL_VERSION ?= 6.2

all: vmm quark wasi

bin/vmm-sandboxer:
	@cd vmm && cargo build --release --features=${HYPERVISOR}
	@mkdir -p bin && cp vmm/target/release/vmm-sandboxer bin/vmm-sandboxer 

bin/vmm-task:
	@cd vmm/task && cargo build --release --target=x86_64-unknown-linux-musl
	@mkdir -p bin && cp vmm/target/x86_64-unknown-linux-musl/release/vmm-task bin/vmm-task

bin/vmlinux.bin:
	@bash -x vmm/scripts/kernel/${HYPERVISOR}/build.sh ${KERNEL_VERSION}
	@mkdir -p bin && cp vmm/scripts/kernel/${HYPERVISOR}/vmlinux.bin bin/vmlinux.bin && rm vmm/scripts/kernel/${HYPERVISOR}/vmlinux.bin

bin/kuasar.img:
	@bash -x vmm/scripts/image/${GUESTOS_IMAGE}/build.sh
	@mkdir -p bin && cp /tmp/kuasar.img bin/kuasar.img && rm /tmp/kuasar.img

bin/wasi-sandboxer:
	@cd wasi && cargo build --release --features=${WASI_RUNTIME}
	@mkdir -p bin && cp wasi/target/release/wasi-sandboxer bin/wasi-sandboxer

bin/quark-sandboxer:
	@cd quark && cargo build --release
	@mkdir -p bin && cp quark/target/release/quark-sandboxer bin/quark-sandboxer

vmm: bin/vmm-sandboxer bin/kuasar.img bin/vmlinux.bin
wasi: bin/wasi-sandboxer
quark: bin/quark-sandboxer

clean:
	@rm -rf bin
	@cd vmm && cargo clean
	@cd wasi && cargo clean
	@cd quark && cargo clean

install: all
	@install -p -m 755 bin/vmm-sandboxer /usr/local/bin/vmm-sandboxer
	@install -d /var/lib/kuasar
	@install -p -m 644 bin/kuasar.img /var/lib/kuasar/kuasar.img
	@install -p -m 644 bin/vmlinux.bin /var/lib/kuasar/vmlinux.bin
	@install -p -m 644 vmm/sandbox/config_clh.toml /var/lib/kuasar/config_clh.toml
	@install -p -m 755 bin/wasi-sandboxer /usr/local/bin/wasi-sandboxer
	@install -p -m 755 bin/quark-sandboxer /usr/local/bin/quark-sandboxer
	@install -p -m 755 bin/wasi-sandbo /usr/local/bin/wasi-sandboxer
	@install -p -m 755 bin/wasi-sandboxer /usr/local/bin/wasi-sandboxer


.PHONY: vmm wasi quark clean all install

