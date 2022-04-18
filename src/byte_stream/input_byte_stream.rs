use std::mem::{size_of, transmute};

use crate::byte_stream::{Endianness, get_platform_endianness};
use crate::byte_stream::swap_bytes::*;

pub struct InputByteStream<'a> {
    buf: &'a [u8],
    head: usize,
    endianness: Endianness,
}

impl<'a> InputByteStream<'a> {
    pub fn new(buf: &'a [u8], endianness: Endianness) -> InputByteStream<'a> {
        InputByteStream {
            buf,
            head: 0,
            endianness,
        }
    }

    fn read<T>(&mut self) -> T {
        let num_bytes = size_of::<T>();
        let bytes = &self.buf[self.head..self.head + num_bytes];
        self.head += num_bytes;
        unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        }
    }

    pub fn read_u8(&mut self) -> u8 { self.read() }
    pub fn read_i8(&mut self) -> i8 { self.read() }

    pub fn read_bool(&mut self) -> bool {
        let byte = self.read_u8();
        if byte == 0 { false } else { true }
    }

    pub fn read_u16(&mut self) -> u16 {
        let data = self.read();
        if self.endianness != get_platform_endianness() {
            swap_2_bytes(data)
        } else {
            data
        }
    }
    pub fn read_i16(&mut self) -> i16 { self.read_u16() as i16 }

    pub fn read_u32(&mut self) -> u32 {
        let data = self.read();
        if self.endianness != get_platform_endianness() {
            swap_4_bytes(data)
        } else {
            data
        }
    }
    pub fn read_i32(&mut self) -> i32 { self.read_u32() as i32 }

    pub fn read_u64(&mut self) -> u64 {
        let data = self.read();
        if self.endianness != get_platform_endianness() {
            swap_8_bytes(data)
        } else {
            data
        }
    }
    pub fn read_i64(&mut self) -> i64 { self.read_u64() as i64 }

    pub fn read_f32(&mut self) -> f32 {
        unsafe { transmute(self.read_u32()) }
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_u32() as usize;
        let mut bytes = vec![0; len];
        for i in 0..len {
            bytes[i] = self.read_u8();
        }
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}