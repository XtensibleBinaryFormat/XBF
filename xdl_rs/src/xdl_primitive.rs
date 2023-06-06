use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{self, Write};

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum XdlPrimitiveId {
    U8 = 0,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Bool,
}

impl From<u8> for XdlPrimitiveId {
    fn from(x: u8) -> Self {
        match x {
            0 => XdlPrimitiveId::U8,
            1 => XdlPrimitiveId::U16,
            2 => XdlPrimitiveId::U32,
            3 => XdlPrimitiveId::U64,
            4 => XdlPrimitiveId::I8,
            5 => XdlPrimitiveId::I16,
            6 => XdlPrimitiveId::I32,
            7 => XdlPrimitiveId::I64,
            8 => XdlPrimitiveId::F32,
            9 => XdlPrimitiveId::F64,
            10 => XdlPrimitiveId::String,
            11 => XdlPrimitiveId::Bool,
            _ => panic!("invalid primitive id {x}"),
        }
    }
}

pub struct XdlPrimitiveMetadata(XdlPrimitiveId);

#[derive(Debug, Clone)]
pub enum XdlPrimitive {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Bool(bool),
}

impl XdlPrimitive {
    pub fn serialize_with_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XdlPrimitive::U8(x) => writer.write_u8(*x),
            XdlPrimitive::U16(x) => writer.write_u16::<NetworkEndian>(*x),
            XdlPrimitive::U32(x) => writer.write_u32::<NetworkEndian>(*x),
            XdlPrimitive::U64(x) => writer.write_u64::<NetworkEndian>(*x),
            XdlPrimitive::I8(x) => writer.write_i8(*x),
            XdlPrimitive::I16(x) => writer.write_i16::<NetworkEndian>(*x),
            XdlPrimitive::I32(x) => writer.write_i32::<NetworkEndian>(*x),
            XdlPrimitive::I64(x) => writer.write_i64::<NetworkEndian>(*x),
            XdlPrimitive::F32(x) => writer.write_f32::<NetworkEndian>(*x),
            XdlPrimitive::F64(x) => writer.write_f64::<NetworkEndian>(*x),
            XdlPrimitive::String(x) => {
                writer.write_u32::<NetworkEndian>(x.len() as u32)?;
                writer.write_all(x.as_bytes())
            }
            XdlPrimitive::Bool(x) => writer.write_u8(u8::from(*x)),
        }
    }
}

macro_rules! xdl_primitive_from_impl {
    ($ty:ty, $xdl_type:tt) => {
        impl From<$ty> for XdlPrimitive {
            fn from(x: $ty) -> Self {
                XdlPrimitive::$xdl_type(x)
            }
        }
    };
}

xdl_primitive_from_impl!(u8, U8);
xdl_primitive_from_impl!(u16, U16);
xdl_primitive_from_impl!(u32, U32);
xdl_primitive_from_impl!(u64, U64);
xdl_primitive_from_impl!(i8, I8);
xdl_primitive_from_impl!(i16, I16);
xdl_primitive_from_impl!(i32, I32);
xdl_primitive_from_impl!(i64, I64);
xdl_primitive_from_impl!(f32, F32);
xdl_primitive_from_impl!(f64, F64);
xdl_primitive_from_impl!(String, String);
xdl_primitive_from_impl!(bool, Bool);
