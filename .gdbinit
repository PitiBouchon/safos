set confirm off
set architecture riscv:rv64
set disassemble-next-line on
set riscv use-compressed-breakpoints yes
file target/riscv64imac-unknown-none-elf/debug/safos
target remote :1234
b *0x80200000
