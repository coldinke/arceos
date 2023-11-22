#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22_000_000;

#[cfg_attr(feature = "axstd", no_mangle)]                
fn main() {
    // Big endian
    let apps_start = PLASH_START as *const u8;
    // 1 byte to store the size of app
    let size_byte = unsafe {
        core::slice::from_raw_parts(apps_start.offset(0 as isize), 8)
    };
    let apps_size = bytes_to_usize(size_byte);

    println!("Loading payload...");

    let code = unsafe {
        core::slice::from_raw_parts(apps_start.offset(8 as isize), apps_size)
    };

    println!("content: {:?}, size: {}", code, apps_size);
    println!("Load payload ok!");
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}