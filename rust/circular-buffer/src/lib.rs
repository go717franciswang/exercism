#![feature(unique)]
#![feature(alloc, heap_api)]

extern crate alloc;

use std::ptr::{Unique, self};
use std::mem;
use alloc::heap;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer
}

pub struct CircularBuffer<T> {
    begin: usize,
    cap: usize,
    len: usize,
    ptr: Unique<T>
}

fn oom() {
    ::std::process::exit(-9999);
}

impl<T> CircularBuffer<T> {
    pub fn new(cap: usize) -> Self {
        unsafe {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            let ptr = heap::allocate(elem_size*cap, align);
            if ptr.is_null() { oom(); }

            Self { begin: 0, cap: cap, len: 0, ptr: Unique::new(ptr as *mut _) }
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer)
        }
        let old_begin = self.begin;
        self.begin += 1;
        self.begin %= self.cap;
        self.len -= 1;

        unsafe {
            Ok(ptr::read(self.ptr.as_ptr().offset(old_begin as isize)))
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), Error> {
        if self.len == self.cap {
            return Err(Error::FullBuffer)
        }
        unsafe {
            let offset = (self.begin + self.len) % self.cap;
            ptr::write(self.ptr.as_ptr().offset(offset as isize), item);
            self.len += 1;
            Ok(())
        }
    }

    pub fn overwrite(&mut self, item: T) {
        unsafe {
            let offset = (self.begin + self.len) % self.cap;
            ptr::write(self.ptr.as_ptr().offset(offset as isize), item);
            if self.len == self.cap {
                self.begin = (self.begin + 1) % self.cap;
            } else {
                self.len += 1;
            }
        }
    }

    pub fn clear(&mut self) {
        while self.len > 0 {
            self.read().unwrap();
        }
    }
}

impl<T> Drop for CircularBuffer<T> {
    fn drop(&mut self) {
        self.clear();
        let align = mem::align_of::<T>();
        let elem_size = mem::size_of::<T>();
        unsafe {
            heap::deallocate(self.ptr.as_ptr() as *mut _, elem_size*self.cap, align);
        }
    }
}
