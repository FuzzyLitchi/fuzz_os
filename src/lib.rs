#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kmain() {

}
