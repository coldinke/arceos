#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

const PLASH_START: usize = 0x22_000_000;

#[cfg_attr(feature = "axstd", no_mangle)]                
fn main() {
    let load_start = PLASH_START as *const u8;
    let load_size = 32; // Dangerous!!! We need to get accurate size of apps.

    println!("Loading payload...");

    let load_code = unsafe {
        core::slice::from_raw_parts(load_start, load_size)
    };
    println!("load code {:?}; address [{:?}]", load_code, load_code.as_ptr());

    const RUN_START: usize = 0xffff_ffc0_8010_0000;
    let run_code = unsafe {
        core::slice::from_raw_parts_mut(RUN_START as *mut u8, load_size)
    };
    run_code.copy_from_slice(load_code);
    println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

    // println!("content: {:#x}", bytes_to_usize(&code[..8]));
    println!("Load payload ok!");

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);

    println!("Excute app ...");
    let arg0: u8 = b'A';

    // execute app
    unsafe { core::arch::asm!("
        la      a7, {abi_table}
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
    )}
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}