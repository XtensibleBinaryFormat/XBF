use crate::{
    base_metadata::XbfMetadataUpcast,
    util::{read_string, write_string},
    RcType, XbfMetadata, XbfStruct, VEC_METADATA_DISCRIMINANT,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::IndexMap;
use std::io::{self, Read, Write};

/// The metadata discriminant for a Struct type.
///
/// This is the same for all structs regardless of their contents. It's value should always be
/// equal to the discriminant value of the vector type plus one.
pub const STRUCT_METADATA_DISCRIMINANT: u8 = VEC_METADATA_DISCRIMINANT + 1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::xbf_struct) struct XbfStructMetadataInner {
    pub(in crate::xbf_struct) name: Box<str>,
    pub(in crate::xbf_struct) fields: IndexMap<Box<str>, XbfMetadata>,
}

/// Metadata for a Struct type.
///
/// Struct metadata is immutable, and cannot be changed once created. Cloning this metadata is
/// relatively inexpensive, as the current implementation is using reference-counted shared
/// memory internally.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct XbfStructMetadata {
    pub(in crate::xbf_struct) inner: RcType<XbfStructMetadataInner>,
}

impl XbfStructMetadata {
    /// Creates a new [`XbfStructMetadata`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    /// use xbf_rs::XbfMetadataUpcast;
    ///
    /// use indexmap::indexmap;
    ///
    /// let metadata = XbfStructMetadata::new(
    ///     "test_struct".to_string(),
    ///     indexmap!{
    ///         "a".to_string() => XbfPrimitiveMetadata::I32.into(),
    ///         "b".to_string() => XbfPrimitiveMetadata::U64.into(),
    ///     },
    /// );
    ///
    /// assert_eq!(metadata.name(), "test_struct");
    /// assert_eq!(metadata.get_field_type("a"), Some(&XbfPrimitiveMetadata::I32.into()));
    /// assert_eq!(metadata.get_field_type("b"), Some(&XbfPrimitiveMetadata::U64.into()));
    /// assert_eq!(metadata.get_field_type("c"), None);
    /// ```
    pub fn new(
        name: impl Into<Box<str>>,
        fields: IndexMap<impl Into<Box<str>>, XbfMetadata>,
    ) -> Self {
        Self {
            inner: RcType::new(XbfStructMetadataInner {
                name: name.into(),
                fields: fields.into_iter().map(|(k, v)| (k.into(), v)).collect(),
            }),
        }
    }

    /// Returns the name of the struct.
    ///
    /// # Examples
    /// ```rust
    /// use xbf_rs::prelude::*;
    ///
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// let metadata = XbfStructMetadata::new(
    ///   "test_struct",
    ///   indexmap! { "a" => XbfPrimitiveMetadata::I32.into() },
    /// );
    ///
    /// assert_eq!(metadata.name(), "test_struct");
    /// ```
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// Returns the metadata of a field if it exists, otherwise returns `None`.
    ///
    /// # Examples
    /// ```rust
    /// use xbf_rs::prelude::*;
    ///
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// let name = "test_struct";
    /// let field1_name = "a";
    /// let field1_type = XbfPrimitiveMetadata::I32.into_base_metadata();
    ///
    /// let metadata = XbfStructMetadata::new(
    ///   name.to_string(),
    ///   indexmap!{
    ///     field1_name.to_string() => field1_type.clone(),
    ///   },
    /// );
    ///
    /// assert_eq!(metadata.name(), name);
    /// assert_eq!(metadata.get_field_type(field1_name), Some(&field1_type));
    /// assert_eq!(metadata.get_field_type("b"), None);
    /// ```
    pub fn get_field_type(&self, field: &str) -> Option<&XbfMetadata> {
        self.inner.fields.get(field)
    }

    /// Serialize struct metadata as defined by the XBF specification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    /// use xbf_rs::STRUCT_METADATA_DISCRIMINANT;
    ///
    /// use indexmap::indexmap;
    ///
    /// let struct_name = "test_struct";
    /// let field1_name = "a";
    /// let field2_name = "b";
    /// let metadata = XbfStructMetadata::new(
    ///     struct_name.to_string(),
    ///     indexmap!{
    ///         field1_name.to_string() => XbfPrimitiveMetadata::I32.into(),
    ///         field2_name.to_string() => XbfPrimitiveMetadata::U64.into(),
    ///     },
    /// );
    /// let mut writer = vec![];
    ///
    /// metadata.serialize_struct_metadata(&mut writer).unwrap();
    ///
    /// let expected = {
    ///     let mut v = vec![STRUCT_METADATA_DISCRIMINANT];
    ///     v.extend_from_slice((struct_name.len() as u64).to_le_bytes().as_slice());
    ///     v.extend_from_slice(struct_name.as_bytes());
    ///     v.extend_from_slice(2u16.to_le_bytes().as_slice());
    ///     v.extend_from_slice((field1_name.len() as u64).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field1_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::I32 as u8).to_le_bytes().as_slice());
    ///     v.extend_from_slice((field2_name.len() as u64).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field2_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::U64 as u8).to_le_bytes().as_slice());
    ///     v
    /// };
    ///
    ///
    /// assert_eq!(writer, expected);
    /// ```
    pub fn serialize_struct_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(STRUCT_METADATA_DISCRIMINANT)?;

        let name = &self.inner.name;
        write_string(name, writer)?;

        let fields = &self.inner.fields;
        let len = fields.len() as u16;
        writer.write_u16::<LittleEndian>(len)?;

        fields.iter().try_for_each(|(name, type_)| {
            write_string(name, writer).and_then(|_| type_.serialize_base_metadata(writer))
        })
    }

    /// Deserialize struct metadata as defined by the XBF specification.
    ///
    /// This method assumes that you know for a fact you are about to receive Struct metadata. If you
    /// do not know what sort of metadata you are receiving, use
    /// [`deserialize_base_metadata`](crate::XbfMetadata::deserialize_base_metadata).
    ///
    /// # Example
    ///
    /// ```rust
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    /// use xbf_rs::STRUCT_METADATA_DISCRIMINANT;
    ///
    /// use indexmap::indexmap;
    ///
    /// let struct_name = "test_struct".to_string();
    /// let field1_name = "a".to_string();
    /// let field2_name = "b".to_string();
    ///
    /// let reader = (|| {
    ///     let mut v = vec![];
    ///     v.extend_from_slice((struct_name.len() as u64).to_le_bytes().as_slice());
    ///     v.extend_from_slice(struct_name.as_bytes());
    ///     v.extend_from_slice(2u16.to_le_bytes().as_slice());
    ///     v.extend_from_slice((field1_name.len() as u64).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field1_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::I32 as u8).to_le_bytes().as_slice());
    ///     v.extend_from_slice((field2_name.len() as u64).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field2_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::U64 as u8).to_le_bytes().as_slice());
    ///     v
    /// })();
    /// let mut reader = std::io::Cursor::new(reader);
    ///
    /// let metadata = XbfStructMetadata::deserialize_struct_metadata(&mut reader).unwrap();
    ///
    /// assert_eq!(metadata, XbfStructMetadata::new(struct_name, indexmap!{
    ///     field1_name => XbfPrimitiveMetadata::I32.into(),
    ///     field2_name => XbfPrimitiveMetadata::U64.into(),
    /// }));
    pub fn deserialize_struct_metadata(reader: &mut impl Read) -> io::Result<XbfStructMetadata> {
        let name = read_string(reader)?;
        let len = reader.read_u16::<LittleEndian>()?;
        let mut fields = IndexMap::with_capacity(len as usize);
        for _ in 0..len {
            fields.insert(
                read_string(reader)?,
                XbfMetadata::deserialize_base_metadata(reader)?,
            );
        }
        Ok(XbfStructMetadata::new(name, fields))
    }
}

impl From<&XbfStruct> for XbfStructMetadata {
    fn from(value: &XbfStruct) -> Self {
        value.get_metadata()
    }
}

impl XbfMetadataUpcast for XbfStructMetadata {
    fn into_base_metadata(self) -> XbfMetadata {
        XbfMetadata::Struct(self)
    }

    fn to_base_metadata(&self) -> XbfMetadata {
        XbfMetadata::Struct(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;

    use super::*;
    use crate::{xbf_primitive::XbfPrimitiveMetadata, XbfVecMetadata};
    use std::io::Cursor;

    #[test]
    fn metadata_new_works() {
        let metadata = XbfStructMetadata::new(
            "test",
            indexmap! {
                "a" => XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
            },
        );

        assert_eq!(metadata.name(), "test");
        assert_eq!(
            metadata.get_field_type("a"),
            Some(&XbfMetadata::Primitive(XbfPrimitiveMetadata::I32))
        );
    }

    #[test]
    fn metadata_serde_works() {
        let metadata = XbfStructMetadata::new(
            "test".to_string(),
            indexmap! {
                    "a" => XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
                    "b" => XbfMetadata::Vec(XbfVecMetadata::new(XbfPrimitiveMetadata::I32)),
                    "c" => XbfMetadata::Struct(XbfStructMetadata::new(
                        "inner".to_string(),
                        indexmap! {
                            "d".to_string( ) =>
                            XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
                        },
                    )),
            },
        );

        let mut writer = Vec::new();
        metadata.serialize_struct_metadata(&mut writer).unwrap();

        let mut expected = Vec::new();
        // disciminant
        expected.write_u8(STRUCT_METADATA_DISCRIMINANT).unwrap();
        // name
        write_string(&metadata.inner.name, &mut expected).unwrap();
        // num of fields
        expected.write_u16::<LittleEndian>(3).unwrap();
        // field a
        write_string("a", &mut expected).unwrap();
        expected.write_u8(XbfPrimitiveMetadata::I32 as u8).unwrap();
        // field b
        write_string("b", &mut expected).unwrap();
        expected.write_u8(VEC_METADATA_DISCRIMINANT).unwrap();
        expected.write_u8(XbfPrimitiveMetadata::I32 as u8).unwrap();
        // field c
        write_string("c", &mut expected).unwrap();
        // field c is a struct, so do struct stuff again
        // discriminant
        expected.write_u8(STRUCT_METADATA_DISCRIMINANT).unwrap();
        // name
        write_string("inner", &mut expected).unwrap();
        // num_of_fields
        expected.write_u16::<LittleEndian>(1).unwrap();
        // field d
        write_string("d", &mut expected).unwrap();
        expected.write_u8(XbfPrimitiveMetadata::I32 as u8).unwrap();

        assert_eq!(expected, writer);

        let mut reader = Cursor::new(writer);
        let deserialized = XbfMetadata::deserialize_base_metadata(&mut reader).unwrap();
        assert_eq!(XbfMetadata::Struct(metadata), deserialized);
    }

    #[test]
    fn upcast_works() {
        let struct_metadata = XbfStructMetadata::new(
            "test_struct".to_string(),
            indexmap! {"field1".to_string() => XbfPrimitiveMetadata::I32.into()},
        );
        let struct_metadata_ref = &struct_metadata;

        assert_eq!(
            XbfMetadata::Struct(struct_metadata.clone()),
            struct_metadata_ref.to_base_metadata()
        );
        assert_eq!(
            XbfMetadata::Struct(struct_metadata.clone()),
            struct_metadata.into_base_metadata()
        );
    }
}
