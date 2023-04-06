use crate::exit::{Exit, ExitCode};

// from: https://github.com/qemu/qemu/blob/master/include/hw/misc/sifive_test.h
const SUCCESS: u32 = 0x55_55;
const FAIL: u32 = 0x33_33;
const RESET: u32 = 0x77_77;
const EXIT_ADDRESS: usize = 0x10_0000;

#[allow(dead_code)]
pub struct RISCVExit {
    addr: usize,
    reason: ExitCode,
}

fn lookup_exitcode(code: ExitCode) -> u32 {
    match code {
        ExitCode::Success => SUCCESS,
        ExitCode::Failed => FAIL,
        _ => RESET,
    }
}

impl Exit for RISCVExit {
    fn new(addr: usize, reason: ExitCode) -> Self {
        RISCVExit { addr, reason }
    }

    fn exit(&self, reason: ExitCode) -> ! {
        unsafe {
            asm!("sw {0}, 0({1})", in(reg)lookup_exitcode(reason), in(reg)self.addr);

            loop {
                asm!("wfi");
            }
        };
    }

    fn exit_success(&self) -> ! {
        self.exit(ExitCode::Success)
    }

    fn exit_failure(&self) -> ! {
        self.exit(ExitCode::Failed)
    }
}

pub fn exit(reason: ExitCode) {
    RISCVExit::new(EXIT_ADDRESS as usize, reason).exit(reason)
}
