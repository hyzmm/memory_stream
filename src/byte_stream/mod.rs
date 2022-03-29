pub mod output_byte_stream;
pub mod input_byte_stream;
mod swap_bytes;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

pub fn get_platform_endianness() -> Endianness {
    #[cfg(target_endian = "little")]
    {
        Endianness::LittleEndian
    }
    #[cfg(target_endian = "big")]
    {
        Endianness::BigEndian
    }
}