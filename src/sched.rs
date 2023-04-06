use crate::consts;
use crate::process::{State, PROCESS_LIST, TMR_VALUES_LIST};
use alloc::vec;
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
                            if prc.is_tmr{ 
                               if let Some(mut tmr) = TMR_VALUES_LIST.take() {
                                if tmr.len() >= 3 {
                                crate::println!("aaaaaaaa ");
                                let result1 = tmr.pop_front().unwrap();
                                let result2 = tmr.pop_front().unwrap();
                                let result3 = tmr.pop_front().unwrap();
                                TMR_VALUES_LIST.replace(tmr);
                                validate(result1, result2, result3);
                                }
                                break 'procfindloop;
                            } 
                            }else {
                                frame_addr = prc.get_frame_addr();
                                break 'procfindloop;
                            }
                        }
                
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

pub fn validate(result1: usize, result2: usize, result3: usize) -> usize {
    let mut results = vec![];
    results.push(result1);
    results.push(result2);
    results.push(result3);

    let mut output = 0;

    for i in 0..(results.len() - 1) {
        if results[i] == results[i + 1] {
            output = results[i];
            break;
        }
    }
    crate::println!("{}", output);
    output
}
//int global i pra cada vez que o processo roda i++, random valor