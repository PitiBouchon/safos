use crate::{OS_STACK, OS_STACK_SIZE};
use core::arch::asm;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"] // SAFETY: must have the same link_section name as in the linker
unsafe extern "C" fn _entry() -> ! {
    // SAFETY: must have the same function name as in the linker
    asm!(
        // Zeroing the .BSS section since its assumed to be zero on first access
        "lla t0, _start_bss", // SAFETY: must have the same name as in the linker script
        "lla t1, _end_bss", // SAFETY: must have the same name as in the linker script
        "1:",
        "beq t0, t1, 2f",
        "sd zero, (t0)",
        "addi t0, t0, 8",
        "j 1b",
        "2:",
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

#[no_mangle]
pub unsafe extern "C" fn start(hart_id: usize, dtb: usize) -> ! {
    crate::kmain(hart_id)
}
