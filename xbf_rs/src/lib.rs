mod conversions;
mod util;
mod xbf_primitive;
mod xbf_struct;
mod xbf_vec;

use byteorder::ReadBytesExt;
use std::io::{self, Read, Write};
use xbf_primitive::{XbfPrimitive, XbfPrimitiveMetadata};
use xbf_struct::{XbfStruct, XbfStructMetadata, STRUCT_METADATA_DISCRIMINANT};
use xbf_vec::{XbfVec, XbfVecMetadata, VEC_METADATA_DISCRIMINANT};

trait XbfMetadataUpcast: Into<XbfMetadata>
where
    XbfMetadata: for<'a> From<&'a Self>,
{
    fn into_base_metadata(self) -> XbfMetadata {
        self.into()
    }

    fn to_base_metadata(&self) -> XbfMetadata {
        self.into()
    }
}

trait XbfTypeUpcast: Into<XbfType>
where
    XbfType: for<'a> From<&'a Self>,
{
    fn into_base_type(self) -> XbfType {
        self.into()
    }

    fn to_base_type(&self) -> XbfType {
        self.into()
    }
}

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
            Ok(XbfStructMetadata::deserialize_struct_metadata(reader)?)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown metadata discriminant {}", discriminant),
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum XbfType {
    Primitive(XbfPrimitive),
    Vec(XbfVec),
    Struct(XbfStruct),
}

impl XbfType {
    pub fn serialize_base_type(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XbfType::Primitive(x) => x.serialize_primitive_type(writer),
            XbfType::Vec(x) => x.serialize_vec_type(writer),
            XbfType::Struct(_x) => todo!(),
        }
    }

    pub fn deserialize_base_type(
        metadata: &XbfMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XbfType> {
        match metadata {
            XbfMetadata::Primitive(x) => XbfPrimitive::deserialize_primitive_type(x, reader),
            XbfMetadata::Vec(x) => XbfVec::deserialize_vec_type(&x.inner_type, reader),
            XbfMetadata::Struct(_) => todo!(),
        }
    }
}
