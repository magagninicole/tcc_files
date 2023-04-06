/*
Author: Ben Mezger (github.com/benmezger)
*/

#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(strail::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]

use strail::*;

pub mod mmio;

extern crate alloc;

fn kinfo() {
    println!();
    unsafe {
        println!(
            "TEXT:   0x{:x} -> 0x{:x}",
            consts::TEXT_START,
            consts::TEXT_END
        );
        println!(
            "RODATA: 0x{:x} -> 0x{:x}",
            consts::RODATA_START,
            consts::RODATA_END
        );
        println!(
            "DATA:   0x{:x} -> 0x{:x}",
            consts::DATA_START,
            consts::DATA_END
        );
        println!(
            "BSS:    0x{:x} -> 0x{:x}",
            consts::BSS_START,
            consts::BSS_END
        );
        println!(
            "STACK:  0x{:x} -> 0x{:x}",
            consts::STACK_START,
            consts::STACK_END
        );
        println!(
            "HEAP: 0x{:x} -> 0x{:x}",
            arch::kmem::get_head() as usize,
            arch::kmem::get_head() as usize + arch::kmem::get_num_allocations() * 4096
        );
    }
    println!();
}

// The kernel's main entrypoint
// This is called from boot.S
#[cfg(not(test))]
#[no_mangle]
extern "C" fn main() {
    strail::uart::Uart::init();
    crate::arch::mem::init();
    crate::arch::kmem::init();

    println!("Initializing the kernel..");
    kinfo();

    arch::mem::print_page_allocation();

    let ret = process::init();
    println!("Init process created at address 0x{:08x}", ret);

    sched::init_sched(2);
    arch::frame::switch(sched::schedule(), consts::SwitchMode::User);
}

// The kernel's test main function
// This test main calls test_main() from our custom testing framework.
// This is required as we don't want to fully run the kernel's main()
// when running tests
#[cfg(test)]
#[no_mangle]
extern "C" fn main() {
    strail::uart::Uart::init();
    test_main()
}
