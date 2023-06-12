use crate::{xdl_primitive::XdlPrimitive, XdlMetadata, XdlType};

impl From<&XdlType> for XdlMetadata {
    fn from(value: &XdlType) -> Self {
        match value {
            XdlType::Primitive(x) => XdlMetadata::Primitive(x.into()),
            XdlType::Vec(x) => XdlMetadata::Vec(x.into()),
            XdlType::Struct(_x) => todo!(),
        }
    }
}

impl From<XdlPrimitive> for XdlType {
    fn from(value: XdlPrimitive) -> Self {
        XdlType::Primitive(value)
    }
}
