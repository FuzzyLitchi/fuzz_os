#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
    loop {}
}

extern crate rlibc;

#[no_mangle]
pub extern fn kmain() {
    let test = (0..3).flat_map(|x| 0..x).zip(0..);
}
