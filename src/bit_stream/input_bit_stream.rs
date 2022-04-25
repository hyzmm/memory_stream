use std::mem::{size_of, transmute};

use crate::byte_stream::Endianness;

macro_rules! read_be {
    ( $self: ident, $t:ty ) => {{
        const SIZE: usize = size_of::<$t>();
        let bytes = $self.read_bytes(SIZE);
        let ptr = bytes.as_ptr() as *const [u8; SIZE];
        if $self.endianness == Endianness::BigEndian {
            <$t>::from_be_bytes(unsafe { ptr.read() })
        } else {
            <$t>::from_le_bytes(unsafe { ptr.read() })
        }
    }};
}

pub struct InputBitStream<'a> {
    pub buf: &'a [u8],
    pub bit_head: usize,
    pub endianness: Endianness,
}

impl<'a> Default for InputBitStream<'a> {
    fn default() -> Self {
        Self {
            buf: &[],
            bit_head: 0,
            endianness: Endianness::BigEndian,
        }
    }
}

impl<'a> InputBitStream<'a> {
    #[inline]
    fn byte_offset(&self) -> usize {
        self.bit_head >> 3
    }
    #[inline]
    fn bit_offset(&self) -> usize {
        self.bit_head & 0x7
    }

    pub fn new(buf: &'a [u8]) -> InputBitStream<'a> {
        InputBitStream {
            buf,
            bit_head: 0,
            endianness: Endianness::BigEndian,
        }
    }
    // 读取最多一个字节，允许读取 <= 8 数据。如果当前字节剩余位数不足，和下一个字节组合成一个 u8
    fn read_byte(&mut self, bit_count: usize) -> u8 {
        // 计算字节偏移和位偏移
        let byte_offset = self.byte_offset();
        let bit_offset = self.bit_offset();

        // 左侧 8 - bit_offset 位数据
        let mut out_data = self.buf[byte_offset] >> bit_offset;

        let bits_free_this_byte = 8 - bit_offset;
        if bits_free_this_byte < bit_count {
            out_data |= self.buf[byte_offset + 1] << bits_free_this_byte;
        }
        out_data &= !(0xffu16 << bit_count) as u8;
        self.bit_head += bit_count;
        out_data
    }

    fn read_bytes(&mut self, byte_count: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; byte_count];
        for i in 0..byte_count {
            bytes[i] = self.read_byte(8);
        }
        bytes
    }

    fn read<T>(&mut self) -> T {
        unsafe {
            let bytes = self.read_bytes(size_of::<T>());
            std::ptr::read(bytes.as_ptr() as *const _)
        }
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_byte(1) == 1
    }

    pub fn read_u8(&mut self) -> u8 {
        self.read()
    }
    pub fn read_i8(&mut self) -> i8 {
        self.read()
    }

    pub fn read_u16(&mut self) -> u16 {
        read_be!(self, u16)
    }
    pub fn read_i16(&mut self) -> i16 {
        read_be!(self, i16)
    }

    pub fn read_u32(&mut self) -> u32 {
        read_be!(self, u32)
    }
    pub fn read_i32(&mut self) -> i32 {
        read_be!(self, i32)
    }

    pub fn read_u64(&mut self) -> u64 {
        read_be!(self, u64)
    }
    pub fn read_i64(&mut self) -> i64 {
        read_be!(self, i64)
    }

    pub fn read_f32(&mut self) -> f32 {
        unsafe { transmute(self.read_u32()) }
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_u32() as usize;
        let bytes = self.read_bytes(len);
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}
