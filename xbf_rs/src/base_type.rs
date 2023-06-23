use crate::{xbf_primitive::XbfPrimitive, xbf_struct::XbfStruct, xbf_vec::XbfVec, XbfMetadata};
use std::io::{self, Read, Write};

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
            XbfType::Struct(x) => x.serialize_struct_type(writer),
        }
    }

    pub fn deserialize_base_type(
        metadata: &XbfMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XbfType> {
        match metadata {
            XbfMetadata::Primitive(x) => {
                XbfPrimitive::deserialize_primitive_type(x, reader).map(|x| x.into())
            }
            XbfMetadata::Vec(x) => {
                XbfVec::deserialize_vec_type(&x.inner_type, reader).map(|x| x.into())
            }
            XbfMetadata::Struct(x) => {
                XbfStruct::deserialize_struct_type(x, reader).map(|x| x.into())
            }
        }
    }
}

impl From<XbfPrimitive> for XbfType {
    fn from(value: XbfPrimitive) -> Self {
        XbfType::Primitive(value)
    }
}

impl From<&XbfPrimitive> for XbfType {
    fn from(value: &XbfPrimitive) -> Self {
        XbfType::Primitive(value.clone())
    }
}

impl From<XbfVec> for XbfType {
    fn from(value: XbfVec) -> Self {
        XbfType::Vec(value)
    }
}

impl From<&XbfVec> for XbfType {
    fn from(value: &XbfVec) -> Self {
        XbfType::Vec(value.clone())
    }
}

impl From<XbfStruct> for XbfType {
    fn from(value: XbfStruct) -> Self {
        XbfType::Struct(value)
    }
}

impl From<&XbfStruct> for XbfType {
    fn from(value: &XbfStruct) -> Self {
        XbfType::Struct(value.clone())
    }
}

pub trait XbfTypeUpcast: Into<XbfType>
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
