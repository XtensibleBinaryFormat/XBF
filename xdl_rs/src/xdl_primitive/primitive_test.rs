use super::*;
use crate::{DeserializeType, Serialize, XdlMetadata, XdlType};
use std::io::Cursor;

macro_rules! serialize_primitive_test {
    ($xdl_type:tt, $test_num:expr) => {
        let primitive = XdlPrimitive::$xdl_type($test_num);
        let mut writer = Vec::new();

        primitive.serialize(&mut writer).unwrap();

        let expected = $test_num.to_le_bytes();
        assert_eq!(writer, expected);
    };
}

macro_rules! deserialize_primitive_test {
    ($xdl_type:tt, $test_num:expr) => {
        let data = $test_num.to_le_bytes();
        let mut reader = Cursor::new(data);

        let metadata = XdlMetadata::Primitive(XdlPrimitiveMetadata::$xdl_type);
        let expected = XdlType::Primitive(XdlPrimitive::$xdl_type($test_num));

        let primitive = XdlType::deserialize_type(&metadata, &mut reader).unwrap();
        assert_eq!(primitive, expected);
    };
}

#[test]
fn bool_serialize_works() {
    let xdl_true = XdlPrimitive::Bool(true);
    let xdl_false = XdlPrimitive::Bool(false);
    let mut writer = Vec::new();

    xdl_true.serialize(&mut writer).unwrap();
    xdl_false.serialize(&mut writer).unwrap();

    assert_eq!(writer, vec![1, 0]);
}
#[test]
fn bool_deserialize_works() {
    let data = vec![1, 0];
    let mut reader = Cursor::new(data);

    let metadata = XdlMetadata::Primitive(XdlPrimitiveMetadata::Bool);

    let true_type = XdlType::deserialize_type(&metadata, &mut reader).unwrap();
    assert_eq!(true_type, XdlType::Primitive(XdlPrimitive::Bool(true)));

    let false_type = XdlType::deserialize_type(&metadata, &mut reader).unwrap();
    assert_eq!(false_type, XdlType::Primitive(XdlPrimitive::Bool(false)));
}

#[test]
fn u8_serde_works() {
    const TEST_NUM: u8 = 42;
    serialize_primitive_test!(U8, TEST_NUM);
    deserialize_primitive_test!(U8, TEST_NUM);
}

#[test]
fn u16_serde_works() {
    const TEST_NUM: u16 = 420;
    serialize_primitive_test!(U16, TEST_NUM);
    deserialize_primitive_test!(U16, TEST_NUM);
}

#[test]
fn u32_serde_works() {
    const TEST_NUM: u32 = 100_000;
    serialize_primitive_test!(U32, TEST_NUM);
    deserialize_primitive_test!(U32, TEST_NUM);
}

#[test]
fn u64_serde_works() {
    const TEST_NUM: u64 = 100_000_000;
    serialize_primitive_test!(U64, TEST_NUM);
    deserialize_primitive_test!(U64, TEST_NUM);
}

#[test]
fn u128_serde_works() {
    const TEST_NUM: u128 = 18_446_744_073_709_551_617;
    serialize_primitive_test!(U128, TEST_NUM);
    deserialize_primitive_test!(U128, TEST_NUM);
}

#[test]
#[should_panic(expected = "not implemented")]
fn u256_serialize_works() {
    let dne = XdlPrimitive::U256(());
    dne.serialize(&mut Vec::new()).unwrap();
}
#[test]
#[should_panic(expected = "not implemented")]
fn u256_deserialize_works() {
    let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::U256 as u8]);
    XdlType::deserialize_type(
        &XdlMetadata::Primitive(XdlPrimitiveMetadata::U256),
        &mut reader,
    )
    .unwrap();
}

#[test]
fn i8_serde_works() {
    const TEST_NUM: i8 = 42;
    serialize_primitive_test!(I8, TEST_NUM);
    deserialize_primitive_test!(I8, TEST_NUM);
}

#[test]
fn i16_serde_works() {
    const TEST_NUM: i16 = 420;
    serialize_primitive_test!(I16, TEST_NUM);
    deserialize_primitive_test!(I16, TEST_NUM);
}

#[test]
fn i32_serde_works() {
    const TEST_NUM: i32 = 100_000;
    serialize_primitive_test!(I32, TEST_NUM);
    deserialize_primitive_test!(I32, TEST_NUM);
}

#[test]
fn i64_serde_works() {
    const TEST_NUM: i64 = 100_000_000;
    serialize_primitive_test!(I64, TEST_NUM);
    deserialize_primitive_test!(I64, TEST_NUM);
}

#[test]
fn i128_serde_works() {
    const TEST_NUM: i128 = 18_446_744_073_709_551_617;
    serialize_primitive_test!(I128, TEST_NUM);
    deserialize_primitive_test!(I128, TEST_NUM);
}

#[test]
#[should_panic(expected = "not implemented")]
fn i256_serialize_works() {
    let dne = XdlPrimitive::I256(());
    dne.serialize(&mut Vec::new()).unwrap();
}
#[test]
#[should_panic(expected = "not implemented")]
fn i256_deserialize_works() {
    let mut reader = Cursor::new(vec![XdlPrimitiveMetadata::I256 as u8]);
    XdlType::deserialize_type(
        &XdlMetadata::Primitive(XdlPrimitiveMetadata::I256),
        &mut reader,
    )
    .unwrap();
}

#[test]
fn f32_serde_works() {
    const TEST_NUM: f32 = 69.0;
    serialize_primitive_test!(F32, TEST_NUM);
    deserialize_primitive_test!(F32, TEST_NUM);
}

#[test]
fn f64_serde_works() {
    const TEST_NUM: f64 = 69.0;
    serialize_primitive_test!(F64, TEST_NUM);
    deserialize_primitive_test!(F64, TEST_NUM);
}

#[test]
fn string_serialize_works() {
    let test_string = "hello world".to_string();
    let primitive = XdlPrimitive::String(test_string.clone());
    let mut writer = vec![];

    primitive.serialize(&mut writer).unwrap();

    let mut expected = vec![];
    expected.extend_from_slice(&(test_string.len() as u16).to_le_bytes());
    expected.extend_from_slice(test_string.as_bytes());

    assert_eq!(writer, expected);
}
#[test]
fn string_deserialize_works() {
    let test_string = "hello world".to_string();
    let mut data = (test_string.len() as u16).to_le_bytes().to_vec();
    data.extend_from_slice(test_string.as_bytes());

    let mut reader = Cursor::new(data);

    let metadata = XdlMetadata::Primitive(XdlPrimitiveMetadata::String);
    let expected = XdlType::Primitive(XdlPrimitive::String(test_string.clone()));

    let primitive = XdlType::deserialize_type(&metadata, &mut reader).unwrap();
    assert_eq!(primitive, expected);
}

#[test]
fn test_xdl_primitive_from_bool() {
    let value: bool = true;
    let primitive: XdlPrimitive = value.into();
    assert_eq!(primitive, XdlPrimitive::Bool(true));
}

#[test]
fn test_xdl_primitive_from_u8() {
    let value: u8 = 42;
    let primitive: XdlPrimitive = value.into();
    assert_eq!(primitive, XdlPrimitive::U8(42));
}

// Add similar tests for other primitive types...

#[test]
fn test_xdl_primitive_from_string() {
    let value: String = "Hello, World!".to_string();
    let primitive: XdlPrimitive = value.into();
    assert_eq!(primitive, XdlPrimitive::String("Hello, World!".to_string()));
}
