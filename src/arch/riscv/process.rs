use super::{encoding, trap};

// How many pages we give to a given a process for their stack
pub const STACK_PAGES: usize = 35;

pub fn create_process(frame: &mut trap::TrapFrame, pc: usize, sp: usize, ra: usize, pid: usize) {
    frame.pc = pc;
    frame.regs[1] = ra;
    frame.regs[2] = sp * 4096; // SP
    frame.mode = encoding::CpuMode::Machine as usize;
    frame.pid = pid;
}
