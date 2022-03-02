use std::cmp::max;
use std::mem;

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

        for i in 0..num_bytes {
            self.buf[self.head + i] = unsafe { *ptr.offset(i as isize) };
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

    pub fn buffer(&self) -> &[u8] { &self.buf[0..self.head] }
}
