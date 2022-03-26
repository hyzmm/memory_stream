use std::mem::size_of;

pub fn swap_byte<T>(data: &T) -> *const u8 {
    let size = size_of::<T>();
    let ptr = data as *const _;
    unsafe {
        match size {
            2 => swap_2_bytes(std::ptr::read(ptr as *const u16)) as *const u8,
            4 => swap_4_bytes(std::ptr::read(ptr as *const u32)) as *const u8,
            8 => swap_8_bytes(std::ptr::read(ptr as *const u64)) as *const u8,
            _ => ptr as *const u8,
        }
    }
}

fn swap_2_bytes(data: u16) -> u16 {
    data >> 8 | data << 8
}

fn swap_4_bytes(data: u32) -> *const u8 {
    (data >> 24 & 0x0000_00FF |
        data >> 8 & 0x0000_FF00 |
        data << 8 & 0x00FF_0000 |
        data << 24 & 0xFF00_0000
    ) as *const u8
}

fn swap_8_bytes(data: u64) -> *const u8 {
    (data >> 56 & 0x0000_0000_0000_00FF |
        data >> 40 & 0x0000_0000_0000_FF00 |
        data >> 24 & 0x0000_0000_00FF_0000 |
        data >> 8 & 0x0000_0000_FF00_0000 |
        data << 8 & 0x0000_00FF_0000_0000 |
        data << 24 & 0x0000_FF00_0000_0000 |
        data << 40 & 0x00FF_0000_0000_0000 |
        data << 56 & 0xFF00_0000_0000_0000) as *const u8
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use crate::byte_stream::swap_bytes::swap_byte;

    #[test]
    fn swap_one_bytes() {
        let num = 127u8;
        unsafe {
            assert_eq!(swap_byte(&num).read(), num);
        }
    }

    #[test]
    fn swap_two_bytes() {
        let num = 10000u16;
        unsafe {
            let a = &num as *const _ as *const u16;
            let b = (num.swap_bytes() as *const u16);

            println!("{}", *a);
            // assert_ne!(num, num.swap_bytes());
            // assert_eq!(swap_byte(&num).to_bits() as u16, num.swap_bytes());
        }
    }

    #[test]
    fn swap_four_bytes() {
        let num = 1234567890u32;
        unsafe {
            assert_ne!(num, num.swap_bytes());
            assert_eq!(swap_byte(&num).to_bits() as u32, num.swap_bytes());
        }
    }
}