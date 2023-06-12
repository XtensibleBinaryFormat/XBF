use crate::{DeserializeType, Serialize, XdlMetadata, XdlType};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct XdlVec {
    pub(crate) inner_type: Box<XdlMetadata>,
    elements: Vec<XdlType>,
}

impl Serialize for XdlVec {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u16::<LittleEndian>(self.elements.len() as u16)?;
        self.elements.iter().try_for_each(|e| e.serialize(writer))
    }
}

impl DeserializeType for XdlVec {
    fn deserialize_type(metadata: &XdlMetadata, reader: &mut impl Read) -> io::Result<XdlType> {
        let len = reader.read_u16::<LittleEndian>()? as usize;
        let mut elements = Vec::with_capacity(len);
        for _ in 0..len {
            elements.push(XdlType::deserialize_type(metadata, reader)?);
        }
        Ok(XdlType::Vec(
            XdlVec::new(metadata.clone(), elements).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "All elements are not the same type",
                )
            })?,
        ))
    }
}

impl XdlVec {
    pub fn new(
        inner_type: XdlMetadata,
        elements: Vec<XdlType>,
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

    pub fn new_unchecked(inner_type: XdlMetadata, elements: Vec<XdlType>) -> Self {
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
    use crate::{
        xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata},
        xdl_vec::XdlVecMetadata,
    };

    use super::*;

    #[test]
    fn serialize_vec_primitive_works() {
        const TEST_NUM: i32 = 42;
        let vec = XdlVec::new(
            XdlMetadata::Primitive(XdlPrimitiveMetadata::I32),
            vec![XdlType::Primitive(XdlPrimitive::I32(TEST_NUM))],
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
        let vec_of_two_i32 = XdlVec::new(
            XdlPrimitiveMetadata::I32.into(),
            vec![
                XdlType::Primitive(XdlPrimitive::I32(TEST_NUM)),
                XdlType::Primitive(XdlPrimitive::I32(TEST_NUM)),
            ],
        )
        .unwrap();
        let vec_of_i32_metadata: XdlVecMetadata = (&vec_of_two_i32).into();
        let vec_of_vec_of_i32 = XdlVec::new_unchecked(
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
}
