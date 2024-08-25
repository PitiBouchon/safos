# Safos

Safe OS microkernel written in Rust for the RiscV architecture

## Dependencies

In order to build and run the OS you'll need :
- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- [cargo-make](https://github.com/sagiegurari/cargo-make) which you can install via `cargo install cargo-make`
- [QEMU](https://www.qemu.org/download/) for RiscV
- *(for debug)* gdb compatible with riscv (such as gdb-multiarch or riscv64-unknown-elf-gdb)

## Build & Run

You can use the usual commands `cargo build` and `cargo run` to run the OS using QEMU

## Debugging with gdb

Run in a terminal `cargo make qemu-riscv-gdb` 
and in another terminal (in this directory) run your riscv compatible gdb *(you may need to enable the .gdbinit script)*
