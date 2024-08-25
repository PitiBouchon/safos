#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(slice_as_chunks)]

mod arch;

const OS_STACK_SIZE: usize = 4096 * 16;

#[repr(C, align(16))]
struct Stack([u8; OS_STACK_SIZE]);

#[no_mangle]
static OS_STACK: Stack = Stack([0; OS_STACK_SIZE]);

fn kmain(_hart_id: usize) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
