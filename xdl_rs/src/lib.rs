mod conversions;
mod util;
mod xdl_primitive;
mod xdl_struct;
mod xdl_vec;

use byteorder::ReadBytesExt;
use std::io::{self, Read, Write};
use xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata};
use xdl_struct::{XdlStruct, XdlStructMetadata, STRUCT_METADATA_DISCRIMINANT};
use xdl_vec::{XdlVec, XdlVecMetadata, VEC_METADATA_DISCRIMINANT};

trait XdlMetadataUpcast: Into<XdlMetadata>
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

trait XdlTypeUpcast: Into<XdlType>
where
    XdlType: for<'a> From<&'a Self>,
{
    fn into_base_type(self) -> XdlType {
        self.into()
    }
    fn to_base_type(&self) -> XdlType {
        self.into()
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum XdlType {
    Primitive(XdlPrimitive),
    Vec(XdlVec),
    Struct(XdlStruct),
}

impl XdlType {
    pub fn serialize_base_type(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XdlType::Primitive(x) => x.serialize_primitive_type(writer),
            XdlType::Vec(x) => x.serialize_vec_type(writer),
            XdlType::Struct(_x) => todo!(),
        }
    }

    pub fn deserialize_base_type(
        metadata: &XdlMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XdlType> {
        match metadata {
            XdlMetadata::Primitive(x) => XdlPrimitive::deserialize_primitive_type(x, reader),
            XdlMetadata::Vec(x) => XdlVec::deserialize_vec_type(&x.inner_type, reader),
            XdlMetadata::Struct(_) => todo!(),
        }
    }
}
