#![no_std]
extern crate alloc;

use alloc::{vec, vec::Vec};
use core::result::Result::Ok;
use fdt::{Fdt, FdtError};

pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo, FdtError> {
    let fdt = unsafe {
        match Fdt::from_ptr(dtb_pa as *const u8) {
            Ok(fdt) => fdt,
            Err(_) => return Err(FdtError::BadPtr),
        } 
    };

    // memory area:
    let memory_addr = fdt
        .memory()
        .regions()
        .next()
        .unwrap()
        .starting_address as usize;
    let memory_size = fdt
        .memory()
        .regions()
        .next()
        .unwrap()
        .size
        .expect("ERROR: GEt memroy_size error!");

    let mut mmio_regions: Vec<(usize, usize)> = vec![];
    

    // virtio_mmio
    // in /soc/virtio_mmio
    for node in fdt.find_all_nodes("/soc/virtio_mmio") {
        if let Some(iter) = node.raw_reg() {
            mmio_regions.extend(iter.map(| reg | {
                (
                    // RISCV64 is little endian
                    usize::from_le_bytes(
                        reg.address
                            .try_into()
                            .unwrap_or_else(|_| [0; core::mem::size_of::<usize>()])
                    ),
                    usize::from_le_bytes(
                        reg.size
                            .try_into()
                            .unwrap_or_else(|_| [0; core::mem::size_of::<usize>()])    
                    ),
                )
            }))
        } 
    }
    Ok(DtbInfo { 
        memory_addr: memory_addr,
        memory_size: memory_size, 
        mmio_regions: mmio_regions, 
    })
}