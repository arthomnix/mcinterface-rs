#![no_std]

use core::panic::PanicInfo;

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

#[inline]
pub fn print(value: i32) {
    unsafe { _mci_unsafe_print(value) }
}

#[inline]
pub fn turtle_x(value: i32) {
    unsafe { _mci_unsafe_turtle_x(value) }
}

#[inline]
pub fn turtle_y(value: i32) {
    unsafe { _mci_unsafe_turtle_y(value) }
}

#[inline]
pub fn turtle_z(value: i32) {
    unsafe { _mci_unsafe_turtle_z(value) }
}

#[inline]
pub fn turtle_pos(x: i32, y: i32, z: i32) {
    unsafe {
        _mci_unsafe_turtle_x(x);
        _mci_unsafe_turtle_y(y);
        _mci_unsafe_turtle_z(z);
    }
}

#[inline]
pub fn turtle_fill(block: Block, x_span: i32, y_span: i32, z_span: i32) {
    unsafe { _mci_unsafe_turtle_fill(block, x_span, y_span, z_span) }
}

#[inline]
pub fn turtle_set(block: Block) {
    unsafe { _mci_unsafe_turtle_set(block) }
}

#[inline]
pub fn turtle_get() -> Block {
    unsafe { _mci_unsafe_turtle_get() }
}

#[inline]
pub fn turtle_check(block: Block) -> bool {
    block == unsafe { _mci_unsafe_turtle_get() }
}

#[inline]
pub fn turtle_get_char() -> i32 {
    unsafe { _mci_unsafe_turtle_get_char() }
}

#[inline]
pub fn mc_sleep() {
    unsafe { _mci_unsafe_mc_sleep() }
}

#[inline]
pub fn mc_putc(ch: i32) {
    unsafe { _mci_unsafe_mc_putc(ch) }
}

pub fn print_str(s: &str) {
    for c in s.chars() {
        mc_putc(c as i32);
    }
}

pub fn println(s: &str) {
    for c in s.chars() {
        mc_putc(c as i32);
    }
    mc_putc('\n' as i32);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println("RUST PANIC - entering infinite loop!");
    loop { mc_sleep(); }
}