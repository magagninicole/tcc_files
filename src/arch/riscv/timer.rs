use super::encoding::{
    read_mhartid, read_mie, write_mie, CLINT_BASE_ADDR, CLINT_MTIME_LO_ADDR, INTERVAL, MIE_MTIE,
    MTIME_REG_HI_ADDR, MTIME_REG_LO_ADDR,
};

fn clint_mtimecmp(hartid: usize) -> usize {
    CLINT_BASE_ADDR + MTIME_REG_HI_ADDR + MTIME_REG_LO_ADDR * hartid
}

pub fn init() {
    let hartid = read_mhartid();

    unsafe {
        (clint_mtimecmp(hartid) as *mut usize)
            .write_volatile((CLINT_MTIME_LO_ADDR as *mut usize).read_volatile() + INTERVAL);

        // enable machine-timer interrupt
        write_mie(read_mie() | MIE_MTIE);
        //let scratch: *mut usize = null_mut();
        //scratch.add(3).write_volatile(clint_mtimecmp(hartid));
        //scratch.add(4).write_volatile(INTERVAL);

        //write_mscratch(scratch as usize);
    }
}
