use std::cmp::max;
use std::mem;
use std::mem::transmute;
use std::ptr::copy_nonoverlapping;

use crate::byte_stream::{Endianness, get_platform_endianness};
use crate::byte_stream::swap_bytes::*;

pub struct OutputByteStream {
    pub buf: Vec<u8>,
    pub head: usize,
    pub endianness: Endianness,
}

impl Default for OutputByteStream {
    fn default() -> Self {
        OutputByteStream {
            buf: vec![0; 1024],
            head: 0,
            endianness: Endianness::BigEndian,
        }
    }
}

impl OutputByteStream {
    fn write<T>(&mut self, data: &T) {
        let buf = &mut self.buf;
        let num_bytes = mem::size_of::<T>();

        if self.head + num_bytes > buf.len() {
            buf.resize(max(buf.len() * 2, self.head + num_bytes), 0);
        }

        unsafe {
            copy_nonoverlapping(
                data as *const _ as *const u8,
                self.buf[self.head..].as_mut_ptr(),
                num_bytes,
            );
        }
        self.head += num_bytes;
    }

    pub fn write_u8(&mut self, data: u8) { self.write(&data) }
    pub fn write_i8(&mut self, data: i8) { self.write(&data) }

    pub fn write_bool(&mut self, data: bool) { self.write(&data) }

    pub fn write_u16(&mut self, data: u16) {
        let mut data = data;
        if self.endianness != get_platform_endianness() {
            data = swap_2_bytes(data);
        }
        self.write(&data)
    }
    pub fn write_i16(&mut self, data: i16) { self.write_u16(data as u16) }

    pub fn write_u32(&mut self, data: u32) {
        let mut data = data;
        if self.endianness != get_platform_endianness() {
            data = swap_4_bytes(data);
        }
        self.write(&data)
    }
    pub fn write_i32(&mut self, data: i32) { self.write_u32(data as u32) }

    pub fn write_u64(&mut self, data: u64) {
        let mut data = data;
        if self.endianness != get_platform_endianness() {
            data = swap_8_bytes(data);
        }
        self.write(&data)
    }

    pub fn write_i64(&mut self, data: i64) { self.write_u64(data as u64) }

    pub fn write_f32(&mut self, data: f32) {
        unsafe { self.write_u32(transmute(data)) }
    }

    pub fn write_string(&mut self, data: &String) {
        self.write_u32(data.len() as u32);
        let bytes = data.as_bytes();
        for byte in bytes {
            self.write_u8(*byte);
        }
    }

    // pub fn write_vec(&mut self, data: Vec<T>) {
    //     self.write_u32(data.len() as u32);
    //
    // }

    pub fn buffer(&self) -> &[u8] { &self.buf[0..self.head] }
}
