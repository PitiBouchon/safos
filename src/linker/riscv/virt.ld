OUTPUT_ARCH(riscv64imac);
ENTRY(_entry); /* SAFETY: must have the same name as the function in entry.rs */

SECTIONS
{
    . = 0x80200000;

    PROVIDE(_kernel_start = .);

    .text : {
        *(.text.entry) /* SAFETY: must have the same name section as in entry.rs */
        ASSERT(. - _kernel_start > 0, "Error: no entry code");
        *(.text .text.*)
    }

    .rodata : {
        . = ALIGN(16);
        *(.srodata .srodata.*) /* do not need to distinguish this from .rodata */
        . = ALIGN(16);
        *(.rodata .rodata.*)
    }

    .data : {
        . = ALIGN(16);
        PROVIDE( __global_pointer$ = . ); /* TODO: should be placed at the middle of .sdata see : https://gnu-mcu-eclipse.github.io/arch/riscv/programmer/ */
        *(.sdata .sdata.*) /* do not need to distinguish this from .data */
        . = ALIGN(16);
        *(.data .data.*)
    }

    .bss : {
        PROVIDE(_start_bss = .); /* SAFETY: must have the same name as in entry.rs*/
        . = ALIGN(16);
        *(.sbss .sbss.*) /* do not need to distinguish this from .bss */
        . = ALIGN(16);
        *(.bss .bss.*)
        PROVIDE(_end_bss = .);
    }

    PROVIDE(_kernel_end = .);
};
