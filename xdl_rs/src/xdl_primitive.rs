use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Write};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum XdlPrimitiveId {
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

impl From<u8> for XdlPrimitiveId {
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

pub struct XdlPrimitiveMetadata(pub XdlPrimitiveId);

#[derive(Debug, Clone)]
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
            XdlPrimitive::Bool(_) => XdlPrimitiveMetadata(XdlPrimitiveId::Bool),
            XdlPrimitive::U8(_) => XdlPrimitiveMetadata(XdlPrimitiveId::U8),
            XdlPrimitive::U16(_) => XdlPrimitiveMetadata(XdlPrimitiveId::U16),
            XdlPrimitive::U32(_) => XdlPrimitiveMetadata(XdlPrimitiveId::U32),
            XdlPrimitive::U64(_) => XdlPrimitiveMetadata(XdlPrimitiveId::U64),
            XdlPrimitive::U128(_) => XdlPrimitiveMetadata(XdlPrimitiveId::U128),
            XdlPrimitive::U256(_) => XdlPrimitiveMetadata(XdlPrimitiveId::U256),
            XdlPrimitive::I8(_) => XdlPrimitiveMetadata(XdlPrimitiveId::I8),
            XdlPrimitive::I16(_) => XdlPrimitiveMetadata(XdlPrimitiveId::I16),
            XdlPrimitive::I32(_) => XdlPrimitiveMetadata(XdlPrimitiveId::I32),
            XdlPrimitive::I64(_) => XdlPrimitiveMetadata(XdlPrimitiveId::I64),
            XdlPrimitive::I128(_) => XdlPrimitiveMetadata(XdlPrimitiveId::I128),
            XdlPrimitive::I256(_) => XdlPrimitiveMetadata(XdlPrimitiveId::I256),
            XdlPrimitive::F32(_) => XdlPrimitiveMetadata(XdlPrimitiveId::F32),
            XdlPrimitive::F64(_) => XdlPrimitiveMetadata(XdlPrimitiveId::F64),
            XdlPrimitive::String(_) => XdlPrimitiveMetadata(XdlPrimitiveId::String),
        }
    }
}

impl XdlPrimitive {
    pub fn serialize_with_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        let metadata = XdlPrimitiveMetadata::from(self);
        writer.write_u8(metadata.0 as u8)?;
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
mod test {
    use super::*;

    macro_rules! make_with_metadata_test {
        ($xdl_type:tt) => {
            (|| {
                let primitive = XdlPrimitive::$xdl_type(42);
                let mut writer = Vec::new();
                primitive.serialize_with_metadata(&mut writer).unwrap();
                writer
            })()
        };
    }

    macro_rules! make_without_metadata_test {
        ($xdl_type:tt) => {
            (|| {
                let primitive = XdlPrimitive::$xdl_type(42);
                let mut writer = Vec::new();
                primitive.serialize_without_metadata(&mut writer).unwrap();
                writer
            })()
        };
    }

    #[test]
    fn primitive_id_from_u8_works_for_known_id() {
        assert_eq!(XdlPrimitiveId::from(0), XdlPrimitiveId::Bool);
        assert_eq!(XdlPrimitiveId::from(1), XdlPrimitiveId::U8);
        assert_eq!(XdlPrimitiveId::from(2), XdlPrimitiveId::U16);
        assert_eq!(XdlPrimitiveId::from(3), XdlPrimitiveId::U32);
        assert_eq!(XdlPrimitiveId::from(4), XdlPrimitiveId::U64);
        assert_eq!(XdlPrimitiveId::from(5), XdlPrimitiveId::U128);
        assert_eq!(XdlPrimitiveId::from(6), XdlPrimitiveId::U256);
        assert_eq!(XdlPrimitiveId::from(7), XdlPrimitiveId::I8);
        assert_eq!(XdlPrimitiveId::from(8), XdlPrimitiveId::I16);
        assert_eq!(XdlPrimitiveId::from(9), XdlPrimitiveId::I32);
        assert_eq!(XdlPrimitiveId::from(10), XdlPrimitiveId::I64);
        assert_eq!(XdlPrimitiveId::from(11), XdlPrimitiveId::I128);
        assert_eq!(XdlPrimitiveId::from(12), XdlPrimitiveId::I256);
        assert_eq!(XdlPrimitiveId::from(13), XdlPrimitiveId::F32);
        assert_eq!(XdlPrimitiveId::from(14), XdlPrimitiveId::F64);
        assert_eq!(XdlPrimitiveId::from(15), XdlPrimitiveId::String);
    }

    #[test]
    #[should_panic(expected = "invalid primitive id 16")]
    fn primitive_id_from_u8_panics_for_unknown_id() {
        let _ = XdlPrimitiveId::from(16);
    }

    #[test]
    fn bool_serialize_with_metadata_works() {
        let xdl_true = XdlPrimitive::Bool(true);
        let xdl_false = XdlPrimitive::Bool(false);
        let mut writer = Vec::new();

        xdl_true.serialize_with_metadata(&mut writer).unwrap();
        xdl_false.serialize_with_metadata(&mut writer).unwrap();
        assert_eq!(writer, vec![XdlPrimitiveId::Bool as u8, 1, 0, 0]);
    }
    #[test]
    fn bool_serialize_without_metadata_works() {
        let xdl_true = XdlPrimitive::Bool(true);
        let xdl_false = XdlPrimitive::Bool(false);
        let mut writer = Vec::new();

        xdl_true.serialize_without_metadata(&mut writer).unwrap();
        xdl_false.serialize_without_metadata(&mut writer).unwrap();
        assert_eq!(writer, vec![1, 0]);
    }

    #[test]
    fn u8_serialize_with_metadata_works() {
        let writer = make_with_metadata_test!(U8);
        assert_eq!(writer, vec![XdlPrimitiveId::U8 as u8, 42]);
    }
    #[test]
    fn u8_serialize_without_metadata_works() {
        let writer = make_without_metadata_test!(U8);
        assert_eq!(writer, vec![42]);
    }

    #[test]
    fn u16_serialize_with_metadata_works() {
        let writer = make_with_metadata_test!(U16);
        assert_eq!(writer, vec![XdlPrimitiveId::U16 as u8, 42, 0]);
    }
    #[test]
    fn u16_serialize_without_metadata_works() {
        let writer = make_without_metadata_test!(U16);
        assert_eq!(writer, vec![42, 0]);
    }

    #[test]
    fn u32_serialize_with_metadata_works() {
        let writer = make_with_metadata_test!(U32);
        assert_eq!(writer, vec![XdlPrimitiveId::U32 as u8, 42, 0, 0, 0]);
    }
    #[test]
    fn u32_serialize_without_metadata_works() {
        let writer = make_without_metadata_test!(U32);
        assert_eq!(writer, vec![42, 0, 0, 0]);
    }

    #[test]
    fn u64_serialize_with_metadata_works() {
        let writer = make_with_metadata_test!(U64);
        assert_eq!(
            writer,
            vec![XdlPrimitiveId::U64 as u8, 42, 0, 0, 0, 0, 0, 0, 0]
        );
    }
    #[test]
    fn u64_serialize_without_metadata_works() {
        let writer = make_without_metadata_test!(U64);
        assert_eq!(writer, vec![42, 0, 0, 0, 0, 0, 0, 0]);
    }
}
