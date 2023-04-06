#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(strail::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
#[no_mangle]
extern "C" fn main() {
    test_main();
    strail::exit_qemu_as_success();
}

#[test_case]
fn test_mmio_read_write() {
    let mut addr: u8 = 10;

    unsafe {
        strail::mmio::mmio_write(&mut addr, 2, 12);
        assert_eq!(strail::mmio::mmio_read(&mut addr, 2), 12);
    }
}
