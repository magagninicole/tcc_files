/*
Author: Ben Mezger (github.com/benmezger)
*/

/// Writes `value` to memory memory-mapped I/O at `address` in `offset`
///
/// # Safety
///
/// This function overwrites any value from `address`
pub unsafe fn mmio_write(address: *mut u8, offset: usize, value: u8) {
    address.add(offset).write_volatile(value);
}

/// Reads a value from memory memory-mapped I/O at `address` in `offset`
///
/// # Safety
///
/// This function reads the value from `address`
pub unsafe fn mmio_read(address: *mut u8, offset: usize) -> u8 {
    address.add(offset).read_volatile()
}
