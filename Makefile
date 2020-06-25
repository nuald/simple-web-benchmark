.PHONY: build
build:
	docker build suite/ -t simple-web-benchmark

.PHONY: scaling_governor
performance_governor:
	sudo cpupower frequency-set -g performance

.PHONY: shell
shell: performance_governor
	docker run -it --rm -v $(shell pwd):/src --network="host" simple-web-benchmark

.PHONY: run
run:
	cargo run --manifest-path suite/Cargo.toml -- all

.PHONY: versions
versions:
	cargo run --manifest-path suite/Cargo.toml --bin versions
