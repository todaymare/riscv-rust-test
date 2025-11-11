# RISC-V Rust Test
a minimal test to compile fib(30) in rust for RISC-V, specifically targetting my own risc-v emulator.
it writes the result to `a0` and calls `ecall 93` which the emulator interprets as "exit"

thanks for coming to my ted talk
