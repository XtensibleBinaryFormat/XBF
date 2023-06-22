use crate::{DeserializeType, Serialize, XbfMetadata, XbfType};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct XbfVec {
    pub(crate) inner_type: Box<XbfMetadata>,
    elements: Vec<XbfType>,
}

impl Serialize for XbfVec {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u16::<LittleEndian>(self.elements.len() as u16)?;
        self.elements.iter().try_for_each(|e| e.serialize(writer))
    }
}

impl DeserializeType for XbfVec {
    fn deserialize_type(metadata: &XbfMetadata, reader: &mut impl Read) -> io::Result<XbfType> {
        let len = reader.read_u16::<LittleEndian>()? as usize;
        let mut elements = Vec::with_capacity(len);
        for _ in 0..len {
            elements.push(XbfType::deserialize_type(metadata, reader)?);
        }
        Ok(XbfType::Vec(
            XbfVec::new(metadata.clone(), elements).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "All elements are not the same type",
                )
            })?,
        ))
    }
}

impl XbfVec {
    pub fn new(
        inner_type: XbfMetadata,
        elements: Vec<XbfType>,
    ) -> Result<Self, ElementsNotHomogenousError> {
        let all_same_type = elements.iter().all(|x| inner_type == x.into());
        if all_same_type {
            Ok(Self {
                inner_type: Box::new(inner_type),
                elements,
            })
        } else {
            Err(ElementsNotHomogenousError)
        }
    }

    pub fn new_unchecked(inner_type: XbfMetadata, elements: Vec<XbfType>) -> Self {
        Self {
            inner_type: Box::new(inner_type),
            elements,
        }
    }
}

#[derive(Debug)]
pub struct ElementsNotHomogenousError;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        xbf_primitive::{XbfPrimitive, XbfPrimitiveMetadata},
        xbf_vec::XbfVecMetadata,
    };
    use std::io::Cursor;

    #[test]
    fn serialize_vec_primitive_works() {
        const TEST_NUM: i32 = 42;
        let vec = XbfVec::new(
            XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
            vec![XbfType::Primitive(XbfPrimitive::I32(TEST_NUM))],
        )
        .unwrap();
        let mut writer = vec![];

        vec.serialize(&mut writer).unwrap();

        let mut expected = vec![];
        expected.extend_from_slice(&1u16.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());

        assert_eq!(writer, expected);
    }

    #[test]
    fn serialize_vec_of_vec_works() {
        const TEST_NUM: i32 = 42;
        let vec_of_two_i32 = XbfVec::new(
            XbfPrimitiveMetadata::I32.into(),
            vec![
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
            ],
        )
        .unwrap();
        let vec_of_i32_metadata: XbfVecMetadata = (&vec_of_two_i32).into();
        let vec_of_vec_of_i32 = XbfVec::new_unchecked(
            vec_of_i32_metadata.into(),
            vec![vec_of_two_i32.clone().into(), vec_of_two_i32.clone().into()],
        );

        let mut writer = vec![];

        vec_of_vec_of_i32.serialize(&mut writer).unwrap();

        let mut expected = vec![];
        expected.extend_from_slice(&2u16.to_le_bytes());
        expected.extend_from_slice(&2u16.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());
        expected.extend_from_slice(&2u16.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());

        assert_eq!(writer, expected);
    }

    #[test]
    fn deserialize_vec_primitive_works() {
        const TEST_NUM: i32 = 42;
        let mut data = vec![];
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        let mut reader = Cursor::new(data);

        let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32.into());
        let expected = XbfVec::new(
            XbfPrimitiveMetadata::I32.into(),
            vec![XbfType::Primitive(XbfPrimitive::I32(TEST_NUM))],
        )
        .unwrap();

        let vec = XbfType::deserialize_type(&(metadata.into()), &mut reader).unwrap();

        assert_eq!(vec, expected.into());
    }

    #[test]
    fn deserialize_vec_of_vec_works() {
        const TEST_NUM: i32 = 42;
        let mut data = vec![];
        data.extend_from_slice(&2u16.to_le_bytes());
        data.extend_from_slice(&2u16.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        data.extend_from_slice(&2u16.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        let mut reader = Cursor::new(data);

        let inner_integer_metadata = XbfPrimitiveMetadata::I32;
        let inner_vec_metadata = XbfVecMetadata::new(inner_integer_metadata.into());

        let metadata = XbfVecMetadata::new(inner_vec_metadata.clone().into());
        let expected_inner_vec = XbfVec::new(
            inner_integer_metadata.clone().into(),
            vec![
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
            ],
        )
        .unwrap();
        let expected = XbfVec::new(
            inner_vec_metadata.into(),
            vec![
                expected_inner_vec.clone().into(),
                expected_inner_vec.clone().into(),
            ],
        )
        .unwrap();

        let vec = XbfType::deserialize_type(&(metadata.into()), &mut reader).unwrap();

        assert_eq!(vec, expected.into());
    }
}