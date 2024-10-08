use fdt::{Fdt, FdtError};

pub unsafe fn setup_machine<'a>(dtb_ptr: usize) -> Result<super::ArchSpecificData<'a>, FdtError> {
    let _fdt = Fdt::from_ptr(dtb_ptr as *const u8)?;

    Ok(super::ArchSpecificData { _fdt })
}
