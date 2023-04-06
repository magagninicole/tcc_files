/*
Author: Ben Mezger (github.com/benmezger)
*/

pub mod asm;

pub mod encoding;
pub mod exit;
pub mod page;
pub mod process;
pub mod switch;
pub mod timer;
pub mod trap;

#[no_mangle]
extern "C" fn kinit_hart(hartid: usize) {
    // All non-0 harts initialize here.
    unsafe {
        // We have to store the kernel's table. The tables will be moved
        // back and forth between the kernel's table and user
        // applicatons' tables.
        encoding::write_mscratch(
            (&mut crate::cpu::KERNEL_TRAP_FRAME[hartid] as *mut trap::TrapFrame) as usize,
        );
        // Copy the same mscratch over to the supervisor version of the
        // same register.
        encoding::write_sscratch(encoding::read_mscratch());
        crate::cpu::KERNEL_TRAP_FRAME[hartid].hartid = hartid;
        // We can't do the following until zalloc() is locked, but we
        // don't have locks, yet :( cpu::KERNEL_TRAP_FRAME[hartid].satp
        // = cpu::KERNEL_TRAP_FRAME[0].satp;
        // cpu::KERNEL_TRAP_FRAME[hartid].trap_stack = page::zalloc(1);
    }
}
