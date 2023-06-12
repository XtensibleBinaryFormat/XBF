use crate::{
    xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata},
    xdl_struct::{XdlStruct, XdlStructMetadata},
    xdl_vec::{XdlVec, XdlVecMetadata},
    XdlMetadata, XdlType,
};

impl From<XdlPrimitiveMetadata> for XdlMetadata {
    fn from(value: XdlPrimitiveMetadata) -> Self {
        XdlMetadata::Primitive(value)
    }
}

impl From<XdlVecMetadata> for XdlMetadata {
    fn from(value: XdlVecMetadata) -> Self {
        XdlMetadata::Vec(value)
    }
}

impl From<XdlStructMetadata> for XdlMetadata {
    fn from(value: XdlStructMetadata) -> Self {
        XdlMetadata::Struct(value)
    }
}

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

impl From<XdlVec> for XdlType {
    fn from(value: XdlVec) -> Self {
        XdlType::Vec(value)
    }
}

impl From<XdlStruct> for XdlType {
    fn from(value: XdlStruct) -> Self {
        XdlType::Struct(value)
    }
}
