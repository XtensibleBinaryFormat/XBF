use crate::{xdl_primitive::XdlPrimitive, xdl_struct::XdlStruct, xdl_vec::XdlVec, XdlMetadata};
use std::io::{self, Read, Write};

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
            XdlType::Struct(x) => x.serialize_struct_type(writer),
        }
    }

    pub fn deserialize_base_type(
        metadata: &XdlMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XdlType> {
        match metadata {
            XdlMetadata::Primitive(x) => {
                XdlPrimitive::deserialize_primitive_type(x, reader).map(|x| x.into())
            }
            XdlMetadata::Vec(x) => {
                XdlVec::deserialize_vec_type(&x.inner_type, reader).map(|x| x.into())
            }
            XdlMetadata::Struct(x) => {
                XdlStruct::deserialize_struct_type(x, reader).map(|x| x.into())
            }
        }
    }
}

impl From<XdlPrimitive> for XdlType {
    fn from(value: XdlPrimitive) -> Self {
        XdlType::Primitive(value)
    }
}

impl From<&XdlPrimitive> for XdlType {
    fn from(value: &XdlPrimitive) -> Self {
        XdlType::Primitive(value.clone())
    }
}

impl From<XdlVec> for XdlType {
    fn from(value: XdlVec) -> Self {
        XdlType::Vec(value)
    }
}

impl From<&XdlVec> for XdlType {
    fn from(value: &XdlVec) -> Self {
        XdlType::Vec(value.clone())
    }
}

impl From<XdlStruct> for XdlType {
    fn from(value: XdlStruct) -> Self {
        XdlType::Struct(value)
    }
}

impl From<&XdlStruct> for XdlType {
    fn from(value: &XdlStruct) -> Self {
        XdlType::Struct(value.clone())
    }
}

pub trait XdlTypeUpcast: Into<XdlType>
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
