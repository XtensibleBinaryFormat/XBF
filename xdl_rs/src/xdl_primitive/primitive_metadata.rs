use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{DeserializeMetadata, Serialize, XdlMetadata};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum XdlPrimitiveMetadata {
    Bool = 0,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    I8,
    I16,
    I32,
    I64,
    I128,
    I256,
    F32,
    F64,
    String,
}

impl Serialize for XdlPrimitiveMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

impl DeserializeMetadata for XdlPrimitiveMetadata {
    fn deserialize(reader: &mut impl Read) -> io::Result<XdlMetadata> {
        reader
            .read_u8()
            .map(XdlPrimitiveMetadata::from)
            .map(XdlMetadata::new_primitive_metadata)
    }
}

impl From<u8> for XdlPrimitiveMetadata {
    fn from(x: u8) -> Self {
        match x {
            0 => Self::Bool,
            1 => Self::U8,
            2 => Self::U16,
            3 => Self::U32,
            4 => Self::U64,
            5 => Self::U128,
            6 => Self::U256,
            7 => Self::I8,
            8 => Self::I16,
            9 => Self::I32,
            10 => Self::I64,
            11 => Self::I128,
            12 => Self::I256,
            13 => Self::F32,
            14 => Self::F64,
            15 => Self::String,
            _ => panic!("invalid primitive id {x}"),
        }
    }
}
