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

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    vga_buffer::clear_screen();

    println!("Hello {}{}", "World", '!');

    let boot_info = unsafe{ multiboot2::load(multiboot_info_addr) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
    .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    loop {}
}
