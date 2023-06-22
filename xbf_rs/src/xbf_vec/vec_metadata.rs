use crate::{xbf_primitive::XbfPrimitiveMetadata, DeserializeMetadata, Serialize, XbfMetadata};
use byteorder::WriteBytesExt;
use std::io::{self, Write};

pub const VEC_METADATA_DISCRIMINANT: u8 = XbfPrimitiveMetadata::String as u8 + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbfVecMetadata {
    pub(crate) inner_type: Box<XbfMetadata>,
}

impl XbfVecMetadata {
    pub fn new(inner_type: XbfMetadata) -> Self {
        Self {
            inner_type: Box::new(inner_type),
        }
    }

    pub fn from_boxed_type(inner_type: Box<XbfMetadata>) -> Self {
        Self { inner_type }
    }
}

impl Serialize for XbfVecMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(VEC_METADATA_DISCRIMINANT)?;
        self.inner_type.serialize(writer)
    }
}

impl DeserializeMetadata for XbfVecMetadata {
    fn deserialize_metadata(reader: &mut impl io::Read) -> io::Result<XbfMetadata> {
        let inner_type = XbfMetadata::deserialize_metadata(reader)?;
        Ok(XbfVecMetadata::new(inner_type).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use io::Cursor;

    #[test]
    fn primitive_metadata_serialize_works() {
        let vec_i32_metadata =
            XbfVecMetadata::new(XbfMetadata::Primitive(XbfPrimitiveMetadata::I32));
        let vec_string_metadata = XbfVecMetadata::from_boxed_type(Box::new(
            XbfMetadata::Primitive(XbfPrimitiveMetadata::String),
        ));
        let mut writer = vec![];

        vec_i32_metadata.serialize(&mut writer).unwrap();
        vec_string_metadata.serialize(&mut writer).unwrap();

        assert_eq!(
            writer,
            vec![
                VEC_METADATA_DISCRIMINANT,
                XbfPrimitiveMetadata::I32 as u8,
                VEC_METADATA_DISCRIMINANT,
                XbfPrimitiveMetadata::String as u8
            ]
        );
    }

    #[test]
    fn primitive_metadata_deserialize_works() {
        let data = vec![
            VEC_METADATA_DISCRIMINANT,
            XbfPrimitiveMetadata::I32 as u8,
            VEC_METADATA_DISCRIMINANT,
            XbfPrimitiveMetadata::String as u8,
        ];
        let mut reader = Cursor::new(data);

        let vec_i32_metadata = XbfMetadata::deserialize_metadata(&mut reader).unwrap();
        let vec_string_metadata = XbfMetadata::deserialize_metadata(&mut reader).unwrap();

        assert_eq!(
            vec_i32_metadata,
            XbfMetadata::Vec(XbfVecMetadata::new(XbfMetadata::Primitive(
                XbfPrimitiveMetadata::I32
            )))
        );
        assert_eq!(
            vec_string_metadata,
            XbfMetadata::Vec(XbfVecMetadata::new(XbfMetadata::Primitive(
                XbfPrimitiveMetadata::String
            )))
        )
    }

    #[test]
    fn nested_vec_metadata_serialize_works() {
        let vec_vec_i32_metadata = XbfVecMetadata {
            inner_type: Box::new(XbfMetadata::Vec(XbfVecMetadata {
                inner_type: Box::new(XbfMetadata::Primitive(XbfPrimitiveMetadata::I32)),
            })),
        };
        let mut writer = vec![];

        vec_vec_i32_metadata.serialize(&mut writer).unwrap();

        assert_eq!(
            writer,
            vec![
                VEC_METADATA_DISCRIMINANT,
                VEC_METADATA_DISCRIMINANT,
                XbfPrimitiveMetadata::I32 as u8
            ]
        )
    }

    #[test]
    fn nested_vec_metadata_deserialize_works() {
        let data = vec![
            VEC_METADATA_DISCRIMINANT,
            VEC_METADATA_DISCRIMINANT,
            XbfPrimitiveMetadata::I32 as u8,
        ];
        let mut reader = Cursor::new(data);

        let vec_vec_i32_metadata = XbfMetadata::deserialize_metadata(&mut reader).unwrap();

        let expected_metadata = XbfMetadata::Vec(XbfVecMetadata::new(XbfMetadata::Vec(
            XbfVecMetadata::new(XbfMetadata::Primitive(XbfPrimitiveMetadata::I32)),
        )));

        assert_eq!(vec_vec_i32_metadata, expected_metadata);
    }
}
