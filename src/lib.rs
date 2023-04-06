/*
Author: Ben Mezger (github.com/benmezger)
*/

#![allow(unused_attributes)]
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![feature(decl_macro)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
pub mod consts;

#[macro_use]
pub mod asm;
#[macro_use]
pub mod macros;

pub mod mmio;
pub mod uart;

pub mod page;

pub mod arch;
pub mod cpu;
pub mod exit;
mod heap;
pub mod process;
pub mod sched;
pub mod syscall;

extern crate alloc;
/// Prints to the standard output
///
/// At the moment, this calls write! with Uart's interface.
///
/// # Panics
///
/// Panics if writting to standard output fails.
///
/// # Examples
///
/// ```
/// print!("Hello, world\n");
/// printl!("Hello from {}\n", rust);
/// ```
//From: https://github.com/sgmarz/osblog/blob/master/risc_v/src/main.rs
#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::uart::Uart::init(), $($args)+);
    });
}

/// Prints to the standard output appending a new line
///
/// # Panics
///
/// Panics if writting to standard output fails.
///
/// # Examples
///
/// ```rust
/// println!("Hello, world");
/// printlln!("Hello from {}", rust);
/// ```
#[macro_export]
macro_rules! println
{
    () => ({
        crate::print!("\r\n")
    });
    ($fmt:expr) => ({
        crate::print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        crate::print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

#[no_mangle]
extern "C" fn eh_personality() {}

#[cfg(all(test, feature = "test-wrap-panic"))]
macro_rules! UNREACHEBLE {
    () => {
        loop {}
    };
}

/// Kernel's panic handler
///
/// Writes info to standard output and waits for interrupt
///
/// # Panics
///
/// Panics if writting to standard output fails.
///
/// # Features
///
/// If `test-wrap-panic` is set, `test_`
#[cfg(not(all(test, feature = "test-wrap-panic")))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Panic: ");
    match info.location() {
        Some(_p) => crate::println!(
            "line {}, file {}: {}",
            _p.line(),
            _p.file(),
            info.message().unwrap()
        ),
        None => println!("no information available."),
    }
    abort()
}

/// Default panic for tests
#[allow(unreachable_code)]
#[cfg(all(test, feature = "test-wrap-panic"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu_as_failure();

    UNREACHEBLE!();
}

/// Abort by waiting for interrupt
///
/// Loops waiting for interrupt
///
/// # Examples
///
/// ```rust
/// abort();
/// ```
#[no_mangle]
pub extern "C" fn abort() -> ! {
    #[cfg(all(feature = "close-on-panic"))]
    exit_qemu_as_failure();

    #[allow(unreachable_code)]
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

/// Trait for running test functions
///
/// # Example
///
/// ```rust
/// impl<T> Testable for T
/// where
///     T: Fn(),
/// {
///     fn run(&self) {
///         self();
///     }
/// }
/// ```
pub trait Testable {
    /// Runs the test function
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        crate::println!("{}...\t", core::any::type_name::<T>());
        self();
        crate::println!("[ok]");
    }
}

/// Exit Qemu with a failed rc
///
/// This is expected to never return
pub fn exit_qemu_as_failure() -> ! {
    arch::sys::exit(exit::ExitCode::Failed);
    // we shouldn't get here
    panic!("We shoudn't get here.");
}

/// Exit Qemu with a success rc
///
/// This is expected to never return
pub fn exit_qemu_as_success() -> ! {
    arch::sys::exit(exit::ExitCode::Success);
    // we shouldn't get here
    panic!("We shoudn't get here.");
}

/// The test runner for all tests
///
/// Expects to receive an array of Testable functions.
/// This is expected to never return.
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu_as_success();
}

#[cfg(test)]
#[no_mangle]
extern "C" fn main() {
    uart::Uart::init();
    test_main();
}
