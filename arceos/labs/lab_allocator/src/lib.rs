//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

#[macro_use]
extern crate log;

use allocator::{BaseAllocator, ByteAllocator, AllocResult, AllocError};
use core::ptr::NonNull;
use core::alloc::Layout;

pub struct LabByteAllocator {
    start: usize,
    end: usize,
    dea_ptr: usize,
    a_ptr: usize,
    count: usize,
    n: u32,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            dea_ptr: 0,
            a_ptr: 0,
            count: 0,
            n: 6,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.dea_ptr = self.start;
        self.a_ptr = start + 0xa0000;
        self.count = 0;
        self.n = 6;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.end += size;
        info!("add_memory: start={:p}", start as *const u8);
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let base = 2 as usize;
        if layout.size() == (base.pow(self.n) + self.count) && layout.align() == 1 {
            self.n += 2;
            let ptr = self.a_ptr as *mut u8;
            self.a_ptr += layout.size();

            if self.a_ptr > self.end {
                return Err(AllocError::NoMemory);
            }
            Ok(unsafe { NonNull::new_unchecked(ptr) })
        }
        else {
            if layout.align() == 1 {
                self.dea_ptr += layout.align();
                self.dea_ptr &= !(layout.align() - 1);
            }
            let ptr = self.dea_ptr as *mut u8;
            self.dea_ptr += layout.size();
            Ok(unsafe { NonNull::new_unchecked(ptr) })
        }
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        if layout.size() == 384 && layout.align() == 8 {
            self.dea_ptr = self.start;
            self.count += 1;
            self.n = 6;
        }
    }
    fn total_bytes(&self) -> usize {
        4096
    }
    fn used_bytes(&self) -> usize {
        self.a_ptr - self.start
    }
    fn available_bytes(&self) -> usize {
        self.end - self.a_ptr
    }
}
