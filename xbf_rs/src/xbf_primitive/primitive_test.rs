use super::*;
use crate::{DeserializeType, Serialize, XbfMetadata, XbfType, XbfTypeUpcast};
use std::io::Cursor;

macro_rules! serialize_primitive_test {
    ($xbf_type:tt, $test_num:expr) => {
        let primitive = XbfPrimitive::$xbf_type($test_num);
        let mut writer = Vec::new();

        primitive.serialize(&mut writer).unwrap();

        let expected = $test_num.to_le_bytes();
        assert_eq!(writer, expected);
    };
}

macro_rules! deserialize_primitive_test {
    ($xbf_type:tt, $test_num:expr) => {
        let data = $test_num.to_le_bytes();
        let mut reader = Cursor::new(data);

        let metadata = XbfMetadata::Primitive(XbfPrimitiveMetadata::$xbf_type);
        let expected = XbfType::Primitive(XbfPrimitive::$xbf_type($test_num));

        let primitive = XbfType::deserialize_type(&metadata, &mut reader).unwrap();
        assert_eq!(primitive, expected);
    };
}

#[test]
fn bool_serialize_works() {
    let xbf_true = XbfPrimitive::Bool(true);
    let xbf_false = XbfPrimitive::Bool(false);
    let mut writer = Vec::new();

    xbf_true.serialize(&mut writer).unwrap();
    xbf_false.serialize(&mut writer).unwrap();

    assert_eq!(writer, vec![1, 0]);
}
#[test]
fn bool_deserialize_works() {
    let data = vec![1, 0];
    let mut reader = Cursor::new(data);

    let metadata = XbfMetadata::Primitive(XbfPrimitiveMetadata::Bool);

    let true_type = XbfType::deserialize_type(&metadata, &mut reader).unwrap();
    assert_eq!(true_type, XbfType::Primitive(XbfPrimitive::Bool(true)));

    let false_type = XbfType::deserialize_type(&metadata, &mut reader).unwrap();
    assert_eq!(false_type, XbfType::Primitive(XbfPrimitive::Bool(false)));
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
fn u256_serde_works() {
    const TEST_NUM: [u64; 4] = [1, 2, 3, 4];
    let primitive = XbfPrimitive::U256(TEST_NUM);
    let mut writer = Vec::new();

    primitive.serialize(&mut writer).unwrap();

    let expected = TEST_NUM
        .iter()
        .flat_map(|x| x.to_le_bytes())
        .collect::<Vec<_>>();
    assert_eq!(writer, expected);

    let mut reader = Cursor::new(writer);
    let deserialized = XbfType::deserialize_type(
        &XbfMetadata::Primitive(XbfPrimitiveMetadata::U256),
        &mut reader,
    )
    .unwrap();
    assert_eq!(deserialized, primitive.to_base_type());
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
fn i256_serde_works() {
    const TEST_NUM: [u64; 4] = [1, 2, 3, 4];
    let primitive = XdlPrimitive::I256(TEST_NUM);
    let mut writer = Vec::new();

    primitive.serialize(&mut writer).unwrap();

    let expected = TEST_NUM
        .iter()
        .flat_map(|x| x.to_le_bytes())
        .collect::<Vec<_>>();
    assert_eq!(writer, expected);

    let mut reader = Cursor::new(writer);
    let deserialized = XbfType::deserialize_type(
        &XbfMetadata::Primitive(XbfPrimitiveMetadata::I256),
        &mut reader,
    )
    .unwrap();
    assert_eq!(deserialized, primitive.to_base_type());
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
    let primitive = XbfPrimitive::String(test_string.clone());
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

    let metadata = XbfMetadata::Primitive(XbfPrimitiveMetadata::String);
    let expected = XbfType::Primitive(XbfPrimitive::String(test_string.clone()));

    let primitive = XbfType::deserialize_type(&metadata, &mut reader).unwrap();
    assert_eq!(primitive, expected);
}
