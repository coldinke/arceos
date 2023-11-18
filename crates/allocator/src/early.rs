//! Eearly memory allocation
//! 
//! TODO: unknow

use super::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;


/// Earlyallocator
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    page_pos: usize,
    byte_pos: usize,
    page_alloc_times: usize,
    byte_alloc_times: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Create a new empty `EarlyAllocator`
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            page_pos: 0,
            byte_pos: 0,
            page_alloc_times: 0,
            byte_alloc_times: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        assert!(PAGE_SIZE.is_power_of_two());
        self.start = start;
        self.end = start + size;
        self.byte_pos = self.start;
        self.page_pos = self.end;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory)
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.available_bytes() > layout.size() {
            let ptr = NonNull::new(self.byte_pos as *mut u8)
                .ok_or(AllocError::NoMemory)?;
            self.byte_pos += layout.size();
            self.byte_alloc_times += layout.size();
            Ok(ptr)
        } else {
            Err(AllocError::NoMemory)
        }
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, layout: Layout) {
        self.byte_alloc_times -= layout.size();
        if self.byte_alloc_times == 0 {
            self.byte_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.byte_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        if self.available_pages() > num_pages {
            self.page_alloc_times += num_pages;
            self.page_pos -= PAGE_SIZE * num_pages;
            Ok(self.page_pos)
        } else {
            Err(AllocError::NoMemory)
        }
    }

    fn dealloc_pages(&mut self, _pos: usize, num_pages: usize) {
        self.page_alloc_times -= num_pages;
        if self.page_alloc_times == 0 {
            self.page_pos = self.end;
        }
    }

    fn total_pages(&self) -> usize {
        (self.end - self.start) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.page_pos) / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos) / PAGE_SIZE
    }
}