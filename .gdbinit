set architecture riscv:rv64
set disassemble-next-line on
symbol-file target/riscv64imac-unknown-none-elf/debug/safos
set riscv use-compressed-breakpoints yes
target remote :1234
