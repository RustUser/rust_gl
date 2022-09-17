#[derive(Debug, Clone, Copy)]
pub struct Float(f32);
#[derive(Debug, Clone, Copy)]
pub struct Double(f64);

#[derive(Debug, Clone, Copy)]
pub struct Byte(i8);
#[derive(Debug, Clone, Copy)]
pub struct Short(i16);
#[derive(Debug, Clone, Copy)]
pub struct Int(i32);
#[derive(Debug, Clone, Copy)]
pub struct Long(i64);

#[derive(Debug, Clone, Copy)]
pub struct UByte(u8);
#[derive(Debug, Clone, Copy)]
pub struct UShort(u16);
#[derive(Debug, Clone, Copy)]
pub struct UInt(u32);
#[derive(Debug, Clone, Copy)]
pub struct ULong(u64);

#[derive(Debug, Clone, Copy)]
pub struct Bool(bool);