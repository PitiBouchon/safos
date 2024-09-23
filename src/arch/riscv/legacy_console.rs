struct LegacyConsoleWriter;

pub fn legacy_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    LegacyConsoleWriter.write_fmt(args).unwrap();
}

impl core::fmt::Write for LegacyConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        legayc_sbi_print_str(s);
        Ok(())
    }
}

fn legayc_sbi_print_str(s: &str) {
    for c in s.bytes() {
        if c.is_ascii() {
            sbi::legacy::console_putchar(c);
        }
    }
}
