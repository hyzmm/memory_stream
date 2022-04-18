use std::cmp::max;
use std::mem::{size_of_val, transmute};
use std::ptr::addr_of;

pub struct OutputBitStream {
    pub buf: Vec<u8>,
    pub bit_head: usize,
}

impl Default for OutputBitStream {
    fn default() -> Self {
        OutputBitStream {
            buf: vec![0; 1024],
            bit_head: 0,
        }
    }
}

impl OutputBitStream {
    #[inline]
    fn byte_offset(&self) -> usize { self.bit_head >> 3 }
    #[inline]
    fn bit_offset(&self) -> usize { self.bit_head & 0x7 }

    pub fn buffer(&self) -> &[u8] {
        let mut head = self.byte_offset();
        if self.bit_offset() != 0 {
            head += 1;
        }
        &self.buf[0..head]
    }

    // 写入小于等于 8 位的数据
    fn write_byte(&mut self, data: u8, bit_count: usize) {
        assert!(bit_count <= 8);

        let next_byte_head = (self.bit_head + bit_count) >> 3;
        if next_byte_head > self.buf.len() {
            self.buf.resize(max(self.buf.len(), next_byte_head) * 2, 0);
        }

        // 计算字节偏移和位偏移
        let byte_offset = self.byte_offset();
        let bit_offset = self.bit_offset();

        // 写入数据和原有数据进行整合
        let current_mask = !(0xFF << bit_offset);
        self.buf[byte_offset] = (self.buf[byte_offset] & current_mask) | (data << bit_offset);

        let bits_free_this_byte = 8 - bit_offset;

        // 将当前字节无法存下的剩余数据写入到下一个字节
        if bits_free_this_byte < bit_count {
            self.buf[byte_offset + 1] = data >> bits_free_this_byte;
        }

        self.bit_head = self.bit_head + bit_count;
    }

    fn write_bytes(&mut self, data: *const u8, bit_count: usize) {
        let mut bit_count = bit_count;
        let mut offset = 0;
        while bit_count > 8 {
            unsafe { self.write_byte(*data.offset(offset), 8); }
            offset += 1;
            bit_count -= 8;
        }
        if bit_count > 0 {
            unsafe { self.write_byte(*data.offset(offset), bit_count); }
        }
    }

    fn write<T>(&mut self, obj: &T) {
        self.write_bytes(
            addr_of!(*obj) as *const u8,
            size_of_val(obj) * 8,
        );
    }

    pub fn write_bool(&mut self, value: bool) {
        self.write_byte(
            if value { 1 } else { 0 },
            1,
        );
    }

    pub fn write_u8(&mut self, value: u8) { self.write(&value) }
    pub fn write_i8(&mut self, value: i8) { self.write(&value) }

    pub fn write_u16(&mut self, value: u16) { self.write(&value.to_be()) }
    pub fn write_i16(&mut self, value: i16) { self.write(&value.to_be()) }

    pub fn write_u32(&mut self, value: u32) { self.write(&value.to_be()) }
    pub fn write_i32(&mut self, value: i32) { self.write(&value.to_be()) }

    pub fn write_u64(&mut self, value: u64) { self.write(&value.to_be()) }
    pub fn write_i64(&mut self, value: i64) { self.write(&value.to_be()) }

    pub fn write_f32(&mut self, value: f32) { self.write_u32(unsafe { transmute(value) }) }

    pub fn write_string(&mut self, data: &String) {
        self.write_u32(data.len() as u32);
        let bytes = data.as_bytes();
        for byte in bytes {
            self.write_u8(*byte);
        }
    }
}