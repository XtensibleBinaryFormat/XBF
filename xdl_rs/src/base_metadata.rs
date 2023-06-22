use crate::xdl_primitive::XdlPrimitiveMetadata;
use crate::xdl_struct::{XdlStructMetadata, STRUCT_METADATA_DISCRIMINANT};
use crate::xdl_vec::{XdlVecMetadata, VEC_METADATA_DISCRIMINANT};
use crate::XdlType;
use byteorder::ReadBytesExt;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XdlMetadata {
    Primitive(XdlPrimitiveMetadata),
    Vec(XdlVecMetadata),
    Struct(XdlStructMetadata),
}

impl XdlMetadata {
    pub fn serialize_base_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XdlMetadata::Primitive(x) => x.serialize_primitive_metadata(writer),
            XdlMetadata::Vec(x) => x.serialize_vec_metadata(writer),
            XdlMetadata::Struct(x) => x.serialize_struct_metadata(writer),
        }
    }

    pub fn deserialize_base_metadata(reader: &mut impl Read) -> io::Result<XdlMetadata> {
        let discriminant = reader.read_u8()?;
        if let Ok(x) = XdlPrimitiveMetadata::try_from(discriminant) {
            Ok(XdlMetadata::Primitive(x))
        } else if discriminant == VEC_METADATA_DISCRIMINANT {
            Ok(XdlVecMetadata::deserialize_vec_metadata(reader)?.to_base_metadata())
        } else if discriminant == STRUCT_METADATA_DISCRIMINANT {
            Ok(XdlStructMetadata::deserialize_struct_metadata(reader)?)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown metadata discriminant {}", discriminant),
            ))
        }
    }
}

impl From<XdlPrimitiveMetadata> for XdlMetadata {
    fn from(value: XdlPrimitiveMetadata) -> Self {
        XdlMetadata::Primitive(value)
    }
}

impl From<&XdlPrimitiveMetadata> for XdlMetadata {
    fn from(value: &XdlPrimitiveMetadata) -> Self {
        XdlMetadata::Primitive(*value)
    }
}

impl From<XdlVecMetadata> for XdlMetadata {
    fn from(value: XdlVecMetadata) -> Self {
        XdlMetadata::Vec(value)
    }
}

impl From<&XdlVecMetadata> for XdlMetadata {
    fn from(value: &XdlVecMetadata) -> Self {
        XdlMetadata::Vec(value.clone())
    }
}

impl From<XdlStructMetadata> for XdlMetadata {
    fn from(value: XdlStructMetadata) -> Self {
        XdlMetadata::Struct(value)
    }
}

impl From<&XdlStructMetadata> for XdlMetadata {
    fn from(value: &XdlStructMetadata) -> Self {
        XdlMetadata::Struct(value.clone())
    }
}

impl From<&XdlType> for XdlMetadata {
    fn from(value: &XdlType) -> Self {
        match value {
            XdlType::Primitive(v) => XdlPrimitiveMetadata::from(v).to_base_metadata(),
            XdlType::Vec(v) => XdlVecMetadata::from(v).to_base_metadata(),
            XdlType::Struct(v) => XdlStructMetadata::from(v).to_base_metadata(),
        }
    }
}

pub trait XdlMetadataUpcast: Into<XdlMetadata>
where
    XdlMetadata: for<'a> From<&'a Self>,
{
    fn into_base_metadata(self) -> XdlMetadata {
        self.into()
    }
    fn to_base_metadata(&self) -> XdlMetadata {
        self.into()
    }
}
