use super::XdlPrimitive;
use crate::{DeserializeMetadata, Serialize, XdlMetadata};
use byteorder::{ReadBytesExt, WriteBytesExt};
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

impl Serialize for XdlPrimitiveMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

impl DeserializeMetadata for XdlPrimitiveMetadata {
    fn deserialize(reader: &mut impl Read) -> io::Result<XdlMetadata> {
        reader
            .read_u8()
            // TODO: could we use try_from instead of panicking?
            .map(XdlPrimitiveMetadata::from)
            .map(XdlMetadata::Primitive)
    }
}

// TODO: use try_from instead to remove panicking path?
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! serialize_primitive_metdatada_test {
        ($xdl_type:tt, $expected_value:expr) => {
            let metadata = XdlPrimitiveMetadata::$xdl_type;
            let mut writer = Vec::new();
            metadata.serialize(&mut writer).unwrap();
            assert_eq!(writer, vec![$expected_value]);
        };
    }

    #[test]
    fn metadata_serialize_works() {
        serialize_primitive_metdatada_test!(Bool, 0);
        serialize_primitive_metdatada_test!(U8, 1);
        serialize_primitive_metdatada_test!(U16, 2);
        serialize_primitive_metdatada_test!(U32, 3);
        serialize_primitive_metdatada_test!(U64, 4);
        serialize_primitive_metdatada_test!(U128, 5);
        serialize_primitive_metdatada_test!(U256, 6);
        serialize_primitive_metdatada_test!(I8, 7);
        serialize_primitive_metdatada_test!(I16, 8);
        serialize_primitive_metdatada_test!(I32, 9);
        serialize_primitive_metdatada_test!(I64, 10);
        serialize_primitive_metdatada_test!(I128, 11);
        serialize_primitive_metdatada_test!(I256, 12);
        serialize_primitive_metdatada_test!(F32, 13);
        serialize_primitive_metdatada_test!(F64, 14);
        serialize_primitive_metdatada_test!(String, 15);
    }

    use std::io::Cursor;

    macro_rules! deserialize_primitive_metadata_test {
        ($xdl_type:tt) => {
            let data = vec![XdlPrimitiveMetadata::$xdl_type as u8];
            let mut reader = Cursor::new(data);
            let metadata = XdlPrimitiveMetadata::deserialize(&mut reader).unwrap();
            assert_eq!(
                metadata,
                XdlMetadata::Primitive(XdlPrimitiveMetadata::$xdl_type)
            );
        };
    }

    #[test]
    fn metadata_deserialize_works() {
        deserialize_primitive_metadata_test!(Bool);
        deserialize_primitive_metadata_test!(U8);
        deserialize_primitive_metadata_test!(U16);
        deserialize_primitive_metadata_test!(U32);
        deserialize_primitive_metadata_test!(U64);
        deserialize_primitive_metadata_test!(U128);
        deserialize_primitive_metadata_test!(U256);
        deserialize_primitive_metadata_test!(I8);
        deserialize_primitive_metadata_test!(I16);
        deserialize_primitive_metadata_test!(I32);
        deserialize_primitive_metadata_test!(I64);
        deserialize_primitive_metadata_test!(I128);
        deserialize_primitive_metadata_test!(I256);
        deserialize_primitive_metadata_test!(F32);
        deserialize_primitive_metadata_test!(F64);
        deserialize_primitive_metadata_test!(String);
    }

    #[test]
    #[should_panic(expected = "invalid primitive id 16")]
    fn metadata_from_u8_panics_for_unknown_id() {
        let _ = XdlPrimitiveMetadata::from(16);
    }

    #[test]
    fn metadata_from_primitive_works() {
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::Bool(true)),
            XdlPrimitiveMetadata::Bool
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U8(1)),
            XdlPrimitiveMetadata::U8
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U16(1)),
            XdlPrimitiveMetadata::U16
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U32(1)),
            XdlPrimitiveMetadata::U32
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U64(1)),
            XdlPrimitiveMetadata::U64
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U128(1)),
            XdlPrimitiveMetadata::U128
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U256(())),
            XdlPrimitiveMetadata::U256
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I8(1)),
            XdlPrimitiveMetadata::I8
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I16(1)),
            XdlPrimitiveMetadata::I16
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I32(1)),
            XdlPrimitiveMetadata::I32
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I64(1)),
            XdlPrimitiveMetadata::I64
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I128(1)),
            XdlPrimitiveMetadata::I128
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I256(())),
            XdlPrimitiveMetadata::I256
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::F32(1.0)),
            XdlPrimitiveMetadata::F32
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::F64(1.0)),
            XdlPrimitiveMetadata::F64
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::String("test".to_string())),
            XdlPrimitiveMetadata::String
        );
    }
}
