#![feature(lang_items, const_fn, unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

const VALUE_START: usize = 0x150000;
const VALUE_SIZE: usize = 1024;

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    vga_buffer::clear_screen();

    println!("Booted");

    let mut first: u64 = 0;
    let mut second: u64 = 1;
    for pointer in (0..VALUE_SIZE).map(|value| value * 8 + VALUE_START) {
        unsafe{ *(pointer as *mut _) = first };

        let tmp = second;
        second = first.wrapping_add(second);
        first = tmp;
    }

    println!("Successfuly saved {} values", VALUE_SIZE);

    for pointer in (0..VALUE_SIZE).map(|value| value * 8 + VALUE_START) {
        println!("{}", unsafe{ *(pointer as *mut u64) });
    }

    loop {}
}
