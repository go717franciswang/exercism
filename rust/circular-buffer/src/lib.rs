extern crate alloc;

#![stable(feature = "rust1", since = "1.0.0")]
use alloc::raw_vec::RawVec;
use std::ptr;

enum Error {
    EmptyBuffer
}

pub struct CircularBuffer<T> {
    begin: usize,
    len: usize,
    buf: RawVec<T>,
    cap: usize
}

impl<T> CircularBuffer<T> {
    pub fn new(cap: usize) -> Self {
        Self { begin: 0, len: 0, buf: RawVec::with_capacity(cap), cap: cap }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer)
        }
        let i = self.begin;
        self.begin += 1;
        self.begin %= self.cap;

        unsafe {
            Ok(ptr::read(self.buf.ptr().offset(i as isize)))
        }
    }
}
