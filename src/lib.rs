//! Wrapper library for wasmcraft2 datapacks written in Rust.
//!
//! [wasmcraft2](https://github.com/SuperTails/wasmcraft2) is a WebAssembly to Minecraft datapack
//! transpiler. This library provides safe access to wasmcraft's API, containing all functions present
//! in wasmcraft's `mcinterface.h` as well as some additional helper functions.
//!
//! When writing programs for wasmcraft2, it is important to note its limitations - notably, floating
//! point operations are not supported, so using the [`fixed`](https://docs.rs/fixed/latest/fixed/)
//! crate is recommended if integers are not enough. Minecraft programs must be `#![no_main]` and `#![no_std]`; this
//! crate provides a Minecraft-compatible panic handler but there is no allocator. Decreasing the default
//! stack size is recommended - you can do this by adding the following to your `.cargo/config`:
//! ```toml
//! [target.wasm32-unknown-unknown]
//! rustflags = [ "-C", "link-args=-z stack-size=4096" ]
//! ```
//! If more stack space is required, you can change 4096 to some greater number.
//!
//! While you're in `.cargo/config`, you should also set the default target to `wasm32-unknown-unknown`
//! ```toml
//! [build]
//! target = "wasm32-unknown-unknown"
//! ```
//! Enabling some optimisation even in debug builds is recommended, since Minecraft commands are not
//! the fastest compilation target ever - add the following to your `Cargo.toml`:
//! ```toml
//! [profile.dev]
//! opt-level = 1
//! ```
//! wasmcraft2 does not support the `main` function - your entrypoint must be declared as follows:
//! ```no_run
//! #[no_mangle]
//! pub extern fn _start() -> i32 {
//!     // Your code goes here...
//!     return 0;
//! }
//! ```

#![no_std]

#[cfg(feature = "fmt")]
pub mod fmt;

use core::panic::PanicInfo;

/// An enum representing a Minecraft block.
/// This contains all the block types currently supported by wasmcraft2, which is a very limited
/// subset of Minecraft's block selection. There is currently no way to place any other blocks
/// through wasmcraft2.
#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Block {
    Air,
    Cobblestone,
    Granite,
    Andesite,
    Diorite,
    Lapis,
    Iron,
    Gold,
    Diamond,
    Redstone,
    Emerald,
    Dirt,
    OakLog,
    OakLeaves,
}

extern {
    #[link_name = "print"]
    fn _mci_unsafe_print(value: i32);

    /// Store an 8 bit value in memory.
    ///
    /// Safety: `ptr` must be aligned to 32 bytes.
    pub fn store_8(ptr: *mut i32, value: i32);

    #[link_name = "turtle_x"]
    fn _mci_unsafe_turtle_x(value: i32);
    #[link_name = "turtle_y"]
    fn _mci_unsafe_turtle_y(value: i32);
    #[link_name = "turtle_z"]
    fn _mci_unsafe_turtle_z(value: i32);
    #[link_name = "turtle_fill"]
    fn _mci_unsafe_turtle_fill(block: Block, x_span: i32, y_span: i32, z_span: i32);
    #[link_name = "turtle_set"]
    fn _mci_unsafe_turtle_set(block: Block);
    #[link_name = "turtle_get"]
    fn _mci_unsafe_turtle_get() -> Block;
    #[link_name = "turtle_get_char"]
    fn _mci_unsafe_turtle_get_char() -> i32;

    #[link_name = "mc_sleep"]
    fn _mci_unsafe_mc_sleep();
    #[link_name = "mc_putc"]
    fn _mci_unsafe_mc_putc(ch: i32);
}

/// Print an integer to the Minecraft chat.
#[inline]
pub fn print(value: i32) {
    unsafe { _mci_unsafe_print(value) }
}

/// Set the x position of the turtle
#[inline]
pub fn turtle_x(value: i32) {
    unsafe { _mci_unsafe_turtle_x(value) }
}

/// Set the y position of the turtle.
#[inline]
pub fn turtle_y(value: i32) {
    unsafe { _mci_unsafe_turtle_y(value) }
}

/// Set the z position of the turtle.
#[inline]
pub fn turtle_z(value: i32) {
    unsafe { _mci_unsafe_turtle_z(value) }
}

/// Set the position of the turtle. This will call `turtle_x`, `turtle_y` and `turtle_z`, so it is
/// more efficient to call those individually if you do not need to change all 3 coordinates.
#[inline]
pub fn turtle_pos(x: i32, y: i32, z: i32) {
    unsafe {
        _mci_unsafe_turtle_x(x);
        _mci_unsafe_turtle_y(y);
        _mci_unsafe_turtle_z(z);
    }
}


/// Fills a volume relative to the turtle's postion.
/// The x, y, and z span arguments are effectively the size of the region minus one,
/// so `turtle_fill(block, 0, 0, 0)` is equivalent to `turtle_set(block)`
///
/// This function is unstable, and may cause wasmcraft2 to fail compilation.
#[inline]
pub fn turtle_fill(block: Block, x_span: i32, y_span: i32, z_span: i32) {
    unsafe { _mci_unsafe_turtle_fill(block, x_span, y_span, z_span) }
}

/// Set the block at the turtle's position.
#[inline]
pub fn turtle_set(block: Block) {
    unsafe { _mci_unsafe_turtle_set(block) }
}

/// Get the block at the turtle's position.
#[inline]
pub fn turtle_get() -> Block {
    unsafe { _mci_unsafe_turtle_get() }
}

/// Check if the given block is present at the turtle's position.
#[inline]
pub fn turtle_check(block: Block) -> bool {
    block == unsafe { _mci_unsafe_turtle_get() }
}

/// wasmcraft2 documentation is unclear about what this function does; included for completeness.
#[inline]
pub fn turtle_get_char() -> i32 {
    unsafe { _mci_unsafe_turtle_get_char() }
}

/// Pauses execution until the next game tick.
///
/// wasmcraft2 will automatically insert sleep calls before functions and inside loops. However, if
/// your program contains large stretches of code without loops or function calls, it may be necessary
/// to manually insert `mc_sleep()` calls. See the wasmcraft2 README for more information.
#[inline]
pub fn mc_sleep() {
    unsafe { _mci_unsafe_mc_sleep() }
}

/// Write a character to the game chat. Characters will not appear until a newline (`'\n'`) is written.
///
/// Only ASCII printable characters will be printed; any other characters will appear as a � symbol.
#[inline]
pub fn mc_putc(ch: char) {
    unsafe { _mci_unsafe_mc_putc(ch as i32) }
}

/// Print a string to the game chat. Any printed characters will not appear until a newline (`'\n'`) is written.
/// Only ASCII printable characters will be printed; any other characters will appear as a � symbol.
///
/// If you want to print a string to the game chat with a newline, consider using [`println`].
pub fn print_str(s: &str) {
    for c in s.chars() {
        mc_putc(c);
    }
}

/// Print a string to the game chat, with a newline.
/// Only ASCII printable characters will be printed; any other characters will appear as a � symbol.
pub fn println(s: &str) {
    for c in s.chars() {
        mc_putc(c);
    }
    mc_putc('\n');
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println("RUST PANIC - entering infinite loop!");
    loop { mc_sleep(); }
}