mod conversions;
mod util;
mod xdl_primitive;
mod xdl_struct;
mod xdl_vec;

use byteorder::ReadBytesExt;
use std::io::{self, Read, Write};
use xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata};
use xdl_struct::{XdlStruct, XdlStructMetadata};
use xdl_vec::{XdlVec, XdlVecMetadata, VEC_METADATA_DISCRIMINANT};

trait Serialize {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()>;
}

trait DeserializeType {
    fn deserialize_type(metadata: &XdlMetadata, reader: &mut impl Read) -> io::Result<XdlType>;
}

trait DeserializeMetadata {
    fn deserialize_metadata(reader: &mut impl Read) -> io::Result<XdlMetadata>;
}

trait IntoBaseMetadata {
    fn into_base_metadata(self) -> XdlMetadata;
}

trait IntoBaseType {
    fn into_base_type(self) -> XdlType;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XdlMetadata {
    Primitive(XdlPrimitiveMetadata),
    Vec(XdlVecMetadata),
    Struct(XdlStructMetadata),
}

impl Serialize for XdlMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XdlMetadata::Primitive(x) => x.serialize(writer),
            XdlMetadata::Vec(x) => x.serialize(writer),
            XdlMetadata::Struct(_x) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum XdlType {
    Primitive(XdlPrimitive),
    Vec(XdlVec),
    Struct(XdlStruct),
}

impl Serialize for XdlType {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XdlType::Primitive(x) => x.serialize(writer),
            XdlType::Vec(x) => x.serialize(writer),
            XdlType::Struct(_x) => todo!(),
        }
    }
}

impl DeserializeType for XdlType {
    fn deserialize_type(metadata: &XdlMetadata, reader: &mut impl Read) -> io::Result<XdlType> {
        match metadata {
            XdlMetadata::Primitive(x) => XdlPrimitive::deserialize_primitive(x, reader),
            XdlMetadata::Vec(x) => XdlVec::deserialize_type(&x.inner_type, reader),
            XdlMetadata::Struct(_) => todo!(),
        }
    }
}

impl DeserializeMetadata for XdlMetadata {
    fn deserialize_metadata(reader: &mut impl Read) -> io::Result<XdlMetadata> {
        let discriminant = reader.read_u8()?;
        if let Ok(x) = discriminant.try_into() {
            Ok(XdlMetadata::Primitive(x))
        } else if discriminant == VEC_METADATA_DISCRIMINANT {
            Ok(XdlVecMetadata::deserialize_metadata(reader)?)
        } else {
            todo!()
        }
    }
}
