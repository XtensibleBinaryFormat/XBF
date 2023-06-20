use crate::{
    xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata},
    xdl_struct::{XdlStruct, XdlStructMetadata},
    xdl_vec::{XdlVec, XdlVecMetadata},
    XdlMetadata, XdlMetadataUpcast, XdlType, XdlTypeUpcast,
};

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
impl XdlMetadataUpcast for XdlPrimitiveMetadata {}

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
impl XdlMetadataUpcast for XdlVecMetadata {}

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
impl XdlMetadataUpcast for XdlStructMetadata {}

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
impl From<&XdlPrimitive> for XdlType {
    fn from(value: &XdlPrimitive) -> Self {
        XdlType::Primitive(value.clone())
    }
}
impl XdlTypeUpcast for XdlPrimitive {}

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
impl XdlTypeUpcast for XdlVec {}

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
impl XdlTypeUpcast for XdlStruct {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upcast_metadata() {
        let primitive_metadata = XdlPrimitiveMetadata::I32;
        let vec_metadata = XdlVecMetadata::new(primitive_metadata.into());
        let struct_metadata = XdlStructMetadata::new(
            "test_struct".to_string(),
            vec![("field1".to_string(), XdlPrimitiveMetadata::I32.into())],
        );

        assert_eq!(
            XdlMetadata::Primitive(primitive_metadata),
            (&primitive_metadata).to_base_metadata()
        );
        assert_eq!(
            XdlMetadata::Primitive(primitive_metadata),
            primitive_metadata.into_base_metadata()
        );

        assert_eq!(
            XdlMetadata::Vec(vec_metadata.clone()),
            (&vec_metadata).to_base_metadata()
        );
        assert_eq!(
            XdlMetadata::Vec(vec_metadata.clone()),
            vec_metadata.into_base_metadata()
        );

        assert_eq!(
            XdlMetadata::Struct(struct_metadata.clone()),
            (&struct_metadata).to_base_metadata()
        );
        assert_eq!(
            XdlMetadata::Struct(struct_metadata.clone()),
            struct_metadata.into_base_metadata()
        )
    }

    #[test]
    fn test_upcast_type() {
        let primitive_type = XdlPrimitive::I32(69);
        let vec_type =
            XdlVec::new(XdlMetadata::Primitive((&primitive_type).into()), vec![]).unwrap();
        // TODO: Test Struct metadata once complete

        assert_eq!(
            XdlType::Primitive(primitive_type.clone()),
            (&primitive_type).to_base_type()
        );
        assert_eq!(
            XdlType::Primitive(primitive_type.clone()),
            primitive_type.into_base_type()
        );
        assert_eq!(XdlType::Vec(vec_type.clone()), (&vec_type).to_base_type());
        assert_eq!(XdlType::Vec(vec_type.clone()), vec_type.into_base_type());
    }
}
