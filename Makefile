ifeq ($(TARGET),)
	TARGET := riscv32imac-unknown-none-elf
endif

.PHONY: all patch

all: patch
	cargo run --package raltool -- generate svd/HPM6750.svd.patched --transform raltool-cfg.yml
	cargo fmt
	cargo check --features hpm6750 --target ${TARGET}

patch: devices/hpm6750.yaml
	svd patch devices/hpm6750.yaml
