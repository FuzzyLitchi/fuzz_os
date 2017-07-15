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

static VALUE_START: usize = 0x150000;
static VALUE_SIZE: usize = 10;

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    vga_buffer::clear_screen();

    println!("Booted");

    let mut pointer = VALUE_START;
    let mut first: u64 = 0;
    let mut second: u64 = 1;
    for _ in 0..VALUE_SIZE {
        unsafe{ *(pointer as *mut _) = first };

        let tmp = second;
        second = first.wrapping_add(second);
        first = tmp;

        pointer += 8;
    }

    println!("Successfuly saved {} values", VALUE_SIZE);

    let mut pointer = VALUE_START;
    for _ in 0..VALUE_SIZE {
        println!("{}", unsafe{ *(pointer as *mut u64) });
        pointer += 8;
    }

    loop {}
}
