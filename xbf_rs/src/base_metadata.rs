use crate::{
    xbf_primitive::XbfPrimitiveMetadata,
    xbf_struct::{XbfStructMetadata, STRUCT_METADATA_DISCRIMINANT},
    xbf_vec::{XbfVecMetadata, VEC_METADATA_DISCRIMINANT},
    XbfType,
};
use byteorder::ReadBytesExt;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XbfMetadata {
    Primitive(XbfPrimitiveMetadata),
    Vec(XbfVecMetadata),
    Struct(XbfStructMetadata),
}

impl XbfMetadata {
    pub fn serialize_base_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XbfMetadata::Primitive(x) => x.serialize_primitive_metadata(writer),
            XbfMetadata::Vec(x) => x.serialize_vec_metadata(writer),
            XbfMetadata::Struct(x) => x.serialize_struct_metadata(writer),
        }
    }

    pub fn deserialize_base_metadata(reader: &mut impl Read) -> io::Result<XbfMetadata> {
        let discriminant = reader.read_u8()?;
        if let Ok(x) = XbfPrimitiveMetadata::try_from(discriminant) {
            Ok(XbfMetadata::Primitive(x))
        } else if discriminant == VEC_METADATA_DISCRIMINANT {
            Ok(XbfVecMetadata::deserialize_vec_metadata(reader)?.to_base_metadata())
        } else if discriminant == STRUCT_METADATA_DISCRIMINANT {
            Ok(XbfStructMetadata::deserialize_struct_metadata(reader)?.to_base_metadata())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown metadata discriminant {}", discriminant),
            ))
        }
    }
}

impl From<XbfPrimitiveMetadata> for XbfMetadata {
    fn from(value: XbfPrimitiveMetadata) -> Self {
        value.into_base_metadata()
    }
}

impl From<&XbfPrimitiveMetadata> for XbfMetadata {
    fn from(value: &XbfPrimitiveMetadata) -> Self {
        value.to_base_metadata()
    }
}

impl From<XbfVecMetadata> for XbfMetadata {
    fn from(value: XbfVecMetadata) -> Self {
        value.into_base_metadata()
    }
}

impl From<&XbfVecMetadata> for XbfMetadata {
    fn from(value: &XbfVecMetadata) -> Self {
        value.to_base_metadata()
    }
}

impl From<XbfStructMetadata> for XbfMetadata {
    fn from(value: XbfStructMetadata) -> Self {
        value.into_base_metadata()
    }
}

impl From<&XbfStructMetadata> for XbfMetadata {
    fn from(value: &XbfStructMetadata) -> Self {
        value.to_base_metadata()
    }
}

impl From<&XbfType> for XbfMetadata {
    fn from(value: &XbfType) -> Self {
        match value {
            XbfType::Primitive(v) => XbfPrimitiveMetadata::from(v).to_base_metadata(),
            XbfType::Vec(v) => XbfVecMetadata::from(v).to_base_metadata(),
            XbfType::Struct(v) => XbfStructMetadata::from(v).to_base_metadata(),
        }
    }
}

pub trait XbfMetadataUpcast: private::Sealed {
    fn into_base_metadata(self) -> XbfMetadata;
    fn to_base_metadata(&self) -> XbfMetadata;
}

mod private {
    use crate::{XbfPrimitiveMetadata, XbfStructMetadata, XbfVecMetadata};

    pub trait Sealed {}

    impl Sealed for XbfPrimitiveMetadata {}
    impl Sealed for XbfVecMetadata {}
    impl Sealed for XbfStructMetadata {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn deserialize_unknown_discriminant_works() {
        let bad_discriminant = 69u8;
        let mut reader = Cursor::new(vec![bad_discriminant]);

        let should_be_err = XbfMetadata::deserialize_base_metadata(&mut reader)
            .expect_err("should have failed deserialization");

        assert_eq!(
            should_be_err.to_string(),
            format!("Unknown metadata discriminant {bad_discriminant}")
        )
    }
}
