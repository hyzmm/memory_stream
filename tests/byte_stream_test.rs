use memory_stream::byte_stream::Endianness;
use memory_stream::byte_stream::input_byte_stream::InputByteStream;
use memory_stream::byte_stream::output_byte_stream::OutputByteStream;

#[test]
fn write_read_one_byte() {
    let mut o = OutputByteStream::default();
    o.write_bool(true);
    o.write_bool(false);
    o.write_u8(0u8);
    o.write_u8(255u8);
    o.write_i8(-2i8);
    assert_eq!(o.buffer(), [1, 0, 0, 255, 254]);

    let mut i = InputByteStream::new(o.buffer(), Endianness::BigEndian);
    assert_eq!(i.read_bool(), true);
    assert_eq!(i.read_bool(), false);
    assert_eq!(i.read_u8(), 0);
    assert_eq!(i.read_u8(), 255);
    assert_eq!(i.read_i8(), -2i8);
}

#[test]
fn write_read_two_bytes() {
    let mut o = OutputByteStream::default();
    o.write_u16(0u16);
    o.write_u16(100u16);
    o.write_u16(65535u16);
    o.write_i16(-2i16);

    let mut i = InputByteStream::new(o.buffer(), Endianness::BigEndian);
    assert_eq!(i.read_u16(), 0);
    assert_eq!(i.read_u16(), 100);
    assert_eq!(i.read_u16(), 65535);
    assert_eq!(i.read_i16(), -2i16);
}


#[test]
fn write_read_four_bytes() {
    let mut o = OutputByteStream::default();
    o.write_u32(0u32);
    o.write_u32(100u32);
    o.write_u32(4294967295u32);
    o.write_i32(-2i32);

    let mut i = InputByteStream::new(o.buffer(), Endianness::BigEndian);
    assert_eq!(i.read_u32(), 0);
    assert_eq!(i.read_u32(), 100);
    assert_eq!(i.read_u32(), 4294967295);
    assert_eq!(i.read_i32(), -2i32);
}

#[test]
fn write_read_eight_bytes() {
    let mut o = OutputByteStream::default();
    o.write_u64(0u64);
    o.write_u64(100u64);
    o.write_u64(17446744073709552000);
    o.write_i64(-2i64);

    let mut i = InputByteStream::new(o.buffer(), Endianness::BigEndian);
    assert_eq!(i.read_u64(), 0);
    assert_eq!(i.read_u64(), 100);
    assert_eq!(i.read_u64(), 17446744073709552000);
    assert_eq!(i.read_i64(), -2i64);
}

#[test]
fn write_read_f32() {
    let mut o = OutputByteStream {
        ..Default::default()
    };
    o.write_f32(1.234);
    o.write_f32(-1.234);
    o.write_f32(f32::MAX);
    let mut i = InputByteStream::new(o.buffer(), Endianness::BigEndian);
    assert_eq!(1.234, i.read_f32());
    assert_eq!(-1.234, i.read_f32());
    assert_eq!(f32::MAX, i.read_f32());
}

#[test]
fn write_read_string() {
    let mut o = OutputByteStream::default();
    o.write_string(&"hello world!".to_string());
    let mut i = InputByteStream::new(o.buffer(), Endianness::BigEndian);
    assert_eq!("hello world!", i.read_string().as_str());
}

#[test]
fn write_read_all() {
    fn t(endianness: Endianness) {
        let mut o = OutputByteStream {
            endianness,
            ..Default::default()
        };
        o.write_bool(true);
        o.write_i8(127);
        o.write_i16(30000);
        o.write_i32(65536);
        o.write_i64(-5611626018427388000);
        o.write_f32(123.456);
        o.write_string(&"hello world!".to_string());

        let mut i = InputByteStream::new(o.buffer(), endianness);
        assert_eq!(true, i.read_bool());
        assert_eq!(127, i.read_i8());
        assert_eq!(30000, i.read_i16());
        assert_eq!(65536, i.read_i32());
        assert_eq!(-5611626018427388000, i.read_i64());
        assert_eq!(123.456, i.read_f32());
        assert_eq!("hello world!", i.read_string().as_str());
    }

    t(Endianness::LittleEndian);
    t(Endianness::BigEndian);
}

#[test]
fn write_in_be_read_in_le() {
    let mut o = OutputByteStream::default();
    o.write_u8(120);
    o.write_u16(10000);

    let mut i = InputByteStream::new(o.buffer(), Endianness::LittleEndian);
    assert_eq!(i.read_u8(), 120);
    assert_ne!(i.read_u16(), 10000);
}