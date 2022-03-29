pub fn swap_2_bytes(data: u16) -> u16 {
    data >> 8 | data << 8
}

pub fn swap_4_bytes(data: u32) -> u32 {
    data >> 24 & 0x0000_00FF |
        data >> 8 & 0x0000_FF00 |
        data << 8 & 0x00FF_0000 |
        data << 24 & 0xFF00_0000
}

pub fn swap_8_bytes(data: u64) -> u64 {
    data >> 56 & 0x0000_0000_0000_00FF |
        data >> 40 & 0x0000_0000_0000_FF00 |
        data >> 24 & 0x0000_0000_00FF_0000 |
        data >> 8 & 0x0000_0000_FF00_0000 |
        data << 8 & 0x0000_00FF_0000_0000 |
        data << 24 & 0x0000_FF00_0000_0000 |
        data << 40 & 0x00FF_0000_0000_0000 |
        data << 56 & 0xFF00_0000_0000_0000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_two_bytes() {
        let num = 10000u16;
        assert_eq!(swap_2_bytes(num), num.swap_bytes());
    }

    #[test]
    fn swap_four_bytes() {
        let num = 1234567890u32;
        assert_eq!(swap_4_bytes(num), num.swap_bytes());
    }

    #[test]
    fn swap_eight_bytes() {
        let num = 9000000006854776000u64;
        assert_eq!(swap_8_bytes(num), num.swap_bytes());
    }
}