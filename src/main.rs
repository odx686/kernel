#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod vga; 
mod ps2;
mod keyboard;
mod interrupts;
mod gdt;
mod memory;
mod allocator;
use crate::memory::*;
use crate::keyboard::*;
use core::panic::PanicInfo;
use alloc::{string::String, boxed::Box, vec, vec::Vec, rc::Rc, format};
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

entry_point!(main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init_idt();

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut keyboard = Keyboard::new();
    keyboard.initialize();

    let mut shift = false;
    let mut caps = false;
    let mut command = String::new();

    loop {
        print!("> ");
        loop {
            match keyboard.read_input() {
                Some(Key::Letter(mut letter)) => {
                    if shift || caps { letter = letter.to_ascii_uppercase(); }
                    print!("{}", letter);
                    command.push(letter);
                }
                Some(Key::Space) => { 
                    print!(" ");
                    command.push(' ');
                }
                Some(Key::Enter) => {
                    print!("\n");
                    break;
                }
                Some(Key::LeftShift | Key::RightShift) => shift = !shift,
                Some(Key::CapsLock) => caps = !caps,
                _ => ()
            }
        }
        let args: Vec<&str> = command.split_whitespace().collect();
        command.clear();
    }
}