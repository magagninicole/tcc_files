/*
Author: Ben Mezger (github.com/benmezger)
 */

use core::fmt;

// The frequency of QEMU is 10 MHz
pub const FREQ: u64 = 10_000_000;
// Let's do this 250 times per second for switching
pub const CONTEXT_SWITCH_TIME: u64 = FREQ / 250;

extern "C" {
    pub static HEAP_START: usize;
    pub static HEAP_SIZE: usize;

    pub static TEXT_START: usize;
    pub static TEXT_END: usize;

    pub static RODATA_START: usize;
    pub static RODATA_END: usize;

    pub static DATA_START: usize;
    pub static DATA_END: usize;

    pub static BSS_START: usize;
    pub static BSS_END: usize;

    pub static STACK_START: usize;
    pub static STACK_END: usize;
    pub static UART_ADDRESS: usize;

    pub static KERNEL_STACK_START: usize;
    pub static KERNEL_STACK_END: usize;

}

pub const STACK_PAGES: usize = 5;

cfg_if::cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        pub const STACK_ADDR: usize = 0x1_0000_0000;
    } else {
        pub const STACK_ADDR: usize = 0x1_0000_000;
    }
}

pub const PROCESS_START_ADDR: usize = 0x2000_0000;

pub enum SwitchMode {
    User,
    Supervisor,
}

impl fmt::Display for SwitchMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwitchMode::User => write!(f, "User"),
            SwitchMode::Supervisor => write!(f, "Supervisor"),
        }
    }
}
