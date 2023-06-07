use super::*;
use std::io::Cursor;

macro_rules! serialize_with_metadata_test {
    ($xdl_type:tt, $test_num:expr) => {
        let primitive = XdlPrimitive::$xdl_type($test_num);
        let mut writer = Vec::new();

        primitive.serialize_with_metadata(&mut writer).unwrap();

        let mut expected = vec![XdlPrimitiveMetadata::from(&primitive) as u8];
        expected.extend_from_slice(&$test_num.to_le_bytes());
        assert_eq!(writer, expected);
    };
}

macro_rules! serialize_without_metadata_test {
    ($xdl_type:tt, $test_num:expr) => {
        let primitive = XdlPrimitive::$xdl_type($test_num);
        let mut writer = Vec::new();
        primitive.serialize_without_metadata(&mut writer).unwrap();

        let expected = $test_num.to_le_bytes();
        assert_eq!(writer, expected);
    };
}

macro_rules! deserialize_unknown_metadata_test {
    ($xdl_type:tt, $test_num:expr) => {
        let mut data = vec![XdlPrimitiveMetadata::$xdl_type as u8];
        data.extend_from_slice(&$test_num.to_le_bytes());
        let mut reader = Cursor::new(data);

        let primitive = XdlPrimitive::deserialize_unknown_metadata(&mut reader).unwrap();
        assert_eq!(primitive, XdlPrimitive::$xdl_type($test_num));
    };
}

macro_rules! deserialize_known_metadata_test {
    ($xdl_type:tt, $test_num:expr) => {
        let known_metadata = XdlPrimitiveMetadata::$xdl_type;
        let data = &$test_num.to_le_bytes();
        let mut reader = Cursor::new(data);

        let primitive =
            XdlPrimitive::deserialize_known_metadata(known_metadata, &mut reader).unwrap();
        assert_eq!(primitive, XdlPrimitive::$xdl_type($test_num));
    };
}

#[test]
fn primitive_id_from_u8_works_for_known_id() {
    assert_eq!(XdlPrimitiveMetadata::from(0), XdlPrimitiveMetadata::Bool);
    assert_eq!(XdlPrimitiveMetadata::from(1), XdlPrimitiveMetadata::U8);
    assert_eq!(XdlPrimitiveMetadata::from(2), XdlPrimitiveMetadata::U16);
    assert_eq!(XdlPrimitiveMetadata::from(3), XdlPrimitiveMetadata::U32);
    assert_eq!(XdlPrimitiveMetadata::from(4), XdlPrimitiveMetadata::U64);
    assert_eq!(XdlPrimitiveMetadata::from(5), XdlPrimitiveMetadata::U128);
    assert_eq!(XdlPrimitiveMetadata::from(6), XdlPrimitiveMetadata::U256);
    assert_eq!(XdlPrimitiveMetadata::from(7), XdlPrimitiveMetadata::I8);
    assert_eq!(XdlPrimitiveMetadata::from(8), XdlPrimitiveMetadata::I16);
    assert_eq!(XdlPrimitiveMetadata::from(9), XdlPrimitiveMetadata::I32);
    assert_eq!(XdlPrimitiveMetadata::from(10), XdlPrimitiveMetadata::I64);
    assert_eq!(XdlPrimitiveMetadata::from(11), XdlPrimitiveMetadata::I128);
    assert_eq!(XdlPrimitiveMetadata::from(12), XdlPrimitiveMetadata::I256);
    assert_eq!(XdlPrimitiveMetadata::from(13), XdlPrimitiveMetadata::F32);
    assert_eq!(XdlPrimitiveMetadata::from(14), XdlPrimitiveMetadata::F64);
    assert_eq!(XdlPrimitiveMetadata::from(15), XdlPrimitiveMetadata::String);
}

#[test]
#[should_panic(expected = "invalid primitive id 16")]
fn primitive_id_from_u8_panics_for_unknown_id() {
    let _ = XdlPrimitiveMetadata::from(16);
}

#[test]
fn bool_serialize_with_metadata_works() {
    let xdl_true = XdlPrimitive::Bool(true);
    let xdl_false = XdlPrimitive::Bool(false);
    let mut writer = Vec::new();

    xdl_true.serialize_with_metadata(&mut writer).unwrap();
    xdl_false.serialize_with_metadata(&mut writer).unwrap();
    assert_eq!(writer, vec![XdlPrimitiveMetadata::Bool as u8, 1, 0, 0]);
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
    serialize_with_metadata_test!(U8, TEST_NUM);
}
#[test]
fn u8_serialize_without_metadata_works() {
    const TEST_NUM: u8 = 42;
    serialize_without_metadata_test!(U8, TEST_NUM);
}
#[test]
fn u8_deserialize_unknown_metadata_works() {
    const TEST_NUM: u8 = 42;
    deserialize_unknown_metadata_test!(U8, TEST_NUM);
}
#[test]
fn u8_deserialize_known_metadata_works() {
    const TEST_NUM: u8 = 42;
    deserialize_known_metadata_test!(U8, TEST_NUM);
}

#[test]
fn u16_serialize_with_metadata_works() {
    const TEST_NUM: u16 = 420;
    serialize_with_metadata_test!(U16, TEST_NUM);
}
#[test]
fn u16_serialize_without_metadata_works() {
    const TEST_NUM: u16 = 420;
    serialize_without_metadata_test!(U16, TEST_NUM);
}
#[test]
fn u16_deserialize_unknown_metadata_works() {
    const TEST_NUM: u16 = 420;
    deserialize_unknown_metadata_test!(U16, TEST_NUM);
}
#[test]
fn u16_deserialize_known_metadata_works() {
    const TEST_NUM: u16 = 420;
    deserialize_known_metadata_test!(U16, TEST_NUM);
}

#[test]
fn u32_serialize_with_metadata_works() {
    const TEST_NUM: u32 = 100_000;
    serialize_with_metadata_test!(U32, TEST_NUM);
}
#[test]
fn u32_serialize_without_metadata_works() {
    const TEST_NUM: u32 = 100_000;
    serialize_without_metadata_test!(U32, TEST_NUM);
}
#[test]
fn u32_deserialize_unknown_metadata_works() {
    const TEST_NUM: u32 = 100_000;
    deserialize_unknown_metadata_test!(U32, TEST_NUM);
}
#[test]
fn u32_deserialize_known_metadata_works() {
    const TEST_NUM: u32 = 100_000;
    deserialize_known_metadata_test!(U32, TEST_NUM);
}

#[test]
fn u64_serialize_with_metadata_works() {
    const TEST_NUM: u64 = 100_000_000;
    serialize_with_metadata_test!(U64, TEST_NUM);
}
#[test]
fn u64_serialize_without_metadata_works() {
    const TEST_NUM: u64 = 100_000_000;
    serialize_without_metadata_test!(U64, TEST_NUM);
}
#[test]
fn u64_deserialize_unknown_metadata_works() {
    const TEST_NUM: u64 = 100_000_000;
    deserialize_unknown_metadata_test!(U64, TEST_NUM);
}
#[test]
fn u64_deserialize_known_metadata_works() {
    const TEST_NUM: u64 = 100_000_000;
    deserialize_known_metadata_test!(U64, TEST_NUM);
}

#[test]
fn u128_serialize_with_metadata_works() {
    const TEST_NUM: u128 = 18_446_744_073_709_551_617;
    serialize_with_metadata_test!(U128, TEST_NUM);
}
#[test]
fn u128_serialize_without_metadata_works() {
    const TEST_NUM: u128 = 18_446_744_073_709_551_617;
    serialize_without_metadata_test!(U128, TEST_NUM);
}
#[test]
fn u128_deserialize_unknown_metadata_works() {
    const TEST_NUM: u128 = 18_446_744_073_709_551_617;
    deserialize_unknown_metadata_test!(U128, TEST_NUM);
}
#[test]
fn u128_deserialize_known_metadata_works() {
    const TEST_NUM: u128 = 18_446_744_073_709_551_617;
    deserialize_known_metadata_test!(U128, TEST_NUM);
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
#[should_panic(expected = "not implemented")]
fn u256_deserialize_unknown_metadata_works() {
    let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::U256 as u8]);
    XdlPrimitive::deserialize_unknown_metadata(&mut reader).unwrap();
}
#[test]
#[should_panic(expected = "not implemented")]
fn u256_deserialize_known_metadata_works() {
    let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::U256 as u8]);
    XdlPrimitive::deserialize_unknown_metadata(&mut reader).unwrap();
}

#[test]
fn i8_serialize_with_metadata_works() {
    const TEST_NUM: i8 = 42;
    serialize_with_metadata_test!(I8, TEST_NUM);
}
#[test]
fn i8_serialize_without_metadata_works() {
    const TEST_NUM: i8 = 42;
    serialize_without_metadata_test!(I8, TEST_NUM);
}
#[test]
fn i8_deserialize_unknown_metadata_works() {
    const TEST_NUM: i8 = 42;
    deserialize_unknown_metadata_test!(I8, TEST_NUM);
}
#[test]
fn i8_deserialize_known_metadata_works() {
    const TEST_NUM: i8 = 42;
    deserialize_known_metadata_test!(I8, TEST_NUM);
}

#[test]
fn i16_serialize_with_metadata_works() {
    const TEST_NUM: i16 = 420;
    serialize_with_metadata_test!(I16, TEST_NUM);
}
#[test]
fn i16_serialize_without_metadata_works() {
    const TEST_NUM: i16 = 420;
    serialize_without_metadata_test!(I16, TEST_NUM);
}
#[test]
fn i16_deserialize_unknown_metadata_works() {
    const TEST_NUM: i16 = 420;
    deserialize_unknown_metadata_test!(I16, TEST_NUM);
}
#[test]
fn i16_deserialize_known_metadata_works() {
    const TEST_NUM: i16 = 420;
    deserialize_known_metadata_test!(I16, TEST_NUM);
}

#[test]
fn i32_serialize_with_metadata_works() {
    const TEST_NUM: i32 = 100_000;
    serialize_with_metadata_test!(I32, TEST_NUM);
}
#[test]
fn i32_serialize_without_metadata_works() {
    const TEST_NUM: i32 = 100_000;
    serialize_without_metadata_test!(I32, TEST_NUM);
}
#[test]
fn i32_deserialize_unknown_metadata_works() {
    const TEST_NUM: i32 = 100_000;
    deserialize_unknown_metadata_test!(I32, TEST_NUM);
}
#[test]
fn i32_deserialize_known_metadata_works() {
    const TEST_NUM: i32 = 100_000;
    deserialize_known_metadata_test!(I32, TEST_NUM);
}

#[test]
fn i64_serialize_with_metadata_works() {
    const TEST_NUM: i64 = 100_000_000;
    serialize_with_metadata_test!(I64, TEST_NUM);
}
#[test]
fn i64_serialize_without_metadata_works() {
    const TEST_NUM: i64 = 100_000_000;
    serialize_without_metadata_test!(I64, TEST_NUM);
}
#[test]
fn i64_deserialize_unknown_metadata_works() {
    const TEST_NUM: i64 = 100_000_000;
    deserialize_unknown_metadata_test!(I64, TEST_NUM);
}
#[test]
fn i64_deserialize_known_metadata_works() {
    const TEST_NUM: i64 = 100_000_000;
    deserialize_known_metadata_test!(I64, TEST_NUM);
}

#[test]
fn i128_serialize_with_metadata_works() {
    const TEST_NUM: i128 = 18_446_744_073_709_551_617;
    serialize_with_metadata_test!(I128, TEST_NUM);
}
#[test]
fn i128_serialize_without_metadata_works() {
    const TEST_NUM: i128 = 18_446_744_073_709_551_617;
    serialize_without_metadata_test!(I128, TEST_NUM);
}
#[test]
fn i128_deserialize_unknown_metadata_works() {
    const TEST_NUM: i128 = 18_446_744_073_709_551_617;
    deserialize_unknown_metadata_test!(I128, TEST_NUM);
}
#[test]
fn i128_deserialize_known_metadata_works() {
    const TEST_NUM: i128 = 18_446_744_073_709_551_617;
    deserialize_known_metadata_test!(I128, TEST_NUM);
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
#[should_panic(expected = "not implemented")]
fn i256_deserialize_unknown_metadata_works() {
    let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::I256 as u8]);
    XdlPrimitive::deserialize_unknown_metadata(&mut reader).unwrap();
}
#[test]
#[should_panic(expected = "not implemented")]
fn i256_deserialize_known_metadata_works() {
    let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::I256 as u8]);
    XdlPrimitive::deserialize_unknown_metadata(&mut reader).unwrap();
}

#[test]
fn f32_serialize_with_metadata_works() {
    const TEST_NUM: f32 = 69.0;
    serialize_with_metadata_test!(F32, TEST_NUM);
}
#[test]
fn f32_serialize_without_metadata_works() {
    const TEST_NUM: f32 = 69.0;
    serialize_without_metadata_test!(F32, TEST_NUM);
}
#[test]
fn f32_deserialize_unknown_metadata_works() {
    const TEST_NUM: f32 = 69.0;
    deserialize_unknown_metadata_test!(F32, TEST_NUM);
}
#[test]
fn f32_deserialize_known_metadata_works() {
    const TEST_NUM: f32 = 69.0;
    deserialize_known_metadata_test!(F32, TEST_NUM);
}

#[test]
fn f64_serialize_with_metadata_works() {
    const TEST_NUM: f64 = 69.0;
    serialize_with_metadata_test!(F64, TEST_NUM);
}
#[test]
fn f64_serialize_without_metadata_works() {
    const TEST_NUM: f64 = 69.0;
    serialize_without_metadata_test!(F64, TEST_NUM);
}
#[test]
fn f64_deserialize_unknown_metadata_works() {
    const TEST_NUM: f64 = 69.0;
    deserialize_unknown_metadata_test!(F64, TEST_NUM);
}
#[test]
fn f64_deserialize_known_metadata_works() {
    const TEST_NUM: f64 = 69.0;
    deserialize_known_metadata_test!(F64, TEST_NUM);
}

#[test]
fn string_serialize_with_metadata_works() {
    let test_string = "hello world".to_string();
    let primitive = XdlPrimitive::String(test_string.clone());

    let mut writer = Vec::new();
    primitive.serialize_with_metadata(&mut writer).unwrap();

    // id of the type
    let mut expected = vec![XdlPrimitiveMetadata::from(&primitive) as u8];
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
