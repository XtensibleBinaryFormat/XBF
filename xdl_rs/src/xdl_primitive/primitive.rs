use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

use crate::{DeserializeType, Serialize, XdlMetadata, XdlType};

use super::primitive_metadata::XdlPrimitiveMetadata;

#[derive(Debug, Clone, PartialEq)]
pub enum XdlPrimitive {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(()),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    I256(()),
    F32(f32),
    F64(f64),
    String(String),
}

impl From<XdlPrimitive> for XdlType {
    fn from(value: XdlPrimitive) -> Self {
        XdlType::Primitive(value)
    }
}

impl Serialize for XdlPrimitive {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XdlPrimitive::Bool(x) => writer.write_u8(u8::from(*x)),

            XdlPrimitive::U8(x) => writer.write_u8(*x),
            XdlPrimitive::U16(x) => writer.write_u16::<LittleEndian>(*x),
            XdlPrimitive::U32(x) => writer.write_u32::<LittleEndian>(*x),
            XdlPrimitive::U64(x) => writer.write_u64::<LittleEndian>(*x),
            XdlPrimitive::U128(x) => writer.write_u128::<LittleEndian>(*x),
            XdlPrimitive::U256(_) => unimplemented!(),

            XdlPrimitive::I8(x) => writer.write_i8(*x),
            XdlPrimitive::I16(x) => writer.write_i16::<LittleEndian>(*x),
            XdlPrimitive::I32(x) => writer.write_i32::<LittleEndian>(*x),
            XdlPrimitive::I64(x) => writer.write_i64::<LittleEndian>(*x),
            XdlPrimitive::I128(x) => writer.write_i128::<LittleEndian>(*x),
            XdlPrimitive::I256(_) => unimplemented!(),

            XdlPrimitive::F32(x) => writer.write_f32::<LittleEndian>(*x),
            XdlPrimitive::F64(x) => writer.write_f64::<LittleEndian>(*x),

            XdlPrimitive::String(x) => {
                writer.write_u16::<LittleEndian>(x.len() as u16)?;
                writer.write_all(x.as_bytes())
            }
        }
    }
}

impl DeserializeType for XdlPrimitive {
    fn deserialize(reader: &mut impl Read) -> io::Result<(XdlMetadata, XdlType)> {
        let type_to_deserialize = XdlMetadata::Primitive(reader.read_u8()?.into());
        Self::deserialize_with_metadata(&type_to_deserialize, reader)
            .map(|value| (type_to_deserialize, value))
    }

    fn deserialize_with_metadata(
        metadata: &XdlMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XdlType> {
        match metadata {
            XdlMetadata::Primitive(x) => match x {
                XdlPrimitiveMetadata::Bool => reader.read_u8().map(|x| XdlPrimitive::Bool(x != 0)),
                XdlPrimitiveMetadata::U8 => reader.read_u8().map(XdlPrimitive::U8),
                XdlPrimitiveMetadata::U16 => {
                    reader.read_u16::<LittleEndian>().map(XdlPrimitive::U16)
                }
                XdlPrimitiveMetadata::U32 => {
                    reader.read_u32::<LittleEndian>().map(XdlPrimitive::U32)
                }
                XdlPrimitiveMetadata::U64 => {
                    reader.read_u64::<LittleEndian>().map(XdlPrimitive::U64)
                }
                XdlPrimitiveMetadata::U128 => {
                    reader.read_u128::<LittleEndian>().map(XdlPrimitive::U128)
                }
                XdlPrimitiveMetadata::U256 => unimplemented!(),
                XdlPrimitiveMetadata::I8 => reader.read_i8().map(XdlPrimitive::I8),
                XdlPrimitiveMetadata::I16 => {
                    reader.read_i16::<LittleEndian>().map(XdlPrimitive::I16)
                }
                XdlPrimitiveMetadata::I32 => {
                    reader.read_i32::<LittleEndian>().map(XdlPrimitive::I32)
                }
                XdlPrimitiveMetadata::I64 => {
                    reader.read_i64::<LittleEndian>().map(XdlPrimitive::I64)
                }
                XdlPrimitiveMetadata::I128 => {
                    reader.read_i128::<LittleEndian>().map(XdlPrimitive::I128)
                }
                XdlPrimitiveMetadata::I256 => unimplemented!(),
                XdlPrimitiveMetadata::F32 => {
                    reader.read_f32::<LittleEndian>().map(XdlPrimitive::F32)
                }
                XdlPrimitiveMetadata::F64 => {
                    reader.read_f64::<LittleEndian>().map(XdlPrimitive::F64)
                }
                XdlPrimitiveMetadata::String => {
                    let len = reader.read_u16::<LittleEndian>()?;
                    let mut buf = vec![0; len as usize];
                    reader.read_exact(&mut buf)?;
                    let string = String::from_utf8(buf)
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))?;
                    Ok(XdlPrimitive::String(string))
                }
            }
            .map(|x| x.into()),
            _ => todo!(),
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

xdl_primitive_from_impl!(bool, Bool);

xdl_primitive_from_impl!(u8, U8);
xdl_primitive_from_impl!(u16, U16);
xdl_primitive_from_impl!(u32, U32);
xdl_primitive_from_impl!(u64, U64);
xdl_primitive_from_impl!(u128, U128);

xdl_primitive_from_impl!(i8, I8);
xdl_primitive_from_impl!(i16, I16);
xdl_primitive_from_impl!(i32, I32);
xdl_primitive_from_impl!(i64, I64);
xdl_primitive_from_impl!(i128, I128);

xdl_primitive_from_impl!(f32, F32);
xdl_primitive_from_impl!(f64, F64);

xdl_primitive_from_impl!(String, String);
