use super::primitive_metadata::XdlPrimitiveMetadata;
use crate::{
    util::{read_string, write_string},
    Serialize, XdlType,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum XdlPrimitive {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256([u64; 4]),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    I256([u64; 4]),
    F32(f32),
    F64(f64),
    String(String),
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
            XdlPrimitive::U256(x) => x
                .iter()
                .try_for_each(|x| writer.write_u64::<LittleEndian>(*x)),

            XdlPrimitive::I8(x) => writer.write_i8(*x),
            XdlPrimitive::I16(x) => writer.write_i16::<LittleEndian>(*x),
            XdlPrimitive::I32(x) => writer.write_i32::<LittleEndian>(*x),
            XdlPrimitive::I64(x) => writer.write_i64::<LittleEndian>(*x),
            XdlPrimitive::I128(x) => writer.write_i128::<LittleEndian>(*x),
            XdlPrimitive::I256(x) => x
                .iter()
                .try_for_each(|x| writer.write_u64::<LittleEndian>(*x)),

            XdlPrimitive::F32(x) => writer.write_f32::<LittleEndian>(*x),
            XdlPrimitive::F64(x) => writer.write_f64::<LittleEndian>(*x),

            XdlPrimitive::String(x) => write_string(x, writer),
        }
    }
}

impl XdlPrimitive {
    pub fn deserialize_primitive(
        metadata: &XdlPrimitiveMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XdlType> {
        match metadata {
            XdlPrimitiveMetadata::Bool => reader.read_u8().map(|x| XdlPrimitive::Bool(x != 0)),
            XdlPrimitiveMetadata::U8 => reader.read_u8().map(XdlPrimitive::U8),
            XdlPrimitiveMetadata::U16 => reader.read_u16::<LittleEndian>().map(XdlPrimitive::U16),
            XdlPrimitiveMetadata::U32 => reader.read_u32::<LittleEndian>().map(XdlPrimitive::U32),
            XdlPrimitiveMetadata::U64 => reader.read_u64::<LittleEndian>().map(XdlPrimitive::U64),
            XdlPrimitiveMetadata::U128 => {
                reader.read_u128::<LittleEndian>().map(XdlPrimitive::U128)
            }
            XdlPrimitiveMetadata::U256 => {
                let mut data = [0; 4];
                for i in &mut data {
                    *i = reader.read_u64::<LittleEndian>()?
                }
                Ok(XdlPrimitive::U256(data))
            }
            XdlPrimitiveMetadata::I8 => reader.read_i8().map(XdlPrimitive::I8),
            XdlPrimitiveMetadata::I16 => reader.read_i16::<LittleEndian>().map(XdlPrimitive::I16),
            XdlPrimitiveMetadata::I32 => reader.read_i32::<LittleEndian>().map(XdlPrimitive::I32),
            XdlPrimitiveMetadata::I64 => reader.read_i64::<LittleEndian>().map(XdlPrimitive::I64),
            XdlPrimitiveMetadata::I128 => {
                reader.read_i128::<LittleEndian>().map(XdlPrimitive::I128)
            }
            XdlPrimitiveMetadata::I256 => {
                let mut data = [0; 4];
                for i in &mut data {
                    *i = reader.read_u64::<LittleEndian>()?
                }
                Ok(XdlPrimitive::I256(data))
            }
            XdlPrimitiveMetadata::F32 => reader.read_f32::<LittleEndian>().map(XdlPrimitive::F32),
            XdlPrimitiveMetadata::F64 => reader.read_f64::<LittleEndian>().map(XdlPrimitive::F64),
            XdlPrimitiveMetadata::String => read_string(reader).map(XdlPrimitive::String),
        }
        .map(|x| x.into())
    }
}
