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

trait Serialize {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()>;
}

trait DeserializeType {
    fn deserialize_type(metadata: &XbfMetadata, reader: &mut impl Read) -> io::Result<XbfType>;
}

trait DeserializeMetadata {
    fn deserialize_metadata(reader: &mut impl Read) -> io::Result<XbfMetadata>;
}

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

impl Serialize for XbfMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XbfMetadata::Primitive(x) => x.serialize(writer),
            XbfMetadata::Vec(x) => x.serialize(writer),
            XbfMetadata::Struct(x) => x.serialize(writer),
        }
    }
}

impl DeserializeMetadata for XbfMetadata {
    fn deserialize_metadata(reader: &mut impl Read) -> io::Result<XbfMetadata> {
        let discriminant = reader.read_u8()?;
        if let Ok(x) = discriminant.try_into() {
            Ok(XbfMetadata::Primitive(x))
        } else if discriminant == VEC_METADATA_DISCRIMINANT {
            Ok(XbfVecMetadata::deserialize_metadata(reader)?)
        } else if discriminant == STRUCT_METADATA_DISCRIMINANT {
            Ok(XbfStructMetadata::deserialize_metadata(reader)?)
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

impl Serialize for XbfType {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XbfType::Primitive(x) => x.serialize(writer),
            XbfType::Vec(x) => x.serialize(writer),
            XbfType::Struct(_x) => todo!(),
        }
    }
}

impl DeserializeType for XbfType {
    fn deserialize_type(metadata: &XbfMetadata, reader: &mut impl Read) -> io::Result<XbfType> {
        match metadata {
            XbfMetadata::Primitive(x) => XbfPrimitive::deserialize_primitive(x, reader),
            XbfMetadata::Vec(x) => XbfVec::deserialize_type(&x.inner_type, reader),
            XbfMetadata::Struct(_) => todo!(),
        }
    }
}
