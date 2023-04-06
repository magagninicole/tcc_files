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
fn test_new_uart() {
    let uart = strail::uart::Uart::new(unsafe { strail::consts::UART_ADDRESS });
    assert!(uart.is_set());
    unsafe {
        assert_eq!(
            strail::mmio::mmio_read(
                &mut (strail::consts::UART_ADDRESS as u8),
                (1 << 0) | (1 << 1)
            ),
            0x0
        );
    }
}

#[test_case]
fn test_global_uart_is_set() {
    assert_eq!(strail::uart::GLOBAL_UART.is_set(), true);
}
