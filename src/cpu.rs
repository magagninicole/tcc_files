/// The global kernel trap frame stores 8 separate
/// frames -- one per CPU hart. We will switch these
/// in and out and store a dormant trap frame with
/// the process itself.
pub static mut KERNEL_TRAP_FRAME: [crate::arch::isa::trap::TrapFrame; 8] =
    [crate::arch::isa::trap::TrapFrame::zero(); 8];

pub unsafe fn dump_registers(frame: *const crate::arch::isa::trap::TrapFrame) {
    for i in 1..(*frame).regs.len() - 1 {
        crate::println!("x{:2}:{:08x}   ", i, (*frame).regs[i]);
    }
}
