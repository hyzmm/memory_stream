use std::mem::size_of_val;
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

        let next_bit_head = self.bit_head + bit_count;
        if next_bit_head > self.buf.len() * 8 {
            self.buf.resize(self.buf.len() * 2, 0);
        }

        let byte_offset = self.byte_offset();
        let bit_offset = self.bit_offset();

        let current_mask = !(0xFF << bit_offset);
        self.buf[byte_offset] = (self.buf[byte_offset] & current_mask) | (data << bit_offset);

        let bits_free_this_byte = 8 - bit_offset;

        if bits_free_this_byte < bit_count {
            self.buf[byte_offset + 1] = data >> bits_free_this_byte;
        }

        self.bit_head = next_bit_head;
    }

    fn write_bits(&mut self, data: *const u8, bit_count: usize) {
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

    fn write_any<T: Sized>(&mut self, obj: &T) {
        self.write_bits(
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

    pub fn write_u8(&mut self, value: u8) { self.write_any(&value) }
    pub fn write_i8(&mut self, value: i8) { self.write_any(&value) }

    pub fn write_u16(&mut self, value: u16) { self.write_any(&value) }
    pub fn write_i16(&mut self, value: i16) { self.write_any(&value) }

    pub fn write_u32(&mut self, value: u32) { self.write_any(&value) }
    pub fn write_i32(&mut self, value: i32) { self.write_any(&value) }

    pub fn write_u64(&mut self, value: u64) { self.write_any(&value) }
    pub fn write_i64(&mut self, value: i64) { self.write_any(&value) }

    pub fn write_f32(&mut self, value: f32) { self.write_any(&value) }

    pub fn write_string(&mut self, data: &String) {
        self.write_u32(data.len() as u32);
        let bytes = data.as_bytes();
        for byte in bytes {
            self.write_u8(*byte);
        }
    }
}