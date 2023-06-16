use crate::{
    xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata},
    xdl_struct::{XdlStruct, XdlStructMetadata},
    xdl_vec::{XdlVec, XdlVecMetadata},
    IntoBaseMetadata, IntoBaseType, XdlMetadata, XdlType,
};

impl From<XdlPrimitiveMetadata> for XdlMetadata {
    fn from(value: XdlPrimitiveMetadata) -> Self {
        XdlMetadata::Primitive(value)
    }
}

// TODO: Test this
impl IntoBaseMetadata for XdlPrimitiveMetadata {
    fn into_base_metadata(self) -> XdlMetadata {
        self.into()
    }
}

impl From<XdlVecMetadata> for XdlMetadata {
    fn from(value: XdlVecMetadata) -> Self {
        XdlMetadata::Vec(value)
    }
}

// TODO: Test this
impl IntoBaseMetadata for XdlVecMetadata {
    fn into_base_metadata(self) -> XdlMetadata {
        self.into()
    }
}

impl From<XdlStructMetadata> for XdlMetadata {
    fn from(value: XdlStructMetadata) -> Self {
        XdlMetadata::Struct(value)
    }
}

// TODO: Test this
impl IntoBaseMetadata for XdlStructMetadata {
    fn into_base_metadata(self) -> XdlMetadata {
        self.into()
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

// TODO: Test this
impl IntoBaseType for XdlPrimitive {
    fn into_base_type(self) -> XdlType {
        self.into()
    }
}

impl From<XdlVec> for XdlType {
    fn from(value: XdlVec) -> Self {
        XdlType::Vec(value)
    }
}

// TODO: Test this
impl IntoBaseType for XdlVec {
    fn into_base_type(self) -> XdlType {
        self.into()
    }
}

impl From<XdlStruct> for XdlType {
    fn from(value: XdlStruct) -> Self {
        XdlType::Struct(value)
    }
}

// TODO: Test this
impl IntoBaseType for XdlStruct {
    fn into_base_type(self) -> XdlType {
        self.into()
    }
}
