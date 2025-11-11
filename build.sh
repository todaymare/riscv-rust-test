#!/usr/bin/env bash
set -e

MODE="debug"
CARGO_FLAGS=""
if [[ "$1" == "--release" ]]; then
  MODE="release"
  CARGO_FLAGS="--release"
fi

TARGET=./riscv-target.json
TARGET_DIR=target/riscv-target/$MODE
OUT_ELF="$TARGET_DIR/riscv-test"
OUT_BIN="$TARGET_DIR/out.bin"

cargo rustc --target $TARGET \
  $CARGO_FLAGS \
  -Z build-std=core,compiler_builtins \
  -Z build-std-features=compiler-builtins-mem \
  -- -C link-arg=-Tlink.ld

riscv64-unknown-elf-objcopy -O binary "$OUT_ELF" "$OUT_BIN"
riscv64-unknown-elf-objdump -d "$OUT_ELF" -M no-aliases
