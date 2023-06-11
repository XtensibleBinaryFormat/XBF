use crate::{DeserializeType, Serialize, XdlMetadata, XdlType};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct XdlVec {
    pub(super) inner_type: Box<XdlMetadata>,
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

pub struct ElementsNotHomogenousError;
