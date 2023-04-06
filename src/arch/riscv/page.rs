/*
Author: Ben Mezger (github.com/benmezger)
*/

use crate::println;
use core::{mem::size_of, ptr::null_mut};

use crate::page::Entry;
use crate::page::Table as KTable;

/* see rodata section in boot.S */
extern "C" {
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}

// We will use ALLOC_START to mark the start of the actual
// memory we can dish out.
static mut ALLOC_START: usize = 0;
const PAGE_ORDER: usize = 12;
/// A single page is is 4,096 bytes
pub const PAGE_SIZE: usize = 1 << PAGE_ORDER;

#[repr(u8)]
enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,
}

pub enum SatpMode {
    Off = 0,
    Sv39 = 8,
    Sv48 = 9,
}

impl PageBits {
    fn val(self) -> u8 {
        match self {
            PageBits::Empty => 0,
            PageBits::Taken => 1 << 0,
            PageBits::Last => 1 << 1,
        }
    }
}

/// Align value to order
pub const fn align_val(val: usize, order: usize) -> usize {
    let o = (1usize << order) - 1;
    (val + o) & !o
}

/// RISC-V page representation
#[derive(Debug, Clone, Copy)]
struct Page {
    flags: u8,
}

/// Initialize the page system
impl Page {
    fn is_last(&self) -> bool {
        self.flags & PageBits::Last.val() != 0
    }

    fn is_taken(&self) -> bool {
        self.flags & PageBits::Taken.val() != 0
    }

    fn is_free(&self) -> bool {
        !self.is_taken()
    }

    fn clear(&mut self) {
        self.flags = PageBits::Empty.val();
    }

    fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;
    }

    //fn clear_flag(&mut self, flag: u8) {
    //    self.flags &= !flag;
    //}
}

/// Initialize the page system
fn page_init() {
    unsafe {
        let num_pages = crate::dbg!(HEAP_SIZE / PAGE_SIZE);
        crate::dbg!(num_pages);

        let ptr = HEAP_START as *mut Page;
        crate::dbg!(ptr);

        for i in 0..num_pages {
            (*ptr.add(i)).clear();
        }

        ALLOC_START = align_val(HEAP_START + num_pages * size_of::<Page>(), PAGE_ORDER);
        crate::dbg!(ALLOC_START);
    }
}

/// Initialize RISC-V related components
pub fn init() {
    page_init();
}

/// Get number of total allocated pages
// TODO: Refactor how pages are counted. Linked list maybe?
pub fn page_count() -> usize {
    unsafe {
        crate::dbg!(ALLOC_START);
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let mut beg = HEAP_START as *const Page;
        let end = beg.add(num_pages);

        let mut count = 0;
        while beg < end {
            if (*beg).is_taken() {
                loop {
                    count += 1;
                    if (*beg).is_last() {
                        break;
                    }
                    beg = beg.add(1);
                }
            }
            beg = beg.add(1);
        }
        count
    }
}

/// Get address of the last page
pub fn get_last_page() -> *mut u8 {
    unsafe {
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let mut beg = HEAP_START as *const Page;
        let end = beg.add(num_pages);

        while beg < end {
            if (*beg).is_taken() {
                loop {
                    if (*beg).is_last() {
                        return beg as *mut u8;
                    }
                    beg = beg.add(1);
                }
            }
            beg = beg.add(1);
        }
        beg as *mut u8
    }
}

/// Allocate N pages in RISC-V
///
/// * `pages`: the number of `PAGE_SIZE` pages to allocate
pub fn alloc(pages: usize) -> *mut u8 {
    unsafe {
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let ptr = HEAP_START as *mut Page;

        for i in 0..=num_pages.checked_sub(pages).unwrap() {
            let mut found = false;

            if (*ptr.add(i)).is_free() {
                found = true;
                for j in i..i + pages {
                    if (*ptr.add(j)).is_taken() {
                        found = false;
                        break;
                    }
                }
            }

            if found {
                for k in i..i + pages - 1 {
                    (*ptr.add(k)).set_flag(PageBits::Taken.val());
                }
                (*ptr.add(i + pages - 1)).set_flag(PageBits::Taken.val());
                (*ptr.add(i + pages - 1)).set_flag(PageBits::Last.val());

                return (ALLOC_START + PAGE_SIZE * i) as *mut u8;
            }
        }
    }

    null_mut()
}

/// Dellocates a page in RISC-V
///
/// * `page`: pointer to a page
pub fn dealloc(ptr: *mut u8) {
    // TODO: at the moment we cannot test this panic, since I haven't
    // figured out a solution to handle rust std' test 'should_panic'
    // in a custom test framework
    if ptr.is_null() {
        panic!("Null pointer")
    }
    unsafe {
        let addr = HEAP_START + (ptr as usize - ALLOC_START) / PAGE_SIZE;
        // we are looking for the page structure address
        let mut page = addr as *mut Page;
        if !(*page).is_taken() {
            panic!("Freeing a non-taken page.")
        }

        while (*page).is_taken() && !(*page).is_last() {
            (*page).clear();
            page = page.add(1);
        }

        crate::dbg!((*page).is_free());
        (*page).clear();
        crate::dbg!((*page).is_free());
    }
}

/// Zero allocate a page in RISC-V
///
/// * `pages`: number of pages to zero allocate
pub fn zalloc(pages: usize) -> *mut u8 {
    let ptr = alloc(pages);
    if !ptr.is_null() {
        // 4096 / 8 = 512
        let size = (PAGE_SIZE * pages) / 8;
        let big_ptr = ptr as *mut u64;
        // 8 byte pointers will cover a 4,096-byte page with only 512 loops
        for i in 0..size {
            unsafe {
                (*big_ptr.add(i)) = 0;
            }
        }
    }
    ptr
}

/// Print the currente page allocation status
pub fn print_page_allocation() {
    unsafe {
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let mut beginning = HEAP_START as *const Page;
        let end = beginning.add(num_pages);

        let alloc_start = ALLOC_START;
        let alloc_end = ALLOC_START + num_pages * PAGE_SIZE;

        println!("\n------------------------------------");
        println!(
            "Page allocation table:\nMeta: {:p} -> {:p}:\nPhys: 0x{:x} -> 0x{:x}",
            beginning, end, alloc_start, alloc_end
        );
        println!("------------------------------------");

        let mut num = 0;
        while beginning < end {
            if (*beginning).is_taken() {
                let start = beginning as usize;
                let memaddr = ALLOC_START + (start - HEAP_START) * PAGE_SIZE;
                crate::print!("0x{:x} => ", memaddr);
                loop {
                    num += 1;
                    if (*beginning).is_last() {
                        let end = beginning as usize;
                        let memaddr = ALLOC_START + (end - HEAP_START) * PAGE_SIZE + PAGE_SIZE - 1;
                        println!("0x{:x}: {:>3} page(s)", memaddr, (end - start + 1));
                        break;
                    }
                    beginning = beginning.add(1);
                }
            }
            beginning = beginning.add(1);
        }
        println!("Allocated: {} pages ({} bytes)", num, num * PAGE_SIZE);
        println!(
            "Free: {} pages ({} bytes)",
            num_pages - num,
            (num_pages - num) * PAGE_SIZE
        );

        println!("------------------------------------\n");
    }
}

/// RISC-V Sv39 Page entry bits representation
//See Sv39, chapter 4.5.1, figure 4.21 from The RISC-V privilege architecture
#[repr(usize)]
pub enum PageEntryBits {
    Dirty = 1 << 7,
    Access = 1 << 6,
    Global = 1 << 5,
    User = 1 << 4,
    Execute = 1 << 3,
    Write = 1 << 2,
    Read = 1 << 1,
    Valid = 1 << 0,

    ReadWrite = 1 << 1 | 1 << 2,
    ReadExecute = 1 << 1 | 1 << 3,
    ReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3,

    // User Convenience Combinations
    UserReadWrite = 1 << 1 | 1 << 2 | 1 << 4,
    UserReadExecute = 1 << 1 | 1 << 3 | 1 << 4,
    UserReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

impl PageEntryBits {
    pub fn val(self) -> usize {
        self as usize
    }
}

impl core::convert::From<crate::page::PageBits> for PageEntryBits {
    fn from(bits: crate::page::PageBits) -> Self {
        match bits {
            crate::page::PageBits::UserReadWrite | crate::page::PageBits::ReadWrite => {
                PageEntryBits::UserReadWrite
            }
            crate::page::PageBits::UserReadExecute | crate::page::PageBits::ReadExecute => {
                PageEntryBits::UserReadExecute
            }
            crate::page::PageBits::UserReadWriteExecute => PageEntryBits::UserReadWriteExecute,
        }
    }
}

pub struct PageEntry {
    pub entry: usize,
}

impl Entry<usize> for PageEntry {
    /// Check if entry is valid
    fn is_valid(&self) -> bool {
        self.get_entry() & PageEntryBits::Valid.val() != 0
    }

    /// Check if entry is invalid
    fn is_invalid(&self) -> bool {
        !self.is_valid() // the first bit is for valid
    }

    /// Check if entry is leaf
    fn is_leaf(&self) -> bool {
        self.get_entry() & 0xE != 0 // A leaf has one or more RWX bits set
    }

    /// Check if entry is a branch entry
    fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    /// Set aa entry
    fn set_entry(&mut self, entry: usize) {
        self.entry = entry;
    }

    /// Get the entry
    fn get_entry(&self) -> usize {
        self.entry
    }
}

// Sv39 contains three 9-bit indices
// 2^9 = 512 entries
const TABLE_ENTRIES: usize = 512;

pub struct Table {
    pub entries: [PageEntry; TABLE_ENTRIES],
}

impl KTable for Table {
    fn len(&self) -> usize {
        TABLE_ENTRIES
    }

    fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }
}

// TODO: Write tests
pub fn map(root: &mut Table, vaddr: usize, paddr: usize, bits: usize, level: usize) {
    let vpn = [
        // VPN[0] = vaddr[20:12]
        (vaddr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (vaddr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (vaddr >> 30) & 0x1ff,
    ];

    let ppn = [
        // PPN[0] = paddr[20:12]
        (paddr >> 12) & 0x1ff,
        // PPN[1] = paddr[29:21]
        (paddr >> 21) & 0x1ff,
        // PPN[2] = paddr[55:30]
        (paddr >> 30) & 0x3ff_ffff,
    ];

    // use this so we can set individual entries
    let mut refc = &mut root.entries[vpn[2]];

    for i in (level..2).rev() {
        if !refc.is_valid() {
            let page = zalloc(1);
            refc.set_entry((page as usize >> 2) | PageEntryBits::Valid.val());
        }

        let entry = ((refc.get_entry() & !0x3FF) << 2) as *mut PageEntry;
        refc = unsafe { entry.add(vpn[i]).as_mut().unwrap() };
    }

    let entry = (ppn[2] << 28) | // PPN[2] = [53:28]
                (ppn[1] << 19) | // PPN[1] = [27:19]
                (ppn[0] << 10) | // PPN[0] = [18:10]
                bits           | // specified bits, such as User, Read, Write, etc
                PageEntryBits::Valid.val() // Valid bit
                ;

    refc.set_entry(entry);
}

fn dealloc_level(root: &mut Table) {
    for level in 0..root.len() {
        let entry = &root.entries[level];
        if entry.is_valid() && entry.is_branch() {
            let memaddr_lv0 = (entry.get_entry() & !0x3ff) << 2;
            dealloc(memaddr_lv0 as *mut u8);
        }
    }
}

// TODO: Write tests
pub fn unmap(root: &mut Table) {
    // start at level 2
    for level2 in 0..root.len() {
        let entry = &root.entries[level2];

        if entry.is_valid() && entry.is_branch() {
            let memaddr_lv1 = (entry.get_entry() & !0x3FF) << 2;
            let table_lv1 = unsafe { (memaddr_lv1 as *mut Table).as_mut().unwrap() };

            dealloc_level(table_lv1);
            dealloc(memaddr_lv1 as *mut u8);
        }
    }
}

// TODO: Write tests
pub fn virt_to_phys(root: Table, vaddr: usize) -> Option<usize> {
    let vpn = [
        // VPN[0] = vaddr[20:12]
        (vaddr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (vaddr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (vaddr >> 30) & 0x1ff,
    ];

    let mut refc = &root.entries[vpn[2]];

    for i in (0..2).rev() {
        if refc.is_invalid() {
            break;
        } else if refc.is_leaf() {
            let off_mask = (1 << (12 + i * 9)) - 1;
            let vaddr_pgoff = vaddr & off_mask;
            let addr = ((refc.get_entry() << 2) as usize) & !off_mask;

            return Some(addr | vaddr_pgoff);
        }

        let entry = ((refc.get_entry() & !0x3ff) << 2) as *const PageEntry;
        refc = unsafe { entry.add(vpn[i - 1]).as_ref().unwrap() };
    }

    None
}

/// Identity map a range of address
// TODO: Write tests
pub fn ident_map_range(root: &mut Table, start: usize, end: usize, bits: usize) {
    let mut memaddr = crate::dbg!(start & !(PAGE_SIZE - 1));
    let num_kb_pages = (align_val(end, 12) - memaddr) / PAGE_SIZE;

    for _ in 0..num_kb_pages {
        map(root, memaddr, memaddr, bits, 0);
        memaddr += 1 << 12;
    }
}

pub fn flush_hw_cache(asid: usize) {
    unsafe {
        asm!("sfence.vma zero, {}", in(reg)(asid));
    }
}

pub const fn build_satp(mode: SatpMode, asid: usize, addr: usize) -> usize {
    (mode as usize) << 60 | (asid & 0xffff) << 44 | (addr >> 12) & 0xff_ffff_ffff
}
