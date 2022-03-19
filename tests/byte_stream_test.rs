use memory_stream::byte_stream::input_byte_stream::InputByteStream;
use memory_stream::byte_stream::output_byte_stream::OutputByteStream;

#[test]
fn write_read_one_byte() {
    let mut o = OutputByteStream::new();
    o.write_bool(true);
    o.write_bool(false);
    o.write_u8(0u8);
    o.write_u8(255u8);
    o.write_i8(-2i8);
    assert_eq!(o.buffer(), [1, 0, 0, 255, 254]);

    let mut i = InputByteStream::new(o.buffer());
    assert_eq!(i.read_bool(), true);
    assert_eq!(i.read_bool(), false);
    assert_eq!(i.read_u8(), 0);
    assert_eq!(i.read_u8(), 255);
    assert_eq!(i.read_i8(), -2i8);
}

#[test]
fn write_read_two_bytes() {
    let mut o = OutputByteStream::new();
    o.write_u16(0u16);
    o.write_u16(100u16);
    o.write_u16(65535u16);
    o.write_i16(-2i16);
    assert_eq!(o.buffer(), [
        0u8, 0u8,
        0x64u8, 0u8,
        0xffu8, 0xffu8,
        0xfeu8, 0xffu8,
    ]);

    let mut i = InputByteStream::new(o.buffer());
    assert_eq!(i.read_u16(), 0);
    assert_eq!(i.read_u16(), 100);
    assert_eq!(i.read_u16(), 65535);
    assert_eq!(i.read_i16(), -2i16);
}


#[test]
fn write_read_four_bytes() {
    let mut o = OutputByteStream::new();
    o.write_u32(0u32);
    o.write_u32(100u32);
    o.write_u32(4294967295u32);
    o.write_i32(-2i32);
    assert_eq!(o.buffer(), [
        0u8, 0u8, 0u8, 0u8,
        0x64u8, 0u8, 0u8, 0u8,
        0xffu8, 0xffu8, 0xffu8, 0xffu8,
        0xfeu8, 0xffu8, 0xffu8, 0xffu8,
    ]);

    let mut i = InputByteStream::new(o.buffer());
    assert_eq!(i.read_u32(), 0);
    assert_eq!(i.read_u32(), 100);
    assert_eq!(i.read_u32(), 4294967295);
    assert_eq!(i.read_i32(), -2i32);
}

#[test]
fn write_read_f32() {
    let mut o = OutputByteStream::new();
    o.write_f32(1.234);
    o.write_f32(-1.234);
    o.write_f32(f32::MAX);
    let mut i = InputByteStream::new(o.buffer());
    assert_eq!(1.234, i.read_f32());
    assert_eq!(-1.234, i.read_f32());
    assert_eq!(f32::MAX, i.read_f32());
}

#[test]
fn write_read_string() {
    let mut o = OutputByteStream::new();
    o.write_string(&"hello rust!".to_string());
    let mut i = InputByteStream::new(o.buffer());
    assert_eq!("hello rust!", i.read_string().as_str());
}