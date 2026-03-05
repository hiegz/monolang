use rug::Float;
use rug::Integer;

#[derive(Debug)]
pub enum NumericLiteral {
    Integer(Integer),
    Float(Float),

    Signed8(i8),
    Signed16(i16),
    Signed32(i32),
    Signed64(i64),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Float32(f32),
    Float64(f64),
}
