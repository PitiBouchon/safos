use fdt::Fdt;

mod entry;
mod legacy_console;
mod machine;

pub struct ArchSpecificData<'a> {
    _fdt: Fdt<'a>,
}
