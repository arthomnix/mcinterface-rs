use core::fmt::Write;

pub struct MciWriteStream;

impl Write for MciWriteStream {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::print_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!($crate::fmt::MciWriteStream, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    () => { mc_putc('\n'); };
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!($crate::fmt::MciWriteStream, $($arg)*).unwrap();
    }};
}