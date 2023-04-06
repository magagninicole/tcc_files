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

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))] {
        use strail::arch::isa::encoding::Interrupt;
        use strail::arch::isa::trap::{trap_handler, TrapFrame};

        // TODO: handle panic calls on from_usize
        #[test_case]
        fn test_trap_handler_is_called() {
            strail::uart::Uart::init();

            let mut frame = TrapFrame {
                regs: [0; 32],
                pc: 0x123,
                satp: 0x123,
                mode: 0
            };

            assert_eq!(trap_handler(1, 2, 11, 4, 5, &mut frame), 5);
        }

        // TODO: handle panic calls on from_usize
        #[test_case]
        fn test_trap_encoding_convertion() {
            assert_eq!(Interrupt::from_usize(0), Interrupt::UserSoftware,);
            assert_eq!(Interrupt::from_usize(1), Interrupt::SupervisorSoftware,);
            assert_eq!(Interrupt::from_usize(3), Interrupt::MachineSoftware,);
        }
    }
}
