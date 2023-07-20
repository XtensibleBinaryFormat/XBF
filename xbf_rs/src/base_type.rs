use crate::{xbf_primitive::XbfPrimitive, xbf_struct::XbfStruct, xbf_vec::XbfVec, XbfMetadata};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum XbfType {
    Primitive(XbfPrimitive),
    Vec(XbfVec),
    Struct(XbfStruct),
}

impl XbfType {
    /// Serialize an [`XbfType`] as defined by thejXBF specification.
    ///
    /// This function **does not** write out the metadata of the type. If you want to write out the
    /// metadata, convert this type to a [`XbfMetadata`] and call
    /// [`XbfMetadata::serialize_base_metadata`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::prelude::*;
    /// use xbf_rs::XbfType;
    ///
    /// let x = 42i32.to_xbf_primitive().to_base_type();
    ///
    /// let mut writer = vec![];
    /// x.serialize_base_type(&mut writer).unwrap();
    ///
    /// let mut expected = vec![];
    /// expected.extend_from_slice(&42i32.to_le_bytes());
    /// assert_eq!(writer, expected);
    /// ```
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
            XbfMetadata::Vec(x) => XbfVec::deserialize_vec_type(x, reader).map(|x| x.into()),
            XbfMetadata::Struct(x) => {
                XbfStruct::deserialize_struct_type(x, reader).map(|x| x.into())
            }
        }
    }
}

impl From<XbfPrimitive> for XbfType {
    fn from(value: XbfPrimitive) -> Self {
        value.into_base_type()
    }
}

impl From<&XbfPrimitive> for XbfType {
    fn from(value: &XbfPrimitive) -> Self {
        value.to_base_type()
    }
}

impl From<XbfVec> for XbfType {
    fn from(value: XbfVec) -> Self {
        value.into_base_type()
    }
}

impl From<&XbfVec> for XbfType {
    fn from(value: &XbfVec) -> Self {
        value.to_base_type()
    }
}

impl From<XbfStruct> for XbfType {
    fn from(value: XbfStruct) -> Self {
        value.into_base_type()
    }
}

impl From<&XbfStruct> for XbfType {
    fn from(value: &XbfStruct) -> Self {
        value.to_base_type()
    }
}

pub trait XbfTypeUpcast: private::Sealed {
    fn into_base_type(self) -> XbfType;
    fn to_base_type(&self) -> XbfType;
}

mod private {
    use crate::{XbfPrimitive, XbfStruct, XbfVec};

    pub trait Sealed {}

    impl Sealed for XbfPrimitive {}
    impl Sealed for XbfVec {}
    impl Sealed for XbfStruct {}
}
