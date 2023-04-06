/*
Author: Ben Mezger (github.com/benmezger)
*/

use crate::consts::UART_ADDRESS;
use crate::mmio::mmio_write;
use core::fmt;
use lazy_static::lazy_static;

// TODO: use a spinlock here. Reading and writting to this will be unsafe
// and easily introduce data races and other problems
lazy_static! {
    pub static ref GLOBAL_UART: Uart = Uart::new(unsafe { UART_ADDRESS });
}

/* word lenght of line control register (LCR) */
const UART_LCR_REG: usize = (1 << 0) | (1 << 1);

#[derive(Debug, Clone, Copy)]
pub struct Uart {
    base_address: usize,
    set: bool,
    pub uart_put_func: fn(usize, u8),
}

/// Writes a character to UART's `base_addr`
fn uart_put(base_addr: usize, c: u8) {
    unsafe {
        mmio_write(base_addr as *mut u8, 0, c);
    }
}

impl Uart {
    /// Creates a new `Uart` with `base_address`
    pub fn new(base_address: usize) -> Self {
        Uart {
            base_address,
            uart_put_func: uart_put,
            set: true,
        }
    }

    /// Initializes the `Uart` device
    pub fn init() -> Self {
        if (*GLOBAL_UART).set {
            return *GLOBAL_UART;
        }

        unsafe {
            mmio_write((*GLOBAL_UART).base_address as *mut u8, UART_LCR_REG, 0x0);
        }
        *GLOBAL_UART
    }

    /// Writes a character to `Uart`
    pub fn put(&mut self, c: u8) {
        (self.uart_put_func)(self.base_address, c);
    }

    /// Check if UART's address is set
    pub fn is_set(self) -> bool {
        self.set
    }
}

/// Implements the `Write` trait for `Uart`
impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl fmt::Display for Uart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Uart {}, is {} on put callback {:p}",
            self.base_address,
            if self.is_set() { "set" } else { "not set" },
            self.uart_put_func as *const (),
        )
    }
}
