pub struct InputByteStream<'a> {
    buf: &'a [u8],
    head: usize,
}

impl<'a> InputByteStream<'a> {
    pub fn new(buf: &'a [u8]) -> InputByteStream<'a> {
        InputByteStream {
            buf,
            head: 0,
        }
    }

    fn read_bytes<T: Sized>(&mut self, num_bytes: usize) -> T {
        let bytes = &self.buf[self.head..self.head + num_bytes];
        self.head += num_bytes;
        unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        }
    }

    pub fn read_u8(&mut self) -> u8 { self.read_bytes(1) }
    pub fn read_i8(&mut self) -> i8 { self.read_bytes(1) }

    pub fn read_bool(&mut self) -> bool {
        let byte = self.read_u8();
        if byte == 0 { false } else { true }
    }

    pub fn read_u16(&mut self) -> u16 { self.read_bytes(2) }

    pub fn read_i16(&mut self) -> i16 { self.read_bytes(2) }
    pub fn read_u32(&mut self) -> u32 { self.read_bytes(4) }

    pub fn read_i32(&mut self) -> i32 { self.read_bytes(4) }

    pub fn read_f32(&mut self) -> f32 { self.read_bytes(4) }

    pub fn read_string(&mut self) -> String {
        let len = self.read_u32() as usize;
        let mut bytes: Vec<u8> = vec![0; len];
        for i in 0..len {
            bytes[i] = self.read_u8();
        }
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}