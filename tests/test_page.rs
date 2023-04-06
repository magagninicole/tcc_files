#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(strail::test_runner)]
#![reexport_test_harness_main = "test_main"]

use strail::page::Entry;

#[cfg(test)]
#[no_mangle]
extern "C" fn main() {
    test_main();
    strail::exit_qemu_as_success();
}

use strail::arch::mem;

#[cfg(any(target_arch = "arch::isa::page64", target_arch = "riscv32"))]
#[test_case]
fn test_init_page() {
    mem::init();
    let last = mem::last_page();
    assert!(!last.is_null());
}

#[test_case]
fn test_alloc() {
    mem::init();
    assert_eq!(mem::page_count(), 0);

    let page_1 = mem::alloc(2);
    assert!(!page_1.is_null());
    assert_eq!(mem::page_count(), 2);
    assert!(!mem::last_page().is_null());

    let page_2 = mem::alloc(8);
    assert!(!page_2.is_null());
    assert_eq!(mem::page_count(), 10);
    assert!(!mem::last_page().is_null());

    mem::dealloc(page_1);
    mem::dealloc(page_2);
}

#[test_case]
fn test_dealloc() {
    assert_eq!(mem::page_count(), 0);

    let page = mem::alloc(2);
    assert!(!page.is_null());
    assert_eq!(mem::page_count(), 2);

    mem::dealloc(page);
    assert_eq!(mem::page_count(), 0);
}

#[test_case]
fn test_zalloc() {
    assert_eq!(mem::page_count(), 0);

    let page = mem::zalloc(2);
    assert!(!page.is_null());
    assert_eq!(mem::page_count(), 2);

    for _ in 0..(strail::arch::isa::page::PAGE_SIZE * 2) / 8 {
        unsafe {
            assert!(*page == 0);
        }
    }

    mem::dealloc(page);
    assert_eq!(mem::page_count(), 0);
}

#[cfg(any(target_arch = "arch::isa::page64", target_arch = "riscv32"))]
#[test_case]
fn test_init_page() {
    strail::arch::isa::page::init();
    let last = mem::last_page();
    assert!(!last.is_null());
}

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))] {

        #[test_case]
        fn test_page_entry_bits() {
            assert_eq!(strail::arch::isa::page::PageEntryBits::Dirty.val(), 128);
            assert_eq!(strail::arch::isa::page::PageEntryBits::Access.val(), 64);
            assert_eq!(strail::arch::isa::page::PageEntryBits::Global.val(), 32);
            assert_eq!(strail::arch::isa::page::PageEntryBits::User.val(), 16);
            assert_eq!(strail::arch::isa::page::PageEntryBits::Execute.val(), 8);
            assert_eq!(strail::arch::isa::page::PageEntryBits::Write.val(), 4);
            assert_eq!(strail::arch::isa::page::PageEntryBits::Read.val(), 2);
            assert_eq!(strail::arch::isa::page::PageEntryBits::Valid.val(), 1);
        }

        #[test_case]
        fn test_page_entry() {
            let mut entry = strail::arch::isa::page::PageEntry { entry: 1 };
            assert_eq!(entry.get_entry(), 1);
            assert!(entry.is_branch());
            assert!(!entry.is_leaf());
            assert!(entry.is_valid());
            assert!(!entry.is_invalid());

            entry.set_entry(2);
            assert_eq!(entry.get_entry(), 2);
        }
    }
}
