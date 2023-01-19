.PHONY: all patch

all: patch
	cargo run --package raltool -- generate ./svd/HPM6750.svd.patched --transform ./raltool-cfg.yml
	cargo fmt
	cargo check --features hpm6750

patch: devices\hpm6750.yaml
	svdtools patch ./devices/hpm6750.yaml

