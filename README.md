# rrMicroOS
Little OS written in Rust for Risc-V core

# Prerequisites

For now use the toolchain without compressed extension:
rustup target add riscv32i-unknown-none-elf

For some commands the cargo-binutils are required:

cargo install cargo-binutils
rustup component add llvm-tools

# Build

cargo build --target riscv32imac-unknown-none-elf

# Generate binary FW file

cargo objcopy -- -O binary rr_micro_os.bin

# Print the disassembly

cargo objdump -- -d .\target\riscv32imac-unknown-none-elf\debug\rr_micro_os.elf