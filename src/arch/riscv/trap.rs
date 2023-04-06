/*
Author: Ben Mezger (github.com/benmezger)
*/

use core::fmt;

use super::encoding::{Exception, Interrupt, Register};
use super::switch::switch;

/// `TrapFrame` from a trap
///
/// This holds the trapframe for handling trap.
///
/// At the moment, this is RISC-V specific.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub regs: [usize; 32],  // 0 - 255
    pub fregs: [usize; 32], // 246 - 511
    pub satp: usize,        // 512 - 519
    pub pc: usize,          // 520
    pub hartid: usize,      // 528
    pub qm: usize,          // 536
    pub pid: usize,         //544
    pub mode: usize,        // 552
}

/// Implements `fmt::Display::fmt` for `TrapFrame`
///
/// # Examples
///
/// ```rust
/// crate::println!(trapframe);
/// ```
impl fmt::Display for TrapFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrapFrame (sp) at {:p}", self)
    }
}

impl TrapFrame {
    pub fn set_sp(&mut self, addr: usize) {
        self.regs[2] = addr;
    }

    pub const fn zero() -> Self {
        TrapFrame {
            regs: [0; 32],
            fregs: [0; 32],
            satp: 0,
            pc: 0,
            hartid: 0,
            qm: 0,
            pid: 0,
            mode: 0,
        }
    }

    pub fn syscall_id(&mut self) -> usize {
        self.regs[Register::A7 as usize]
    }
}

/// Handles the trap and calls `TrapFrame.handle` architecture specifc implemetation
///
/// `trap_handler` returns the return address via a0
#[no_mangle]
pub extern "C" fn trap_handler(
    epc: usize,
    tval: usize,
    cause: usize,
    hartid: usize,
    _status: usize,
    frame: *mut TrapFrame,
) {
    cfg_if::cfg_if! {
        if #[cfg(target_pointer_width = "64")] {
            let is_async: bool = cause >> 63 & 1 == 1;
        }
        else {
            let is_async: bool = cause >> 31 & 1 == 1;
        }
    }

    let retpc = epc;
    let cause_num = cause & 0xfff;

    if is_async {
        match Interrupt::from_usize(cause_num) {
            Interrupt::MachineSoftware => {
                crate::println!("Machine software interrupt CPU#{}", hartid);
            }
            Interrupt::MachineTimer => {
                crate::sched::init_sched(1);
                let next_frame = crate::sched::schedule();

                if next_frame != 0 {
                    switch(next_frame, crate::consts::SwitchMode::User);
                }
            }
            _ => {
                panic!("Unhandled async trap CPU#{} -> {}", hartid, cause_num);
            }
        }
    } else {
        match Exception::from_usize(cause_num) {
            Exception::EnvCallFromMMode => {
                crate::println!("Environment call from M-mode");

                unsafe {
                    crate::syscall::make_syscall(retpc, frame, 0);
                }

                crate::sched::init_sched(1);
                let next_frame = crate::sched::schedule();
                switch(next_frame, crate::consts::SwitchMode::User);
            }

            Exception::EnvCallFromSMode => {
                crate::println!("Environment call from S-mode");
            }
            Exception::IllegaInstruction => {
                panic!(
                    "Illegal instruction CPU#{} -> 0x{:08x}: 0x{:08x}\n",
                    hartid, epc, tval
                );
            }
            _ => {
                panic!("Unhandled sync trap CPU#{} -> {}\n", hartid, cause_num);
            }
        }
    }
}
