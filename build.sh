#!/usr/bin/env bash
set -e

TARGET=./target.json
TARGET_DIR=target/target/debug
OUT_ELF="$TARGET_DIR/riscv-test"
OUT_BIN="$TARGET_DIR/out.bin"

cargo rustc --target $TARGET \
  -Z build-std=core,compiler_builtins \
  -Z build-std-features=compiler-builtins-mem \
  -- -C link-arg=-Tlink.ld

riscv64-unknown-elf-objcopy -O binary "$OUT_ELF" "$OUT_BIN"
riscv64-unknown-elf-objdump -d target/target/debug/riscv-test -M no-aliases
