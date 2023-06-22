use super::*;
use crate::{XbfMetadata, XbfType};
use std::io::Cursor;

macro_rules! serialize_primitive_test {
    ($xbf_type:tt, $test_num:expr) => {
        let primitive = XbfPrimitive::$xbf_type($test_num);
        let mut writer = Vec::new();

        primitive.serialize_primitive_type(&mut writer).unwrap();

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

        let primitive = XbfType::deserialize_base_type(&metadata, &mut reader).unwrap();
        assert_eq!(primitive, expected);
    };
}

#[test]
fn bool_serialize_works() {
    let xbf_true = XbfPrimitive::Bool(true);
    let xbf_false = XbfPrimitive::Bool(false);
    let mut writer = Vec::new();

    xbf_true.serialize_primitive_type(&mut writer).unwrap();
    xbf_false.serialize_primitive_type(&mut writer).unwrap();

    assert_eq!(writer, vec![1, 0]);
}
#[test]
fn bool_deserialize_works() {
    let data = vec![1, 0];
    let mut reader = Cursor::new(data);

    let metadata = XbfMetadata::Primitive(XbfPrimitiveMetadata::Bool);

    let true_type = XbfType::deserialize_base_type(&metadata, &mut reader).unwrap();
    assert_eq!(true_type, XbfType::Primitive(XbfPrimitive::Bool(true)));

    let false_type = XbfType::deserialize_base_type(&metadata, &mut reader).unwrap();
    assert_eq!(false_type, XbfType::Primitive(XbfPrimitive::Bool(false)));
}

#[test]
fn unsigned_nums_serde_works() {
    let test_num: u8 = 42;
    serialize_primitive_test!(U8, test_num);
    deserialize_primitive_test!(U8, test_num);

    let test_num: u16 = 420;
    serialize_primitive_test!(U16, test_num);
    deserialize_primitive_test!(U16, test_num);

    let test_num: u32 = 100_000;
    serialize_primitive_test!(U32, test_num);
    deserialize_primitive_test!(U32, test_num);

    let test_num: u64 = 100_000_000;
    serialize_primitive_test!(U64, test_num);
    deserialize_primitive_test!(U64, test_num);

    let test_num: u128 = 18_446_744_073_709_551_617;
    serialize_primitive_test!(U128, test_num);
    deserialize_primitive_test!(U128, test_num);
}

#[test]
#[should_panic(expected = "not implemented")]
fn u256_serialize_works() {
    let dne = XbfPrimitive::U256(());
    dne.serialize_primitive_type(&mut Vec::new()).unwrap();
}
#[test]
#[should_panic(expected = "not implemented")]
fn u256_deserialize_works() {
    let mut reader = Cursor::new(vec![XbfPrimitiveMetadata::U256 as u8]);
    XbfType::deserialize_base_type(
        &XbfMetadata::Primitive(XbfPrimitiveMetadata::U256),
        &mut reader,
    )
    .unwrap();
}

#[test]
fn signed_nums_serde_works() {
    let test_num: i8 = 42;
    serialize_primitive_test!(I8, test_num);
    deserialize_primitive_test!(I8, test_num);

    let test_num: i16 = 420;
    serialize_primitive_test!(I16, test_num);
    deserialize_primitive_test!(I16, test_num);

    let test_num: i32 = 100_000;
    serialize_primitive_test!(I32, test_num);
    deserialize_primitive_test!(I32, test_num);

    let test_num: i64 = 100_000_000;
    serialize_primitive_test!(I64, test_num);
    deserialize_primitive_test!(I64, test_num);

    let test_num: i128 = 18_446_744_073_709_551_617;
    serialize_primitive_test!(I128, test_num);
    deserialize_primitive_test!(I128, test_num);
}

#[test]
#[should_panic(expected = "not implemented")]
fn i256_serialize_works() {
    let dne = XbfPrimitive::I256(());
    dne.serialize_primitive_type(&mut Vec::new()).unwrap();
}

#[test]
#[should_panic(expected = "not implemented")]
fn i256_deserialize_works() {
    let mut reader = Cursor::new(vec![XbfPrimitiveMetadata::I256 as u8]);
    XbfType::deserialize_base_type(
        &XbfMetadata::Primitive(XbfPrimitiveMetadata::I256),
        &mut reader,
    )
    .unwrap();
}

#[test]
fn floating_point_serde_works() {
    let test_num: f32 = 69.0;
    serialize_primitive_test!(F32, test_num);
    deserialize_primitive_test!(F32, test_num);

    let test_num: f64 = 69.0;
    serialize_primitive_test!(F64, test_num);
    deserialize_primitive_test!(F64, test_num);
}

#[test]
fn string_serde_works() {
    let test_string = "hello world".to_string();
    let primitive = XbfPrimitive::String(test_string.clone());
    let mut writer = vec![];

    primitive.serialize_primitive_type(&mut writer).unwrap();

    let mut expected_writer = vec![];
    expected_writer.extend_from_slice(&(test_string.len() as u16).to_le_bytes());
    expected_writer.extend_from_slice(test_string.as_bytes());

    assert_eq!(writer, expected_writer);

    let mut reader = Cursor::new(writer);
    let deserialized = XbfType::deserialize_base_type(
        &XbfMetadata::Primitive(XbfPrimitiveMetadata::String),
        &mut reader,
    )
    .unwrap();

    assert_eq!(
        deserialized,
        XbfType::Primitive(XbfPrimitive::String(test_string))
    );
}
