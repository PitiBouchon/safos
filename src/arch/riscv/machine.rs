use fdt::{Fdt, FdtError};
use spin::Once;

pub unsafe fn setup_machine<'a>(dtb_ptr: usize) -> Result<super::ArchSpecificData<'a>, FdtError> {
    setup_console();

pub unsafe fn set_fdt(dtb_ptr: usize) -> Result<(), FdtError> {
    let fdt = Fdt::from_ptr(dtb_ptr as *const u8)?;

    Ok(super::ArchSpecificData { _fdt })
}
