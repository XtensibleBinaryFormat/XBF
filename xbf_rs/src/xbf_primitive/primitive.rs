use super::primitive_metadata::XbfPrimitiveMetadata;
use crate::{
    util::{read_string, write_string},
    Serialize, XbfType,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum XbfPrimitive {
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

impl Serialize for XbfPrimitive {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XbfPrimitive::Bool(x) => writer.write_u8(u8::from(*x)),

            XbfPrimitive::U8(x) => writer.write_u8(*x),
            XbfPrimitive::U16(x) => writer.write_u16::<LittleEndian>(*x),
            XbfPrimitive::U32(x) => writer.write_u32::<LittleEndian>(*x),
            XbfPrimitive::U64(x) => writer.write_u64::<LittleEndian>(*x),
            XbfPrimitive::U128(x) => writer.write_u128::<LittleEndian>(*x),
            XbfPrimitive::U256(x) => x
                .iter()
                .try_for_each(|x| writer.write_u64::<LittleEndian>(*x)),

            XbfPrimitive::I8(x) => writer.write_i8(*x),
            XbfPrimitive::I16(x) => writer.write_i16::<LittleEndian>(*x),
            XbfPrimitive::I32(x) => writer.write_i32::<LittleEndian>(*x),
            XbfPrimitive::I64(x) => writer.write_i64::<LittleEndian>(*x),
            XbfPrimitive::I128(x) => writer.write_i128::<LittleEndian>(*x),
            XbfPrimitive::I256(x) => x
                .iter()
                .try_for_each(|x| writer.write_u64::<LittleEndian>(*x)),

            XbfPrimitive::F32(x) => writer.write_f32::<LittleEndian>(*x),
            XbfPrimitive::F64(x) => writer.write_f64::<LittleEndian>(*x),

            XbfPrimitive::String(x) => write_string(x, writer),
        }
    }
}

impl XbfPrimitive {
    pub fn deserialize_primitive(
        metadata: &XbfPrimitiveMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XbfType> {
        match metadata {
            XbfPrimitiveMetadata::Bool => reader.read_u8().map(|x| XbfPrimitive::Bool(x != 0)),
            XbfPrimitiveMetadata::U8 => reader.read_u8().map(XbfPrimitive::U8),
            XbfPrimitiveMetadata::U16 => reader.read_u16::<LittleEndian>().map(XbfPrimitive::U16),
            XbfPrimitiveMetadata::U32 => reader.read_u32::<LittleEndian>().map(XbfPrimitive::U32),
            XbfPrimitiveMetadata::U64 => reader.read_u64::<LittleEndian>().map(XbfPrimitive::U64),
            XbfPrimitiveMetadata::U128 => {
                reader.read_u128::<LittleEndian>().map(XbfPrimitive::U128)
            }
            XbfPrimitiveMetadata::U256 => {
                let mut data = [0; 4];
                for i in &mut data {
                    *i = reader.read_u64::<LittleEndian>()?
                }
                Ok(XbfPrimitive::U256(data))
            }
            XbfPrimitiveMetadata::I8 => reader.read_i8().map(XbfPrimitive::I8),
            XbfPrimitiveMetadata::I16 => reader.read_i16::<LittleEndian>().map(XbfPrimitive::I16),
            XbfPrimitiveMetadata::I32 => reader.read_i32::<LittleEndian>().map(XbfPrimitive::I32),
            XbfPrimitiveMetadata::I64 => reader.read_i64::<LittleEndian>().map(XbfPrimitive::I64),
            XbfPrimitiveMetadata::I128 => {
                reader.read_i128::<LittleEndian>().map(XbfPrimitive::I128)
            }
            XbfPrimitiveMetadata::I256 => {
                let mut data = [0; 4];
                for i in &mut data {
                    *i = reader.read_u64::<LittleEndian>()?
                }
                Ok(XbfPrimitive::I256(data))
            }
            XbfPrimitiveMetadata::F32 => reader.read_f32::<LittleEndian>().map(XbfPrimitive::F32),
            XbfPrimitiveMetadata::F64 => reader.read_f64::<LittleEndian>().map(XbfPrimitive::F64),
            XbfPrimitiveMetadata::String => read_string(reader).map(XbfPrimitive::String),
        }
        .map(|x| x.into())
    }
}
