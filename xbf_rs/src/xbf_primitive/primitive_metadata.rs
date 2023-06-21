use crate::Serialize;
use byteorder::WriteBytesExt;
use std::io::{self, Write};

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

impl TryFrom<u8> for XdlPrimitiveMetadata {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bool),
            1 => Ok(Self::U8),
            2 => Ok(Self::U16),
            3 => Ok(Self::U32),
            4 => Ok(Self::U64),
            5 => Ok(Self::U128),
            6 => Ok(Self::U256),
            7 => Ok(Self::I8),
            8 => Ok(Self::I16),
            9 => Ok(Self::I32),
            10 => Ok(Self::I64),
            11 => Ok(Self::I128),
            12 => Ok(Self::I256),
            13 => Ok(Self::F32),
            14 => Ok(Self::F64),
            15 => Ok(Self::String),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid primitive metadata",
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! serialize_primitive_metadata_test {
        ($xdl_type:tt, $expected_value:expr) => {
            let metadata = XdlPrimitiveMetadata::$xdl_type;
            let mut writer = Vec::new();
            metadata.serialize(&mut writer).unwrap();
            assert_eq!(writer, vec![$expected_value]);
        };
    }

    #[test]
    fn metadata_serialize_works() {
        serialize_primitive_metadata_test!(Bool, 0);
        serialize_primitive_metadata_test!(U8, 1);
        serialize_primitive_metadata_test!(U16, 2);
        serialize_primitive_metadata_test!(U32, 3);
        serialize_primitive_metadata_test!(U64, 4);
        serialize_primitive_metadata_test!(U128, 5);
        serialize_primitive_metadata_test!(U256, 6);
        serialize_primitive_metadata_test!(I8, 7);
        serialize_primitive_metadata_test!(I16, 8);
        serialize_primitive_metadata_test!(I32, 9);
        serialize_primitive_metadata_test!(I64, 10);
        serialize_primitive_metadata_test!(I128, 11);
        serialize_primitive_metadata_test!(I256, 12);
        serialize_primitive_metadata_test!(F32, 13);
        serialize_primitive_metadata_test!(F64, 14);
        serialize_primitive_metadata_test!(String, 15);
    }

    use crate::{DeserializeMetadata, XdlMetadata};
    use std::io::Cursor;

    macro_rules! deserialize_primitive_metadata_test {
        ($xdl_type:tt) => {
            let data = vec![XdlPrimitiveMetadata::$xdl_type as u8];
            let mut reader = Cursor::new(data);
            let metadata = XdlMetadata::deserialize_metadata(&mut reader).unwrap();
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
    fn metadata_try_from_u8_err_for_unknown_id() {
        let err = XdlPrimitiveMetadata::try_from(16).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid primitive metadata");
    }
}
