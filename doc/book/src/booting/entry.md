# Entry

To compile the kernel we use a linker script specified in the `.cargo/config.toml` file by `rustflags`

For the qemu virt machine we use the `virt.ld` linker script.


Inside `virt.ld` (in `src/arch/riscv/virt.ld`) we specify the entry point of the OS with the `ENTRY` function

The entry function is what is called after OpenSBI is setup and is defined in `src/arch/riscv/entry.rs`

> `#[naked]` : is used because so the entry funtion is not really a function like a rust function but a symbol with assembly instructions
> 
> > When a normal function is called, the rust compiler will automatically add things, before anything
> inside the function is run : it saves some registers (see the calling convention of the architecture),
> it push a return address to the stack, align the stack...

> `#[no_mangle]` : by default, the Rust compiler mangles symbol names, but here we need to call this function inside 
> the linker script so the name of the function won't change after compilation

> `#[link_section = ".text.entry"]` specify the link section that we put our entry code in. It is in the beginning of the OS in
> the linker script

## Inside the entry function

We first start to zeroing the .BSS section which are data that are considered to be zero by the rust compiler. It is
where static variables are stored.

Then we setup the `sp` (stack pointer) register [^note1] : the stack is where the local variables and other things of the
kernel running will be

At the end of `_entry` we just call the `start` function which is a normal rust function

---

[^note1] : The stack pointer register need to be at the end of the memory range
