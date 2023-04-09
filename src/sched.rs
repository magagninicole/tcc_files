use crate::consts;
use crate::process::{State, PROCESS_LIST};

// TODO: move this to RISC-V
pub const MMIO_MTIMECMP: *mut u64 = 0x0200_4000usize as *mut u64;
pub const MMIO_MTIME: *const u64 = 0x0200_BFF8 as *const u64;

pub fn init_sched(qm: u16) {
    unsafe {
        MMIO_MTIMECMP.write_volatile(
            MMIO_MTIME
                .read_volatile()
                .wrapping_add(consts::CONTEXT_SWITCH_TIME * qm as u64),
        );
    }
}

pub fn schedule() -> usize {
    let mut frame_addr: usize = 0x1111;
    unsafe {
        if let Some(mut pl) = PROCESS_LIST.take() {
            // Rust allows us to label loops so that break statements can be
            // targeted.
            'procfindloop: loop {
                pl.rotate_left(1);
                if let Some(prc) = pl.front_mut() {
                    match prc.state {
                        State::Running => {
                            frame_addr = prc.get_frame_addr();
                            crate::println!("Frame Address: {}", frame_addr);
                            break 'procfindloop;
                        }
                        //State::Sleeping => {
                        //    // Awaken sleeping processes whose sleep until is in
                        //    // the past.
                        //    if prc.sleep_until <= get_mtime() {
                        //        prc.state = ProcessState::Running;
                        //        frame_addr = prc.frame as usize;
                        //        break 'procfindloop;
                        //    }
                        //}
                        _ => {}
                    }
                }
            }
            PROCESS_LIST.replace(pl);
        } else {
            crate::println!("could not take process list");
        }
    }
    frame_addr
}