use crate::{cpu, process};
use core::convert::{TryFrom, TryInto};
use crate::process::{State, TMR_VALUES_LIST};

#[repr(usize)]
pub enum Syscall {
    Nop = 0,
    DumpRegisters,
    Sleep,
    Exit,
    TmrAdd
}

impl TryFrom<usize> for Syscall {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Syscall::Nop as usize => Ok(Syscall::Nop),
            x if x == Syscall::DumpRegisters as usize => Ok(Syscall::DumpRegisters),
            x if x == Syscall::Sleep as usize => Ok(Syscall::Sleep),
            x if x == Syscall::Exit as usize => Ok(Syscall::Exit),
            x if x == Syscall::TmrAdd as usize => Ok(Syscall::TmrAdd),
            _ => Err(()),
        }
    }
}

extern "C" {
    fn _make_syscall(
        sysno: usize, // n da chamada
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize;
}

pub unsafe fn make_syscall(pc: usize, frame_ptr: *mut crate::arch::isa::trap::TrapFrame,total: usize) {
    if frame_ptr.is_null() {
        return;
    }

    let frame = frame_ptr.as_mut().unwrap();
    let syscall_id = frame.syscall_id(); // processo

    // skip ecall
    frame.pc = pc + 4;
    match syscall_id.try_into() {
        Ok(Syscall::Nop) => {
            crate::println!("NOP");
        }
        Ok(Syscall::DumpRegisters) => {
            crate::println!("Registers");
            cpu::dump_registers(frame);
        }
        Ok(Syscall::Sleep) => {
            crate::println!("Sleeping");
            let proc = process::sleep_pid((*frame).pid, 1);
            crate::println!("Process is {}", proc);
        }
        Ok(Syscall::Exit) => {
            crate::println!("Exiting. Bye.");
            crate::abort()
        }
        Ok(Syscall::TmrAdd) => {
            if let Some(mut tmr) = unsafe { TMR_VALUES_LIST.take() } {
                crate::println!("{}", total);
                tmr.push_back(total);
                TMR_VALUES_LIST.replace(tmr);
                } else {
                crate::println!("not gonna happen");
                }    
        }
        Err(_) => panic!("Unknown syscall {}", syscall_id),
    }
}

pub fn syscall_nop() -> usize {
    unsafe { _make_syscall(Syscall::Nop as usize, 0, 0, 0, 0, 0, 0) }
}

pub fn syscall_dump() -> usize {
    unsafe { _make_syscall(Syscall::DumpRegisters as usize, 0, 0, 0, 0, 0, 0) }
}

pub fn syscall_sleep() -> usize {
    unsafe { _make_syscall(Syscall::Sleep as usize, 0, 0, 0, 0, 0, 0) }
}

pub fn syscall_exit() -> usize {
    unsafe { _make_syscall(Syscall::Exit as usize, 0, 0, 0, 0, 0, 0) }
}

pub fn syscall_push_tmr(total: usize) -> usize {
    unsafe { _make_syscall(Syscall::TmrAdd as usize, total, 0, 0, 0, 0, 0) }
}
