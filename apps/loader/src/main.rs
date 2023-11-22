#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22_000_000;

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

    println!("Loading payload...");
    (0..apps_num).for_each(|i| {
        let app = unsafe {
            APPInfo::load(plash_start, offset)
        };
        println!("App: {}, size: {}, byte content: {:?}, usize content: {:#x}", i, app.app_size, app.app_data, bytes_to_usize(app.app_data));
        offset = offset + 8 + (app.app_size as isize);
    });

    println!("Load payload ok!");
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
        let load_size = ((app_size + 8) / 8) * 8 ; 
        let app_data = unsafe {
            core::slice::from_raw_parts(app_start.offset(offset + 8), load_size)
        };

        Self {
            app_size,
            app_data,
        }
    }
}
