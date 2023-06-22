use super::primitive_metadata::XdlPrimitiveMetadata;
use crate::{
    util::{read_string, write_string},
    XdlTypeUpcast,
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

impl XdlPrimitive {
    pub fn serialize_primitive_type(&self, writer: &mut impl Write) -> io::Result<()> {
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

            XdlPrimitive::String(x) => write_string(x, writer),
        }
    }

    pub fn deserialize_primitive_type(
        metadata: &XdlPrimitiveMetadata,
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
            XdlPrimitiveMetadata::String => read_string(reader).map(XdlPrimitive::String),
        }
    }
}

impl XdlTypeUpcast for XdlPrimitive {}

macro_rules! xdl_primitive_from_native_impl {
    ($ty:ty, $xdl_type:tt) => {
        impl From<$ty> for XdlPrimitive {
            fn from(x: $ty) -> Self {
                XdlPrimitive::$xdl_type(x)
            }
        }
    };
}

xdl_primitive_from_native_impl!(bool, Bool);

xdl_primitive_from_native_impl!(u8, U8);
xdl_primitive_from_native_impl!(u16, U16);
xdl_primitive_from_native_impl!(u32, U32);
xdl_primitive_from_native_impl!(u64, U64);
xdl_primitive_from_native_impl!(u128, U128);

xdl_primitive_from_native_impl!(i8, I8);
xdl_primitive_from_native_impl!(i16, I16);
xdl_primitive_from_native_impl!(i32, I32);
xdl_primitive_from_native_impl!(i64, I64);
xdl_primitive_from_native_impl!(i128, I128);

xdl_primitive_from_native_impl!(f32, F32);
xdl_primitive_from_native_impl!(f64, F64);

xdl_primitive_from_native_impl!(String, String);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{XdlMetadata, XdlType};
    use std::io::Cursor;

    macro_rules! serde_primitive_test {
        ($xdl_type:tt, $test_num:expr) => {
            let primitive = XdlPrimitive::$xdl_type($test_num);
            let mut writer = Vec::new();

            primitive.serialize_primitive_type(&mut writer).unwrap();

            let expected = $test_num.to_le_bytes();
            assert_eq!(writer, expected);

            let mut reader = Cursor::new(writer);

            let metadata = XdlMetadata::Primitive(XdlPrimitiveMetadata::$xdl_type);
            let expected = XdlType::Primitive(XdlPrimitive::$xdl_type($test_num));

            let primitive = XdlType::deserialize_base_type(&metadata, &mut reader).unwrap();
            assert_eq!(primitive, expected);
        };
    }

    #[test]
    fn bool_serde_works() {
        let xdl_true = XdlPrimitive::Bool(true);
        let xdl_false = XdlPrimitive::Bool(false);
        let mut writer = Vec::new();

        xdl_true.serialize_primitive_type(&mut writer).unwrap();
        xdl_false.serialize_primitive_type(&mut writer).unwrap();

        assert_eq!(writer, vec![1, 0]);

        let mut reader = Cursor::new(writer);
        let metadata = XdlMetadata::Primitive(XdlPrimitiveMetadata::Bool);

        let true_type = XdlType::deserialize_base_type(&metadata, &mut reader).unwrap();
        let false_type = XdlType::deserialize_base_type(&metadata, &mut reader).unwrap();

        assert_eq!(true_type, XdlType::Primitive(XdlPrimitive::Bool(true)));
        assert_eq!(false_type, XdlType::Primitive(XdlPrimitive::Bool(false)));
    }

    #[test]
    fn unsigned_nums_serde_works() {
        serde_primitive_test!(U8, 42u8);
        serde_primitive_test!(U16, 420u16);
        serde_primitive_test!(U32, 100_000u32);
        serde_primitive_test!(U64, 100_000_000u64);
        serde_primitive_test!(U128, 18_446_744_073_709_551_617u128);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn u256_serialize_works() {
        let dne = XdlPrimitive::U256(());
        dne.serialize_primitive_type(&mut Vec::new()).unwrap();
    }
    #[test]
    #[should_panic(expected = "not implemented")]
    fn u256_deserialize_works() {
        let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::U256 as u8]);
        XdlType::deserialize_base_type(
            &XdlMetadata::Primitive(XdlPrimitiveMetadata::U256),
            &mut reader,
        )
        .unwrap();
    }

    #[test]
    fn signed_nums_serde_works() {
        serde_primitive_test!(I8, 42i8);
        serde_primitive_test!(I16, 420i16);
        serde_primitive_test!(I32, 100_000i32);
        serde_primitive_test!(I64, 100_000_000i64);
        serde_primitive_test!(I128, 18_446_744_073_709_551_617i128);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn i256_serialize_works() {
        let dne = XdlPrimitive::I256(());
        dne.serialize_primitive_type(&mut Vec::new()).unwrap();
    }
    #[test]
    #[should_panic(expected = "not implemented")]
    fn i256_deserialize_works() {
        let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::I256 as u8]);
        XdlType::deserialize_base_type(
            &XdlMetadata::Primitive(XdlPrimitiveMetadata::I256),
            &mut reader,
        )
        .unwrap();
    }

    #[test]
    fn floating_point_serde_works() {
        serde_primitive_test!(F32, 69.0f32);
        serde_primitive_test!(F64, 69.0f64);
    }

    #[test]
    fn string_serialize_works() {
        let test_string = "hello world".to_string();
        let primitive = XdlPrimitive::String(test_string.clone());
        let mut writer = vec![];

        primitive.serialize_primitive_type(&mut writer).unwrap();

        let mut expected_writer = vec![];
        expected_writer.extend_from_slice(&(test_string.len() as u16).to_le_bytes());
        expected_writer.extend_from_slice(test_string.as_bytes());

        assert_eq!(writer, expected_writer);

        let mut reader = Cursor::new(writer);
        let deserialized = XdlType::deserialize_base_type(
            &XdlMetadata::Primitive(XdlPrimitiveMetadata::String),
            &mut reader,
        )
        .unwrap();

        assert_eq!(
            deserialized,
            XdlType::Primitive(XdlPrimitive::String(test_string))
        );
    }

    #[test]
    fn upcast_works() {
        let primitive_type = XdlPrimitive::I32(69);
        let ref_primitive_type = &primitive_type;

        assert_eq!(
            XdlType::Primitive(primitive_type.clone()),
            ref_primitive_type.to_base_type() // ref_primitive_type.to_base_type()
        );
        assert_eq!(
            XdlType::Primitive(primitive_type.clone()),
            primitive_type.into_base_type()
        );
    }

    macro_rules! primitive_from_native_test {
        ($ty:ty, $xdl_type:tt, $test_num:expr) => {
            let value: $ty = $test_num;
            let primitive: XdlPrimitive = value.clone().into();
            assert_eq!(primitive, XdlPrimitive::$xdl_type(value));
        };
    }

    #[test]
    fn primitive_from_native_works() {
        primitive_from_native_test!(bool, Bool, true);
        primitive_from_native_test!(bool, Bool, false);
        primitive_from_native_test!(u8, U8, 42);
        primitive_from_native_test!(u16, U16, 42);
        primitive_from_native_test!(u32, U32, 42);
        primitive_from_native_test!(u64, U64, 42);
        primitive_from_native_test!(u128, U128, 42);
        primitive_from_native_test!(i8, I8, 42);
        primitive_from_native_test!(i16, I16, 42);
        primitive_from_native_test!(i32, I32, 42);
        primitive_from_native_test!(i64, I64, 42);
        primitive_from_native_test!(i128, I128, 42);
        primitive_from_native_test!(f32, F32, 42.0);
        primitive_from_native_test!(f64, F64, 42.0);
        primitive_from_native_test!(String, String, "Hello World".to_string());
    }
}
