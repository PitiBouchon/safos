use crate::arch::riscv::machine::setup_machine;
use crate::{OS_STACK, OS_STACK_SIZE};
use core::arch::asm;

use super::legacy_console::legacy_print;

#[naked]
#[no_mangle]
// SAFETY: must have the same link_section name as in the linker script
#[link_section = ".text.entry"]
// SAFETY: must have the same function name as in the linker
unsafe extern "C" fn _entry() -> ! {
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
        // Load the stack pointer
        "la sp, {STACK0}",
        // Add the stack size to the stack pointer
        "li t0, {OS_STACK_SIZE}",
        "add sp, sp, t0",
        // Call start function
        "call {START}",
        STACK0 = sym OS_STACK,
        OS_STACK_SIZE = const OS_STACK_SIZE,
        START = sym start,
        options(noreturn)
    )
}

#[no_mangle]
pub unsafe extern "C" fn start(hart_id: usize, dtb: usize) -> ! {
    legacy_print(format_args!("HELLO FROM SafOS"));

    match setup_machine(dtb) {
        Err(err) => {
            legacy_print(format_args!("Couldn't setup machine: {:?}", err));
            legacy_print(format_args!("Looping infinitely"));
            #[allow(clippy::empty_loop)]
            loop {}
        }
        Ok(arch_specific_data) => crate::kmain(hart_id, arch_specific_data),
    }
}
