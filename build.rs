fn main() {
    // Rebuild if the linked script has changed

    #[cfg(target_arch = "riscv64")]
    println!("cargo:rerun-if-changed=src/linker/linker_riscv.ld");
}
