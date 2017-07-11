#![feature(lang_items, const_fn, unique)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
    loop {}
}

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn kmain() -> ! {
    vga_buffer::clear_screen();

    println!("Hello {}{}", "World", '!');
    loop {}
}
