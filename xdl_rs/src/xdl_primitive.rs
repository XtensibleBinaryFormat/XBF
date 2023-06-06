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

    macro_rules! with_metadata_test {
        ($xdl_type:tt, $test_num:expr) => {
            let primitive = XdlPrimitive::$xdl_type($test_num);
            let mut writer = Vec::new();
            primitive.serialize_with_metadata(&mut writer).unwrap();
            let mut expected = vec![XdlPrimitiveMetadata::from(&primitive).0 as u8];
            expected.extend_from_slice(&$test_num.to_le_bytes());
            assert_eq!(writer, expected);
        };
    }

    macro_rules! without_metadata_test {
        ($xdl_type:tt, $test_num:expr) => {
            let primitive = XdlPrimitive::$xdl_type($test_num);
            let mut writer = Vec::new();
            primitive.serialize_without_metadata(&mut writer).unwrap();
            let expected = $test_num.to_le_bytes();
            assert_eq!(writer, expected);
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
        const TEST_NUM: u8 = 42;
        with_metadata_test!(U8, TEST_NUM);
    }
    #[test]
    fn u8_serialize_without_metadata_works() {
        const TEST_NUM: u8 = 42;
        without_metadata_test!(U8, TEST_NUM);
    }

    #[test]
    fn u16_serialize_with_metadata_works() {
        const TEST_NUM: u16 = 420;
        with_metadata_test!(U16, TEST_NUM);
    }
    #[test]
    fn u16_serialize_without_metadata_works() {
        const TEST_NUM: u16 = 420;
        without_metadata_test!(U16, TEST_NUM);
    }

    #[test]
    fn u32_serialize_with_metadata_works() {
        const TEST_NUM: u32 = 100_000;
        with_metadata_test!(U32, TEST_NUM);
    }
    #[test]
    fn u32_serialize_without_metadata_works() {
        const TEST_NUM: u32 = 100_000;
        without_metadata_test!(U32, TEST_NUM);
    }

    #[test]
    fn u64_serialize_with_metadata_works() {
        const TEST_NUM: u64 = 100_000_000;
        with_metadata_test!(U64, TEST_NUM);
    }
    #[test]
    fn u64_serialize_without_metadata_works() {
        const TEST_NUM: u64 = 100_000_000;
        without_metadata_test!(U64, TEST_NUM);
    }

    #[test]
    fn u128_serialize_with_metadata_works() {
        const TEST_NUM: u128 = 18_446_744_073_709_551_617;
        with_metadata_test!(U128, TEST_NUM);
    }
    #[test]
    fn u128_serialize_without_metadata_works() {
        const TEST_NUM: u128 = 18_446_744_073_709_551_617;
        without_metadata_test!(U128, TEST_NUM);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn u256_serialize_with_metadata_works() {
        let dne = XdlPrimitive::U256(());
        dne.serialize_with_metadata(&mut Vec::new()).unwrap();
    }
    #[test]
    #[should_panic(expected = "not implemented")]
    fn u256_serialize_without_metadata_works() {
        let dne = XdlPrimitive::U256(());
        dne.serialize_without_metadata(&mut Vec::new()).unwrap();
    }

    #[test]
    fn i8_serialize_with_metadata_works() {
        const TEST_NUM: i8 = 42;
        with_metadata_test!(I8, TEST_NUM);
    }
    #[test]
    fn i8_serialize_without_metadata_works() {
        const TEST_NUM: i8 = 42;
        without_metadata_test!(I8, TEST_NUM);
    }

    #[test]
    fn i16_serialize_with_metadata_works() {
        const TEST_NUM: i16 = 420;
        with_metadata_test!(I16, TEST_NUM);
    }
    #[test]
    fn i16_serialize_without_metadata_works() {
        const TEST_NUM: i16 = 420;
        without_metadata_test!(I16, TEST_NUM);
    }

    #[test]
    fn i32_serialize_with_metadata_works() {
        const TEST_NUM: i32 = 100_000;
        with_metadata_test!(I32, TEST_NUM);
    }
    #[test]
    fn i32_serialize_without_metadata_works() {
        const TEST_NUM: i32 = 100_000;
        without_metadata_test!(I32, TEST_NUM);
    }

    #[test]
    fn i64_serialize_with_metadata_works() {
        const TEST_NUM: i64 = 100_000_000;
        with_metadata_test!(I64, TEST_NUM);
    }
    #[test]
    fn i64_serialize_without_metadata_works() {
        const TEST_NUM: i64 = 100_000_000;
        without_metadata_test!(I64, TEST_NUM);
    }

    #[test]
    fn i128_serialize_with_metadata_works() {
        const TEST_NUM: i128 = 18_446_744_073_709_551_617;
        with_metadata_test!(I128, TEST_NUM);
    }
    #[test]
    fn i128_serialize_without_metadata_works() {
        const TEST_NUM: i128 = 18_446_744_073_709_551_617;
        without_metadata_test!(I128, TEST_NUM);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn i256_serialize_with_metadata_works() {
        let dne = XdlPrimitive::I256(());
        dne.serialize_with_metadata(&mut Vec::new()).unwrap();
    }
    #[test]
    #[should_panic(expected = "not implemented")]
    fn i256_serialize_without_metadata_works() {
        let dne = XdlPrimitive::I256(());
        dne.serialize_without_metadata(&mut Vec::new()).unwrap();
    }

    #[test]
    fn f32_serialize_with_metadata_works() {
        const TEST_NUM: f32 = 69.0;
        with_metadata_test!(F32, TEST_NUM);
    }
    #[test]
    fn f32_serialize_without_metadata_works() {
        const TEST_NUM: f32 = 69.0;
        without_metadata_test!(F32, TEST_NUM);
    }

    #[test]
    fn f64_serialize_with_metadata_works() {
        const TEST_NUM: f64 = 69.0;
        with_metadata_test!(F64, TEST_NUM);
    }
    #[test]
    fn f64_serialize_without_metadata_works() {
        const TEST_NUM: f64 = 69.0;
        without_metadata_test!(F64, TEST_NUM);
    }

    #[test]
    fn string_serialize_with_metadata_works() {
        let test_string = "hello world".to_string();
        let primitive = XdlPrimitive::String(test_string.clone());

        let mut writer = Vec::new();
        primitive.serialize_with_metadata(&mut writer).unwrap();

        // id of the type
        let mut expected = vec![XdlPrimitiveMetadata::from(&primitive).0 as u8];
        // length of the string
        expected.extend_from_slice(&(test_string.len() as u16).to_le_bytes());
        // contents of the string
        expected.extend_from_slice(test_string.as_bytes());

        assert_eq!(writer, expected);
    }
    #[test]
    fn string_serialize_without_metadata_works() {
        let test_string = "hello world".to_string();
        let primitive = XdlPrimitive::String(test_string.clone());
        let mut writer = Vec::new();
        primitive.serialize_without_metadata(&mut writer).unwrap();

        // length as a u16
        let mut expected = (&(test_string.len() as u16).to_le_bytes()).to_vec();
        // contents of the string
        expected.extend_from_slice(test_string.as_bytes());

        assert_eq!(writer, expected);
    }
}
