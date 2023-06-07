use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

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

impl From<&XdlPrimitive> for XdlPrimitiveMetadata {
    fn from(x: &XdlPrimitive) -> Self {
        match x {
            XdlPrimitive::Bool(_) => XdlPrimitiveMetadata::Bool,
            XdlPrimitive::U8(_) => XdlPrimitiveMetadata::U8,
            XdlPrimitive::U16(_) => XdlPrimitiveMetadata::U16,
            XdlPrimitive::U32(_) => XdlPrimitiveMetadata::U32,
            XdlPrimitive::U64(_) => XdlPrimitiveMetadata::U64,
            XdlPrimitive::U128(_) => XdlPrimitiveMetadata::U128,
            XdlPrimitive::U256(_) => XdlPrimitiveMetadata::U256,
            XdlPrimitive::I8(_) => XdlPrimitiveMetadata::I8,
            XdlPrimitive::I16(_) => XdlPrimitiveMetadata::I16,
            XdlPrimitive::I32(_) => XdlPrimitiveMetadata::I32,
            XdlPrimitive::I64(_) => XdlPrimitiveMetadata::I64,
            XdlPrimitive::I128(_) => XdlPrimitiveMetadata::I128,
            XdlPrimitive::I256(_) => XdlPrimitiveMetadata::I256,
            XdlPrimitive::F32(_) => XdlPrimitiveMetadata::F32,
            XdlPrimitive::F64(_) => XdlPrimitiveMetadata::F64,
            XdlPrimitive::String(_) => XdlPrimitiveMetadata::String,
        }
    }
}

impl XdlPrimitive {
    pub fn serialize_with_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        let metadata = XdlPrimitiveMetadata::from(self);
        writer.write_u8(metadata as u8)?;
        self.serialize_without_metadata(writer)
    }

    pub fn serialize_without_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
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

    pub fn deserialize_unknown_metadata(reader: &mut impl Read) -> io::Result<Self> {
        let type_to_deserialize = reader.read_u8()?.into();
        Self::deserialize_known_metadata(type_to_deserialize, reader)
    }

    pub fn deserialize_known_metadata(
        metadata: XdlPrimitiveMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XdlPrimitive> {
        match metadata {
            XdlPrimitiveMetadata::Bool => reader.read_u8().map(|x| XdlPrimitive::Bool(x != 0)),
            XdlPrimitiveMetadata::U8 => reader.read_u8().map(XdlPrimitive::U8),
            XdlPrimitiveMetadata::U16 => reader.read_u16::<LittleEndian>().map(XdlPrimitive::U16),
            XdlPrimitiveMetadata::U32 => reader.read_u32::<LittleEndian>().map(XdlPrimitive::U32),
            XdlPrimitiveMetadata::U64 => reader.read_u64::<LittleEndian>().map(XdlPrimitive::U64),
            XdlPrimitiveMetadata::U128 => {
                reader.read_u128::<LittleEndian>().map(XdlPrimitive::U128)
            }
            XdlPrimitiveMetadata::U256 => unimplemented!(),
            XdlPrimitiveMetadata::I8 => reader.read_i8().map(XdlPrimitive::I8),
            XdlPrimitiveMetadata::I16 => reader.read_i16::<LittleEndian>().map(XdlPrimitive::I16),
            XdlPrimitiveMetadata::I32 => reader.read_i32::<LittleEndian>().map(XdlPrimitive::I32),
            XdlPrimitiveMetadata::I64 => reader.read_i64::<LittleEndian>().map(XdlPrimitive::I64),
            XdlPrimitiveMetadata::I128 => {
                reader.read_i128::<LittleEndian>().map(XdlPrimitive::I128)
            }
            XdlPrimitiveMetadata::I256 => unimplemented!(),
            XdlPrimitiveMetadata::F32 => reader.read_f32::<LittleEndian>().map(XdlPrimitive::F32),
            XdlPrimitiveMetadata::F64 => reader.read_f64::<LittleEndian>().map(XdlPrimitive::F64),
            XdlPrimitiveMetadata::String => {
                let len = reader.read_u16::<LittleEndian>()?;
                let mut buf = vec![0; len as usize];
                reader.read_exact(&mut buf)?;
                let string = String::from_utf8(buf)
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))?;
                Ok(XdlPrimitive::String(string))
            }
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

#[cfg(test)]
mod primitive_test;
