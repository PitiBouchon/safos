use fdt::{Fdt, FdtError};
use spin::Once;

static FDT: Once<Fdt> = Once::new();

pub unsafe fn set_fdt(dtb_ptr: usize) -> Result<(), FdtError> {
    let fdt = Fdt::from_ptr(dtb_ptr as *const u8)?;

    FDT.call_once(|| fdt);
    Ok(())
}
