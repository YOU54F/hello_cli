SHELL := /bin/bash



generate_headers:
	@which cbindgen > /dev/null || cargo install cbindgen
	rustup run nightly cbindgen --config cbindgen.toml --crate pact_cli --output include/pact.h

TARGET=
BINARY_NAME?=pact_cli
SLIM=false

ifeq ($(TARGET),)
	TARGET := $(shell rustup show | grep 'Default host' | awk '{print $$3}')
endif


# Shows a list of available targets for cross-compilation
target_list = $(shell rustup target list)
rustup_target_list:
	@echo "$(target_list)" | sed 's/([^)]*)//g' | tr ' ' '\n' | sed '/^\s*$$/d'

is_slim:
	echo $(SLIM)

# Build the release version of the library
# Can be used to build for a specific target by setting the TARGET environment variable
# e.g. `make cargo_build_release TARGET=x86_64-unknown-linux-gnu`
# by default will use the host target
cargo_build_release:
	echo "Building for target: $(TARGET)"
	if [[ $(SLIM) == "true" ]]; then \
		if [[ "$(shell uname -s)" == "Linux" ]]; then \
			sudo apt install libstd-rust-dev; \
		fi; \
		rustup toolchain install nightly; \
		rustup component add rust-src --toolchain nightly; \
		cargo +nightly install cross --git https://github.com/cross-rs/cross; \
	elif [[ $(TARGET) == "aarch64-unknown-freebsd" ]]; then \
		if [[ "$(shell uname -s)" == "Linux" ]]; then \
			sudo apt install libstd-rust-dev; \
		fi; \
		cargo +nightly install cross --git https://github.com/cross-rs/cross; \
	elif [[ $(TARGET) == *"android"* ]] || [[ $(TARGET) == "x86_64-unknown-netbsd" ]] || [[ $(TARGET) == "x86_64-pc-windows-gnu" ]] || [[ $(TARGET) == "x86_64-unknown-freebsd" ]]; then \
		echo "installing latest cross"; \
		cargo install cross --git https://github.com/cross-rs/cross; \
	elif [[ $(SLIM) == "true" ]]; then \
		if [[ "$(shell uname -s)" == "Linux" ]]; then \
			sudo apt install libstd-rust-dev; \
			rustup toolchain install nightly; \
			rustup component add rust-src --toolchain nightly; \
		else \
			rustup toolchain install nightly $(TARGET); \
			rustup component add rust-src --toolchain nightly --target $(TARGET); \
		fi; \
		cargo +nightly install cross@0.2.5; \
	else \
		cargo install cross@0.2.5; \
	fi
	if [[ $(SLIM) == "true" ]]; then \
		echo "building slimmest binaries"; \
		if [[ $(TARGET) == "aarch64-unknown-freebsd" ]]; then \
			echo "building with cargo nightly, plus std and core for aarch64-unknown-freebsd"; \
			RUSTFLAGS="-Zlocation-detail=none" cross +nightly build -Z build-std=std,panic_abort,core,alloc,proc_macro --profile release-aarch64-freebsd --target=$(TARGET); \
			mv target/aarch64-unknown-freebsd/release-aarch64-freebsd target/aarch64-unknown-freebsd/release; \
		else \
			if [[ $(TARGET) == "aarch64-unknown-linux-musl" ]] || [[ $(TARGET) == "armv5te-unknown-linux-musleabi" ]]; then \
				RUSTFLAGS="-Zlocation-detail=none" cross +nightly build -Z build-std=std,panic_abort,core,alloc,proc_macro -Z build-std-features=panic_immediate_abort --target=$(TARGET) --bin $(BINARY_NAME) --release; \
				RUSTFLAGS="-Ctarget-feature=-crt-static -Zlocation-detail=none" cross +nightly build -Z build-std=std,panic_abort,core,alloc,proc_macro -Z build-std-features=panic_immediate_abort --target=$(TARGET) --lib --release; \
			elif [[ $(TARGET) == *"musl"* ]]; then \
				RUSTFLAGS="-Zlocation-detail=none" cross +nightly build -Z build-std=std,panic_abort,core,alloc,proc_macro -Z build-std-features=panic_immediate_abort --target=$(TARGET) --bin $(BINARY_NAME) --release; \
				RUSTFLAGS="-Ctarget-feature=-crt-static -Zlocation-detail=none" cross +nightly build -Z build-std=std,panic_abort,core,alloc,proc_macro -Z build-std-features=panic_immediate_abort --target=$(TARGET) --lib --release; \
			else \
				RUSTFLAGS="-Zlocation-detail=none" cross +nightly build -Z build-std=std,panic_abort,core,alloc,proc_macro -Z build-std-features=panic_immediate_abort --target=$(TARGET) --release; \
			fi; \
		fi \
	elif [[ $(TARGET) == "aarch64-unknown-freebsd" ]]; then \
		echo "building with cargo nightly, plus std and core for aarch64-unknown-freebsd"; \
		cross +nightly build -Z build-std=std,core,alloc,proc_macro --profile release-aarch64-freebsd --target=$(TARGET); \
		mv target/aarch64-unknown-freebsd/release-aarch64-freebsd target/aarch64-unknown-freebsd/release; \
	elif [[ $(TARGET) == *"musl"* ]]; then \
		RUSTFLAGS="-Zlocation-detail=none" cross build --release --target=$(TARGET) --bin $(BINARY_NAME); \
		RUSTFLAGS="-Ctarget-feature=-crt-static -Zlocation-detail=none" cross build --release --target=$(TARGET) --lib; \
	else \
		cross build --release --target=$(TARGET); \
	fi