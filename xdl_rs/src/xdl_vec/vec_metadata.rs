use super::XdlVec;
use crate::{xdl_primitive::XdlPrimitiveMetadata, Serialize, XdlMetadata};
use byteorder::WriteBytesExt;
use std::io::{self, Write};

const VEC_METADATA_DISCRIMINANT: u8 = XdlPrimitiveMetadata::String as u8 + 1;

#[derive(Debug, Clone, PartialEq)]
pub struct XdlVecMetadata {
    inner_type: Box<XdlMetadata>,
}

impl Serialize for XdlVecMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(VEC_METADATA_DISCRIMINANT)?;
        self.inner_type.serialize(writer)
    }
}

impl XdlVecMetadata {
    pub fn new(inner_type: XdlMetadata) -> Self {
        Self {
            inner_type: Box::new(inner_type),
        }
    }

    pub fn from_boxed_type(inner_type: Box<XdlMetadata>) -> Self {
        Self { inner_type }
    }
}

impl From<&XdlVec> for XdlVecMetadata {
    fn from(value: &XdlVec) -> Self {
        Self::from_boxed_type(value.inner_type.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primitive_metadata_works() {
        let vec_i32_metadata =
            XdlVecMetadata::new(XdlMetadata::Primitive(XdlPrimitiveMetadata::I32));
        let vec_string_metadata = XdlVecMetadata::from_boxed_type(Box::new(
            XdlMetadata::Primitive(XdlPrimitiveMetadata::String),
        ));
        let mut writer = vec![];

        vec_i32_metadata.serialize(&mut writer).unwrap();
        vec_string_metadata.serialize(&mut writer).unwrap();

        assert_eq!(
            writer,
            vec![
                VEC_METADATA_DISCRIMINANT,
                XdlPrimitiveMetadata::I32 as u8,
                VEC_METADATA_DISCRIMINANT,
                XdlPrimitiveMetadata::String as u8
            ]
        );
    }

    #[test]
    fn nested_vec_metadata_works() {
        let vec_vec_i32_metadata = XdlVecMetadata {
            inner_type: Box::new(XdlMetadata::Vec(XdlVecMetadata {
                inner_type: Box::new(XdlMetadata::Primitive(XdlPrimitiveMetadata::I32)),
            })),
        };
        let mut writer = vec![];

        vec_vec_i32_metadata.serialize(&mut writer).unwrap();

        assert_eq!(
            writer,
            vec![
                VEC_METADATA_DISCRIMINANT,
                VEC_METADATA_DISCRIMINANT,
                XdlPrimitiveMetadata::I32 as u8
            ]
        )
    }
}
