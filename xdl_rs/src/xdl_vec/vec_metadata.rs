use crate::{xdl_primitive::XdlPrimitiveMetadata, DeserializeMetadata, Serialize, XdlMetadata};
use byteorder::WriteBytesExt;
use std::io::{self, Write};

pub const VEC_METADATA_DISCRIMINANT: u8 = XdlPrimitiveMetadata::String as u8 + 1;

#[derive(Debug, Clone, PartialEq)]
pub struct XdlVecMetadata {
    pub(crate) inner_type: Box<XdlMetadata>,
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

impl Serialize for XdlVecMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(VEC_METADATA_DISCRIMINANT)?;
        self.inner_type.serialize(writer)
    }
}

impl DeserializeMetadata for XdlVecMetadata {
    fn deserialize_metadata(reader: &mut impl io::Read) -> io::Result<XdlMetadata> {
        let inner_type = XdlMetadata::deserialize_metadata(reader)?;
        Ok(XdlMetadata::Vec(XdlVecMetadata::new(inner_type)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use io::Cursor;

    #[test]
    fn primitive_metadata_serialize_works() {
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
    fn primitive_metadata_deserialize_works() {
        let data = vec![
            VEC_METADATA_DISCRIMINANT,
            XdlPrimitiveMetadata::I32 as u8,
            VEC_METADATA_DISCRIMINANT,
            XdlPrimitiveMetadata::String as u8,
        ];
        let mut reader = Cursor::new(data);

        let vec_i32_metadata = XdlMetadata::deserialize_metadata(&mut reader).unwrap();
        let vec_string_metadata = XdlMetadata::deserialize_metadata(&mut reader).unwrap();

        assert_eq!(
            vec_i32_metadata,
            XdlMetadata::Vec(XdlVecMetadata::new(XdlMetadata::Primitive(
                XdlPrimitiveMetadata::I32
            )))
        );
        assert_eq!(
            vec_string_metadata,
            XdlMetadata::Vec(XdlVecMetadata::new(XdlMetadata::Primitive(
                XdlPrimitiveMetadata::String
            )))
        )
    }

    #[test]
    fn nested_vec_metadata_serialize_works() {
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

    #[test]
    fn nested_vec_metadata_deserialize_works() {
        let data = vec![
            VEC_METADATA_DISCRIMINANT,
            VEC_METADATA_DISCRIMINANT,
            XdlPrimitiveMetadata::I32 as u8,
        ];
        let mut reader = Cursor::new(data);

        let vec_vec_i32_metadata = XdlMetadata::deserialize_metadata(&mut reader).unwrap();

        let expected_metadata = XdlMetadata::Vec(XdlVecMetadata::new(XdlMetadata::Vec(
            XdlVecMetadata::new(XdlMetadata::Primitive(XdlPrimitiveMetadata::I32)),
        )));

        assert_eq!(vec_vec_i32_metadata, expected_metadata);
    }
}
