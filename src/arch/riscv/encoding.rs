/*
Author: Ben Mezger (github.com/benmezger)
*/

pub const CLINT_BASE_ADDR: usize = 0x20000000;
pub const INTERVAL: usize = 20000000;
pub const MTIME_REG_HI_ADDR: usize = 0x4000;
pub const MTIME_REG_LO_ADDR: usize = MTIME_REG_HI_ADDR + 0x0004;
pub const CLINT_MTIME_LO_ADDR: usize = CLINT_BASE_ADDR + 0xbff8;

/* RISC-V encoding */

pub enum CSRegister {
    // machine status register
    Mstatus = 0x300,
    Mie = 0x304,
}

/// Machine Software interrupt
pub const MIP_MSIP: usize = 1 << 3;

/// Machine interrupt enable bit
pub const MSTATUS_MIE: usize = 1 << 3;

/// Machine enable timer interrupt
pub const MIE_MTIE: usize = 1 << 7;

// Execution modes
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum CpuMode {
    User = 0,
    Supervisor = 1,
    Machine = 3,
}

#[derive(Debug, PartialEq)]
pub enum Interrupt {
    UserSoftware = 0,
    SupervisorSoftware = 1,
    MachineSoftware = 3,
    UserTimer = 4,
    SupervisorTimer = 5,
    MachineTimer = 7,
    UserExternal = 8,
    SupervisorExternal = 9,
    MachineExternal = 11,
}

#[repr(usize)]
pub enum Register {
    Zero = 0,
    A0 = 10,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
}

impl Interrupt {
    pub fn from_usize(value: usize) -> Interrupt {
        match value {
            0 => Interrupt::UserSoftware,
            1 => Interrupt::SupervisorSoftware,
            3 => Interrupt::MachineSoftware,
            7 => Interrupt::MachineTimer,
            _ => panic!("Interrupt '{}' not supported", value),
        }
    }
}

#[repr(usize)]
pub enum Exception {
    InstructionAddrMisaligned = 0,
    InstructionAccessFault = 1,
    IllegaInstruction = 2,
    LoadAddrMisaligned = 4,
    LoadAccessFault = 5,
    StoreAMOAddrMisaligned = 6,
    StoreAMOAccessFault = 7,
    EnvCallFromUMode = 8,
    EnvCallFromSMode = 9,
    EnvCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StoreAMOPageFault = 15,
}

impl Exception {
    pub fn from_usize(value: usize) -> Exception {
        match value {
            1 => Exception::InstructionAccessFault,
            2 => Exception::IllegaInstruction,
            5 => Exception::LoadAccessFault,
            9 => Exception::EnvCallFromSMode,
            11 => Exception::EnvCallFromMMode,
            _ => panic!("Exception '{}' not supported", value),
        }
    }
}

/// Check if trap is an exception or interrupt trap
/// If \[31\] = 1 interrupt, otherwise it is an exception
///
/// # Example
///
/// Given `cause` is a given RISC-V m/s/u cause
///
/// * Check if cause is an interrupt, or an asynchronous interrupt
/// `cause >> ASYNC_MASK == 1`
///
/// * Check if cause is an exception, or an synchronous interrupt
/// `cause >> ASYNC_MASK == 0`
pub const CAUSE_MASK: usize = 63 & 1;

/// Read RISC-V's machine's hart id
///
/// # Example
///
/// * Read the current hart thread id
/// `let id = read_mhartid()`
pub fn read_mhartid() -> usize {
    let r: usize;
    unsafe {
        asm!("csrr {}, mhartid", out(reg)r);
    }
    r
}

pub fn read_mie() -> usize {
    let r: usize;
    unsafe {
        asm!("csrr {}, mie", out(reg)r);
    }
    r
}

pub fn write_mie(val: usize) {
    unsafe {
        asm!("csrw mie, {}", in(reg)val);
    }
}

pub fn write_mscratch(val: usize) {
    unsafe {
        asm!("csrw mscratch, {}", in(reg)val);
    }
}

pub fn write_sscratch(val: usize) {
    unsafe {
        asm!("csrw sscratch, {}", in(reg)val);
    }
}

pub fn read_mscratch() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mscratch", out(reg)rval);
        rval
    }
}
