use std::cmp::max;
use std::mem;
use std::ptr::copy_nonoverlapping;

pub struct OutputByteStream {
    buf: Vec<u8>,
    head: usize,
}

impl OutputByteStream {
    pub fn new() -> OutputByteStream {
        OutputByteStream {
            buf: vec![0; 1024],
            head: 0,
        }
    }

    fn write<T>(&mut self, data: &T) {
        let buf = &mut self.buf;
        let ptr = data as *const _ as *const u8;
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

    pub fn write_u16(&mut self, data: u16) { self.write(&data) }
    pub fn write_i16(&mut self, data: i16) { self.write(&data) }

    pub fn write_u32(&mut self, data: u32) { self.write(&data) }
    pub fn write_i32(&mut self, data: i32) { self.write(&data) }

    pub fn write_f32(&mut self, data: f32) { self.write(&data) }

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
