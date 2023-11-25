#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::ptr;

#[cfg(feature = "axstd")]
use axstd::println;
use axhal::misc::terminate;

const PLASH_START: usize = 0x22_000_000;
const RUN_START: usize = 0xffff_ffc0_8010_0000;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_terminate() {
    println!("[ABI:Terminate]: Termiante");
    terminate();
}

#[cfg_attr(feature = "axstd", no_mangle)]                
fn main() {
    // Big endian
    let plash_start = PLASH_START as *const u8;
    let mut offset = 0isize;
    let apps_num = bytes_to_usize(
        unsafe {
            core::slice::from_raw_parts(plash_start.offset(offset), 8)
        }
    );
    offset += 8;

    // register the sbi instruction?
    // Add the number of SYSCALL to handle function
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

    println!("Loading payload...");
    for _i in 0..apps_num {
        let app = APPInfo::load(plash_start, offset);
        // println!("App: {}, size: {}, load code: {:?}, address :[{:?}]", i, app.app_size, app.app_data, app.app_data.as_ptr());
        offset = offset + 8 + (app.app_size as isize);

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app.app_size)
        };
        run_code.copy_from_slice(app.app_data);
        // println!("run code: {:?}, address [{:?}]", run_code, run_code.as_ptr());

        unsafe { core::arch::asm!("
            la      a0, {abi_table}
            li      t2, {run_start}
            jalr    t2",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
            clobber_abi("C")
        )}

        // // clear
        unsafe {
            ptr::write_bytes(run_code.as_mut_ptr(), 0, run_code.len())
        };
    }    
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}

struct APPInfo {
    app_size: usize,
    app_data: &'static [u8],
}

impl APPInfo {
    pub fn load(app_start: *const u8, offset: isize) -> Self {
        let app_size = bytes_to_usize(
            unsafe {
                core::slice::from_raw_parts(app_start.offset(offset), 8)
            }
        );
        // let load_size = ((app_size + 8) / 8) * 8 ; 
        let app_data = unsafe {
            core::slice::from_raw_parts(app_start.offset(offset + 8), app_size)
        };

        Self {
            app_size,
            app_data,
        }
    }
}
