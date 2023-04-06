/*
Author: Ben Mezger (github.com/benmezger)
*/

#[macro_use]
#[cfg_attr(target_arch = "riscv32", path = "riscv/mod.rs")]
#[cfg_attr(target_arch = "riscv64", path = "riscv/mod.rs")]
pub mod isa;

pub mod mem {
    pub fn flush_hw_cache(id: usize) {
        crate::arch::isa::page::flush_hw_cache(id);
    }

    pub fn print_page_allocation() {
        crate::arch::isa::page::print_page_allocation();
    }

    pub fn init() {
        crate::arch::isa::timer::init();
        crate::arch::isa::page::init();
    }

    pub fn map(
        root: &mut crate::arch::isa::page::Table,
        vaddr: usize,
        paddr: usize,
        bits: crate::page::PageBits,
        level: usize,
    ) {
        crate::arch::isa::page::map(
            root,
            vaddr,
            paddr,
            crate::arch::isa::page::PageEntryBits::from(bits).val(),
            level,
        )
    }

    pub fn unmap(root: &mut crate::arch::isa::page::Table) {
        crate::arch::isa::page::unmap(root)
    }

    pub fn zalloc(pages: usize) -> *mut u8 {
        crate::arch::isa::page::zalloc(pages)
    }

    pub fn alloc(pages: usize) -> *mut u8 {
        crate::arch::isa::page::alloc(pages)
    }

    pub fn dealloc(ptr: *mut u8) {
        crate::arch::isa::page::dealloc(ptr)
    }

    pub fn page_count() -> usize {
        crate::arch::isa::page::page_count()
    }

    pub fn last_page() -> *mut u8 {
        crate::arch::isa::page::get_last_page()
    }
}

pub mod kmem {
    use core::fmt;
    use core::mem::size_of;
    use core::ptr::null_mut;

    // TODO: This will fail in 32bit target
    #[allow(clippy::enum_clike_unportable_variant)]
    #[repr(usize)]
    enum AllocFlags {
        #[cfg(target_arch = "riscv64")]
        Taken = 1 << 63,
        #[cfg(target_arch = "riscv32")]
        Taken = 1 << 31,
    }

    impl AllocFlags {
        pub fn val(self) -> usize {
            self as usize
        }
    }

    #[derive(Debug)]
    struct Allocation {
        pub flags_size: usize,
    }

    impl Allocation {
        fn is_taken(&self) -> bool {
            self.flags_size & AllocFlags::Taken.val() != 0
        }

        fn is_free(&self) -> bool {
            !self.is_taken()
        }

        fn set_taken(&mut self) {
            self.flags_size |= AllocFlags::Taken.val()
        }

        fn set_free(&mut self) {
            self.flags_size &= !AllocFlags::Taken.val();
        }

        fn set_size(&mut self, size: usize) {
            let k = self.is_taken();
            self.flags_size = size & !AllocFlags::Taken.val();
            if k {
                self.flags_size |= AllocFlags::Taken.val();
            }
        }

        fn get_size(&self) -> usize {
            self.flags_size & !AllocFlags::Taken.val()
        }
    }

    impl fmt::Display for Allocation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Allocation at {:p}", self)
        }
    }

    static mut KMEM_HEAD: *mut Allocation = null_mut();
    static mut KMEM_ALLOC: usize = 0;
    static mut KMEM_PAGE_TABLE: *mut crate::arch::isa::page::Table = null_mut();

    // TODO: Add safety doc
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_head() -> *mut u8 {
        KMEM_HEAD as *mut u8
    }

    pub fn get_num_allocations() -> usize {
        unsafe { KMEM_ALLOC }
    }

    pub fn init() {
        unsafe {
            KMEM_ALLOC = 2048;
            let k = crate::arch::mem::zalloc(KMEM_ALLOC);
            crate::dbg!(k);
            KMEM_HEAD = k as *mut Allocation;
            (*KMEM_HEAD).set_free();
            (*KMEM_HEAD).set_size(KMEM_ALLOC * crate::arch::isa::page::PAGE_SIZE);
            KMEM_PAGE_TABLE = crate::arch::mem::zalloc(1) as *mut crate::arch::isa::page::Table;

            crate::dbg!(KMEM_PAGE_TABLE);
            crate::dbg!(KMEM_HEAD);

            init_identity_map()
        }
    }

    fn init_identity_map() {
        unsafe {
            let num_pages = crate::consts::HEAP_START
                + crate::consts::HEAP_SIZE / crate::arch::isa::page::PAGE_SIZE;

            crate::arch::isa::page::ident_map_range(
                &mut *KMEM_PAGE_TABLE,
                crate::arch::kmem::get_head() as usize,
                crate::arch::kmem::get_head() as usize
                    + crate::arch::kmem::get_num_allocations() * 4096,
                crate::arch::isa::page::PageEntryBits::from(crate::page::PageBits::ReadWrite).val(),
            );

            crate::arch::isa::page::ident_map_range(
                &mut *KMEM_PAGE_TABLE,
                crate::consts::HEAP_START,
                crate::consts::HEAP_START + num_pages,
                crate::arch::isa::page::PageEntryBits::from(crate::page::PageBits::ReadWrite).val(),
            );

            crate::arch::isa::page::ident_map_range(
                &mut *KMEM_PAGE_TABLE,
                crate::consts::TEXT_START,
                crate::consts::TEXT_END,
                crate::arch::isa::page::PageEntryBits::from(crate::page::PageBits::ReadExecute)
                    .val(),
            );

            crate::arch::isa::page::ident_map_range(
                &mut *KMEM_PAGE_TABLE,
                crate::consts::RODATA_START,
                crate::consts::RODATA_END,
                crate::arch::isa::page::PageEntryBits::from(crate::page::PageBits::ReadExecute)
                    .val(),
            );

            crate::arch::isa::page::ident_map_range(
                &mut *KMEM_PAGE_TABLE,
                crate::consts::DATA_START,
                crate::consts::DATA_END,
                crate::arch::isa::page::PageEntryBits::from(crate::page::PageBits::ReadWrite).val(),
            );

            crate::arch::isa::page::ident_map_range(
                &mut *KMEM_PAGE_TABLE,
                crate::consts::BSS_START,
                crate::consts::BSS_END,
                crate::arch::isa::page::PageEntryBits::from(crate::page::PageBits::ReadWrite).val(),
            );
        }
    }

    pub fn kzmalloc(size: usize) -> *mut u8 {
        let size = crate::arch::isa::page::align_val(size, 3);
        let ret = kmalloc(size);

        if !ret.is_null() {
            for i in 0..size {
                unsafe {
                    (*ret.add(i)) = 0;
                }
            }
        }

        ret
    }

    pub fn kmalloc(size: usize) -> *mut u8 {
        unsafe {
            let size = crate::arch::isa::page::align_val(size, 3) + size_of::<Allocation>();

            let mut head = KMEM_HEAD;
            let tail = (KMEM_HEAD as *mut u8).add(KMEM_ALLOC * crate::arch::isa::page::PAGE_SIZE)
                as *mut Allocation;

            while head < tail {
                if (*head).is_free() && size <= (*head).get_size() {
                    let chunk_size = (*head).get_size();
                    let rem = chunk_size - size;

                    (*head).set_taken();

                    if rem > size_of::<Allocation>() {
                        let next = (head as *mut u8).add(size) as *mut Allocation;

                        (*next).set_free();
                        (*next).set_size(rem);
                        (*head).set_size(size);
                    } else {
                        (*head).set_size(chunk_size);
                    }
                    return head.add(1) as *mut u8;
                } else {
                    head = (head as *mut u8).add((*head).get_size()) as *mut Allocation;
                }
            }
        }
        null_mut()
    }
}

pub mod frame {
    pub fn switch(frame: usize, mode: crate::consts::SwitchMode) -> ! {
        crate::arch::isa::switch::switch(frame, mode)
    }

    pub fn create_process(
        frame: &mut crate::arch::isa::trap::TrapFrame,
        pc: usize,
        sp: usize,
        ra: usize,
        pid: usize,
    ) {
        crate::arch::isa::process::create_process(frame, pc, sp, ra, pid)
    }
}

pub mod sys {
    pub fn exit(reason: crate::exit::ExitCode) {
        crate::arch::isa::exit::exit(reason)
    }
}
