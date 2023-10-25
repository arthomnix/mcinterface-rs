//! Utilities for string formatting.
//!
//! Contains [`MciWriteStream`], a unit struct which implements [`Write`] allowing it to be used
//! with the [`write!`] macro, as well as implementations of [`print!`] and [`println!`] based on
//! this.

use core::fmt::Write;

/// A unit struct that implements [`Write`], allowing it to be used with the [`write!`] and [`writeln!`]
/// macros.
///
/// Usage:
/// ```ignore
/// # use mcinterface::fmt::MciWriteStream;
/// # use core::fmt::Write;
/// write!(MciWriteStream, "Hello, World!\n").unwrap();
/// ```
/// The implementations of [`Write::write_str`] and [`Write::write_char`] will never return an [`Err`] value.
///
/// Text written will appear in the game chat. Text will not appear until a newline is written.
///
/// For cleaner syntax, see the [`print!`] and [`println!`] macros.
pub struct MciWriteStream;

impl Write for MciWriteStream {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::print_str(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        crate::mc_putc(c);
        Ok(())
    }
}

/// An implementation of `print!` using [`MciWriteStream`]. Should behave similarly to `std::print!`,
/// with the caveat that no text will be printed until a newline is printed (due to the fact that
/// Minecraft has no way of modifying a line of text in the chat once it has been sent).
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!($crate::fmt::MciWriteStream, $($arg)*).unwrap();
    }};
}

/// An implementation of `println!` using [`MciWriteStream`]. Should behave similarly to `std::println!`.
#[macro_export]
macro_rules! println {
    () => { mc_putc('\n'); };
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!($crate::fmt::MciWriteStream, $($arg)*).unwrap();
    }};
}