use crate::{
    xbf_primitive::{XbfPrimitive, XbfPrimitiveMetadata},
    xbf_struct::{XbfStruct, XbfStructMetadata},
    xbf_vec::{XbfVec, XbfVecMetadata},
    XbfMetadata, XbfMetadataUpcast, XbfType, XbfTypeUpcast,
};

impl From<XbfPrimitiveMetadata> for XbfMetadata {
    fn from(value: XbfPrimitiveMetadata) -> Self {
        XbfMetadata::Primitive(value)
    }
}
impl From<&XbfPrimitiveMetadata> for XbfMetadata {
    fn from(value: &XbfPrimitiveMetadata) -> Self {
        XbfMetadata::Primitive(*value)
    }
}
impl XbfMetadataUpcast for XbfPrimitiveMetadata {}

impl From<XbfVecMetadata> for XbfMetadata {
    fn from(value: XbfVecMetadata) -> Self {
        XbfMetadata::Vec(value)
    }
}
impl From<&XbfVecMetadata> for XbfMetadata {
    fn from(value: &XbfVecMetadata) -> Self {
        XbfMetadata::Vec(value.clone())
    }
}
impl XbfMetadataUpcast for XbfVecMetadata {}

impl From<XbfStructMetadata> for XbfMetadata {
    fn from(value: XbfStructMetadata) -> Self {
        XbfMetadata::Struct(value)
    }
}
impl From<&XbfStructMetadata> for XbfMetadata {
    fn from(value: &XbfStructMetadata) -> Self {
        XbfMetadata::Struct(value.clone())
    }
}
impl XbfMetadataUpcast for XbfStructMetadata {}

impl From<&XbfType> for XbfMetadata {
    fn from(value: &XbfType) -> Self {
        match value {
            XbfType::Primitive(x) => XbfMetadata::Primitive(x.into()),
            XbfType::Vec(x) => XbfMetadata::Vec(x.into()),
            XbfType::Struct(_x) => todo!(),
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
impl XbfTypeUpcast for XbfPrimitive {}

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
impl XbfTypeUpcast for XbfVec {}

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
// TODO: Test this
impl XbfTypeUpcast for XbfStruct {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upcast_metadata() {
        let primitive_metadata = XbfPrimitiveMetadata::I32;
        let vec_metadata = XbfVecMetadata::new(primitive_metadata.into());
        // TODO: Test Struct metadata once complete

        assert_eq!(
            XbfMetadata::Primitive(primitive_metadata),
            (&primitive_metadata).to_base_metadata()
        );
        assert_eq!(
            XbfMetadata::Primitive(primitive_metadata),
            primitive_metadata.into_base_metadata()
        );

        assert_eq!(
            XbfMetadata::Vec(vec_metadata.clone()),
            (&vec_metadata).to_base_metadata()
        );
        assert_eq!(
            XbfMetadata::Vec(vec_metadata.clone()),
            vec_metadata.into_base_metadata()
        );
    }

    #[test]
    fn test_upcast_type() {
        let primitive_type = XbfPrimitive::I32(69);
        let vec_type =
            XbfVec::new(XbfMetadata::Primitive((&primitive_type).into()), vec![]).unwrap();
        // TODO: Test Struct metadata once complete

        assert_eq!(
            XbfType::Primitive(primitive_type.clone()),
            (&primitive_type).to_base_type()
        );
        assert_eq!(
            XbfType::Primitive(primitive_type.clone()),
            primitive_type.into_base_type()
        );
        assert_eq!(XbfType::Vec(vec_type.clone()), (&vec_type).to_base_type());
        assert_eq!(XbfType::Vec(vec_type.clone()), vec_type.into_base_type());
    }
}
