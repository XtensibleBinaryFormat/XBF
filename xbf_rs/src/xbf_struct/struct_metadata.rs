use crate::{
    base_metadata::XbfMetadataUpcast,
    util::{read_string, write_string},
    XbfMetadata, XbfStruct, VEC_METADATA_DISCRIMINANT,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::{map::Entry, IndexMap};
use std::{
    error::Error,
    fmt::Display,
    io::{self, Read, Write},
};

/// The metadata discriminant for a Struct type.
///
/// This is the same for all structs regardless of their contents. It's value should always be
/// equal to the discriminant value of the vector type plus one.
pub const STRUCT_METADATA_DISCRIMINANT: u8 = VEC_METADATA_DISCRIMINANT + 1;

/// Metadata for a Struct type.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct XbfStructMetadata {
    name: String,
    pub(super) fields: IndexMap<String, XbfMetadata>,
}

impl XbfStructMetadata {
    /// Tries to create a new [`XbfStructMetadata`].
    ///
    /// # Errors
    ///
    /// If there are any duplicate field names, returns a [`StructMetadataDuplicateFieldError`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    /// use xbf_rs::XbfMetadataUpcast;
    ///
    /// let metadata = XbfStructMetadata::new(
    ///     "test_struct".to_string(),
    ///     vec![
    ///         ("a".to_string(), XbfPrimitiveMetadata::I32.into()),
    ///         ("b".to_string(), XbfPrimitiveMetadata::U64.into()),
    ///     ],
    /// ).unwrap();
    ///
    /// assert_eq!(metadata.name(), "test_struct");
    /// assert_eq!(metadata.get_field_type("a"), Some(&XbfPrimitiveMetadata::I32.into()));
    /// assert_eq!(metadata.get_field_type("b"), Some(&XbfPrimitiveMetadata::U64.into()));
    /// assert_eq!(metadata.get_field_type("c"), None);
    ///
    ///
    /// let err = XbfStructMetadata::new(
    ///     "test_struct".to_string(),
    ///     vec![
    ///         ("a".to_string(), XbfPrimitiveMetadata::I32.into()),
    ///         ("a".to_string(), XbfPrimitiveMetadata::U64.into()),
    ///     ]
    /// ).unwrap_err();
    ///
    /// assert_eq!(err.to_string(), format!("Duplicate field: {}, type1: {:?}, type2: {:?}",
    ///         "a",
    ///         XbfPrimitiveMetadata::I32.to_base_metadata(),
    ///         XbfPrimitiveMetadata::U64.to_base_metadata()
    ///     )
    /// );
    /// ```
    pub fn new(
        name: String,
        fields: Vec<(String, XbfMetadata)>,
    ) -> Result<Self, StructMetadataDuplicateFieldError> {
        let mut fields_map: IndexMap<String, XbfMetadata> = IndexMap::new();

        for (field_name, metadata) in fields {
            match fields_map.entry(field_name) {
                Entry::Occupied(o) => {
                    let (dup_name, dup_metadata) = o.remove_entry();

                    Err(StructMetadataDuplicateFieldError::new(
                        &dup_name,
                        &dup_metadata,
                        &metadata,
                    ))?
                }
                Entry::Vacant(v) => {
                    v.insert(metadata);
                }
            };
        }

        Ok(Self {
            name,
            fields: fields_map,
        })
    }

    /// Creates a new [`XbfStructMetadata`] without checking for duplicate field names.
    ///
    /// If you use this function you are proceeding at your own peril.
    ///
    /// # Examples
    /// ```rust
    /// use xbf_rs::prelude::*;
    ///
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// let name = "test_struct";
    /// let field1_name = "a";
    /// let field1_type = XbfPrimitiveMetadata::I32.into_base_metadata();
    /// let field2_name = "b";
    /// let field2_type = XbfPrimitiveMetadata::U64.into_base_metadata();
    ///
    /// let metadata = XbfStructMetadata::new_unchecked(
    ///   name.to_string(),
    ///   vec![
    ///     (field1_name.to_string(), field1_type.clone()),
    ///     (field2_name.to_string(), field2_type.clone()),
    ///   ],
    /// );
    ///
    /// assert_eq!(metadata.name(), name);
    /// assert_eq!(metadata.get_field_type(field1_name), Some(&field1_type));
    /// assert_eq!(metadata.get_field_type(field2_name), Some(&field2_type));
    /// ```
    pub fn new_unchecked(name: String, fields: Vec<(String, XbfMetadata)>) -> Self {
        let fields = fields.into_iter().collect::<IndexMap<_, _>>();
        Self { name, fields }
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
    /// let name = "test_struct";
    ///
    /// let metadata = XbfStructMetadata::new_unchecked(
    ///   name.to_string(),
    ///   vec![],
    /// );
    ///
    /// assert_eq!(metadata.name(), name);
    /// ```
    pub fn name(&self) -> &str {
        &self.name
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
    /// let name = "test_struct";
    /// let field1_name = "a";
    /// let field1_type = XbfPrimitiveMetadata::I32.into_base_metadata();
    ///
    /// let metadata = XbfStructMetadata::new_unchecked(
    ///   name.to_string(),
    ///   vec![
    ///     (field1_name.to_string(), field1_type.clone()),
    ///   ],
    /// );
    ///
    /// assert_eq!(metadata.name(), name);
    /// assert_eq!(metadata.get_field_type(field1_name), Some(&field1_type));
    /// assert_eq!(metadata.get_field_type("b"), None);
    /// ```
    pub fn get_field_type(&self, field: &str) -> Option<&XbfMetadata> {
        self.fields.get(field)
        // self.get_field_index(field).map(|i| &self.fields[*i].1)
    }

    // pub(crate) fn get_field_index(&self, field: &str) -> Option<&usize> {
    //     self.fields_lookup.get(field)
    // }

    /// Serialize a struct as defined by the XBF specification.
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
    /// let struct_name = "test_struct";
    /// let field1_name = "a";
    /// let field2_name = "b";
    /// let metadata = XbfStructMetadata::new_unchecked(
    ///     struct_name.to_string(),
    ///     vec![
    ///         (field1_name.to_string(), XbfPrimitiveMetadata::I32.into()),
    ///         (field2_name.to_string(), XbfPrimitiveMetadata::U64.into()),
    ///     ],
    /// );
    /// let mut writer = vec![];
    ///
    /// metadata.serialize_struct_metadata(&mut writer).unwrap();
    ///
    /// let expected = (|| {
    ///     let mut v = vec![STRUCT_METADATA_DISCRIMINANT];
    ///     v.extend_from_slice((struct_name.len() as u16).to_le_bytes().as_slice());
    ///     v.extend_from_slice(struct_name.as_bytes());
    ///     v.extend_from_slice(2u16.to_le_bytes().as_slice());
    ///     v.extend_from_slice((field1_name.len() as u16).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field1_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::I32 as u8).to_le_bytes().as_slice());
    ///     v.extend_from_slice((field2_name.len() as u16).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field2_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::U64 as u8).to_le_bytes().as_slice());
    ///     v
    /// })();
    ///
    ///
    /// assert_eq!(writer, expected);
    /// ```
    pub fn serialize_struct_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(STRUCT_METADATA_DISCRIMINANT)?;
        write_string(&self.name, writer)?;
        writer.write_u16::<LittleEndian>(self.fields.len() as u16)?;
        self.fields.iter().try_for_each(|(name, type_)| {
            write_string(name, writer).and_then(|_| type_.serialize_base_metadata(writer))
        })
    }

    /// Deserialize struct metadata as defined by the XBF specification.
    ///
    ///This method assumes that you know for a fact you are about to receive Struct metadata. If you
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
    /// let struct_name = "test_struct".to_string();
    /// let field1_name = "a".to_string();
    /// let field2_name = "b".to_string();
    ///
    /// let reader = (|| {
    ///     let mut v = vec![];
    ///     v.extend_from_slice((struct_name.len() as u16).to_le_bytes().as_slice());
    ///     v.extend_from_slice(struct_name.as_bytes());
    ///     v.extend_from_slice(2u16.to_le_bytes().as_slice());
    ///     v.extend_from_slice((field1_name.len() as u16).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field1_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::I32 as u8).to_le_bytes().as_slice());
    ///     v.extend_from_slice((field2_name.len() as u16).to_le_bytes().as_slice());
    ///     v.extend_from_slice(field2_name.as_bytes());
    ///     v.extend_from_slice((XbfPrimitiveMetadata::U64 as u8).to_le_bytes().as_slice());
    ///     v
    /// })();
    /// let mut reader = std::io::Cursor::new(reader);
    ///
    /// let metadata = XbfStructMetadata::deserialize_struct_metadata(&mut reader).unwrap();
    ///
    /// assert_eq!(metadata, XbfStructMetadata::new_unchecked(struct_name, vec![
    ///     (field1_name, XbfPrimitiveMetadata::I32.into()),
    ///     (field2_name, XbfPrimitiveMetadata::U64.into()),
    /// ]));
    pub fn deserialize_struct_metadata(reader: &mut impl Read) -> io::Result<XbfStructMetadata> {
        let name = read_string(reader)?;
        let len = reader.read_u16::<LittleEndian>()?;
        let mut fields = Vec::with_capacity(len as usize);
        for _ in 0..len {
            fields.push((
                read_string(reader)?,
                XbfMetadata::deserialize_base_metadata(reader)?,
            ))
        }
        Ok(XbfStructMetadata::new_unchecked(name, fields))
    }
}

impl XbfMetadataUpcast for XbfStructMetadata {}

impl From<&XbfStruct> for XbfStructMetadata {
    fn from(value: &XbfStruct) -> Self {
        value.get_metadata()
    }
}

/// Error type for creating [`XbfStructMetadata`].
///
/// TODO: similar to [`StructFieldMismatchError`] should this contain the actual fields?
#[derive(Debug)]
pub struct StructMetadataDuplicateFieldError(String);

impl StructMetadataDuplicateFieldError {
    fn new(duplicate_field_name: &str, type1: &XbfMetadata, type2: &XbfMetadata) -> Self {
        let s =
            format!("Duplicate field: {duplicate_field_name}, type1: {type1:?}, type2: {type2:?}");
        StructMetadataDuplicateFieldError(s)
    }
}

impl Display for StructMetadataDuplicateFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StructMetadataDuplicateFieldError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{xbf_primitive::XbfPrimitiveMetadata, XbfVecMetadata};
    use std::io::Cursor;

    #[test]
    fn metadata_new_works() {
        let metadata = XbfStructMetadata::new(
            "test".to_string(),
            vec![(
                "a".to_string(),
                XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
            )],
        )
        .expect("a valid struct metadata");

        assert_eq!(metadata.name(), "test");
        assert_eq!(
            metadata.get_field_type("a"),
            Some(&XbfMetadata::Primitive(XbfPrimitiveMetadata::I32))
        );
    }

    #[test]
    fn metadata_new_failure_works() {
        let dup_field_name = "a";
        let type1 = XbfMetadata::Primitive(XbfPrimitiveMetadata::I32);
        let type2 = XbfMetadata::Primitive(XbfPrimitiveMetadata::String);
        let metadata = XbfStructMetadata::new(
            "test".to_string(),
            vec![
                (dup_field_name.to_string(), type1.clone()),
                (dup_field_name.to_string(), type2.clone()),
            ],
        )
        .expect_err("an invalid struct");

        assert_eq!(
            metadata.to_string(),
            format!("Duplicate field: {dup_field_name}, type1: {type1:?}, type2: {type2:?}")
        )
    }

    #[test]
    fn metadata_serde_works() {
        let metadata = XbfStructMetadata::new(
            "test".to_string(),
            vec![
                (
                    "a".to_string(),
                    XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
                ),
                (
                    "b".to_string(),
                    XbfMetadata::Vec(XbfVecMetadata::new(XbfPrimitiveMetadata::I32.into())),
                ),
                (
                    "c".to_string(),
                    XbfMetadata::Struct(XbfStructMetadata::new_unchecked(
                        "inner".to_string(),
                        vec![(
                            "d".to_string(),
                            XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
                        )],
                    )),
                ),
            ],
        )
        .expect("a valid struct metadata");

        let mut writer = Vec::new();
        metadata.serialize_struct_metadata(&mut writer).unwrap();

        let mut expected = Vec::new();
        // disciminant
        expected.write_u8(STRUCT_METADATA_DISCRIMINANT).unwrap();
        // name
        write_string(&metadata.name, &mut expected).unwrap();
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
            vec![("field1".to_string(), XbfPrimitiveMetadata::I32.into())],
        )
        .expect("a valid struct metadata");
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
