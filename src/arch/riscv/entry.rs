use crate::arch::riscv::machine::set_fdt;
use crate::{OS_STACK, OS_STACK_SIZE};
use core::arch::asm;
use core::ptr::{addr_of, addr_of_mut};

// ANCHOR: entry
#[naked]
#[no_mangle]
#[link_section = ".text.entry"] // SAFETY: must have the same link_section name as in the linker
unsafe extern "C" fn _entry() -> ! {
    // SAFETY: must have the same function name as in the linker
    asm!(
        // TODO: zeroing the .BSS section
        "la sp, {STACK0}",
        "li t0, {OS_STACK_SIZE}",
        "add sp, sp, t0",
        "call {START}",
        STACK0 = sym OS_STACK,
        OS_STACK_SIZE = const OS_STACK_SIZE,
        START = sym start,
        options(noreturn)
    )
}
// ANCHOR_END: entry

#[no_mangle]
pub unsafe extern "C" fn start(hart_id: usize, dtb: usize) -> ! {
    extern "C" {
        // SAFETY: must have the same name as in the linker script
        static mut _start_bss: u8;
        static mut _end_bss: u8;
    }

    // Zeroing the .BSS section (which correspond to uninitialized values)
    // TODO : I think it should be done in assembly
    let bss_size = (addr_of!(_end_bss) as usize)
        .checked_sub(addr_of!(_start_bss) as usize)
        .unwrap();
    core::ptr::write_bytes(addr_of_mut!(_start_bss), 0, bss_size);

    set_fdt(dtb).unwrap();

    crate::main(hart_id)
}
