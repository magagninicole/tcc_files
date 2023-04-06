#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(strail::test_runner)]
#![reexport_test_harness_main = "test_main"]

use strail::process;

#[cfg(test)]
#[no_mangle]
extern "C" fn main() {
    test_main();
    strail::exit_qemu_as_success();
}

#[test_case]
fn test_process_new() {
    strail::arch::mem::init();
    strail::arch::kmem::init();

    fn pc() {}
    let proc = process::Process::new(pc);

    assert!(!proc.is_waiting());
    assert!(!proc.is_dead());
    assert!(!proc.is_running());
    assert!(proc.is_ready());

    assert_eq!(proc.pid, 0);
}
