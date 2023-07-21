//! A struct as defined by the XBF specification
mod struct_metadata;

pub use struct_metadata::*;

use crate::{XbfMetadata, XbfType, XbfTypeUpcast};
use std::{
    error::Error,
    fmt::Display,
    io::{self, Read, Write},
};

/// A struct as defined by the XBF specification.
///
/// The metadata of a struct cannot be changed, however its fields can be mutated. Cloning a struct
/// is relatively expensive, as the underlying fields will also need to be cloned.
#[derive(Debug, Clone, PartialEq)]
pub struct XbfStruct {
    pub(crate) metadata: XbfStructMetadata,
    fields: Box<[XbfType]>,
}

impl XbfStruct {
    /// Tries to create a new [`XbfStruct`] based on the supplied metadata.
    ///
    /// # Errors
    ///
    /// If all fields are not the same XBF type as what's specififed in the metadata,
    /// returns a [`StructError`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::prelude::*;
    ///
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// let name = "test_struct";
    /// let field1_name = "a";
    /// let field1_type = XbfPrimitiveMetadata::I32.into_base_metadata();
    /// let field2_name = "b";
    /// let field2_type = XbfPrimitiveMetadata::U64.into_base_metadata();
    ///
    /// let i32_field = 42i32.into_xbf_primitive().into_base_type();
    /// let u64_field = 42u64.into_xbf_primitive().into_base_type();
    /// let i64_field = 42i64.into_xbf_primitive().into_base_type();
    ///
    /// let metadata = XbfStructMetadata::new(name.to_string(), indexmap!{
    ///     field1_name.to_string() => field1_type,
    ///     field2_name.to_string() => field2_type.clone(),
    /// });
    ///
    /// let struct1 = XbfStruct::new(
    ///     metadata.clone(),
    ///     [
    ///         i32_field.clone(),
    ///         u64_field.clone(),
    ///     ],
    /// ).expect("a valid struct");
    ///
    /// assert_eq!(struct1.get_metadata(), metadata);
    /// assert_eq!(struct1.get("a"), Some(&i32_field));
    /// assert_eq!(struct1.get("b"), Some(&u64_field));
    ///
    /// let struct2 = XbfStruct::new(
    ///     metadata.clone(),
    ///     [
    ///         i32_field.clone(),
    ///         i64_field,
    ///     ],
    /// ).expect_err("an invalid struct");
    ///
    /// let expected_err_message = format!("Provided value for field {field2_name} \
    ///     is of type {:?}, expected {field2_type:?}",
    ///     XbfPrimitiveMetadata::I64.to_base_metadata(),
    /// );
    ///
    /// assert_eq!(struct2.to_string(), expected_err_message);
    ///
    /// let struct3 = XbfStruct::new(metadata, vec![i32_field]).expect_err("an invalid struct");
    ///
    /// let expected_err_message = "Provided fields have length: 1, expected: 2".to_string();
    ///
    /// assert_eq!(struct3.to_string(), expected_err_message);
    /// ```
    pub fn new(
        metadata: XbfStructMetadata,
        fields: impl IntoIterator<Item = XbfType>,
    ) -> Result<Self, StructError> {
        let fields: Box<[XbfType]> = fields.into_iter().collect();

        {
            let given_fields_len = fields.len();
            let metadata_fields_len = metadata.fields.len();

            if given_fields_len != metadata_fields_len {
                Err(StructError::DifferentLengths {
                    metadata_len: metadata_fields_len,
                    fields_len: given_fields_len,
                })?
            }
        }

        for ((name, expected_field_type), val) in metadata.fields.iter().zip(fields.iter()) {
            let actual_field_type = XbfMetadata::from(val);
            if *expected_field_type != actual_field_type {
                Err(StructError::FieldMismatch {
                    field_name: name.to_string(),
                    expected_field_type: expected_field_type.clone(),
                    actual_field_type,
                })?
            }
        }

        Ok(Self { metadata, fields })
    }

    /// Creates a new [`XbfStruct`] with the supplied metadata and fields without checking if the
    /// given fields are the correct types.
    ///
    /// If you use this function you are proceeding at your own peril.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::prelude::*;
    ///
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// let name = "test_struct";
    /// let field1_name = "a";
    /// let field1_type = XbfPrimitiveMetadata::I32.into_base_metadata();
    /// let field2_name = "b";
    /// let field2_type = XbfPrimitiveMetadata::U64.into_base_metadata();
    ///
    /// let i32_field = 42i32.into_xbf_primitive().into_base_type();
    /// let u64_field = 42u64.into_xbf_primitive().into_base_type();
    ///
    /// let metadata = XbfStructMetadata::new(
    ///     name.to_string(),
    ///     indexmap!{
    ///         field1_name.to_string() => field1_type,
    ///         field2_name.to_string() => field2_type,
    ///     }
    /// );
    ///
    /// let struct1 = XbfStruct::new_unchecked(
    ///     metadata.clone(),
    ///     [
    ///         i32_field.clone(),
    ///         u64_field.clone(),
    ///     ]
    /// );
    ///
    /// assert_eq!(struct1.get(&field1_name), Some(&i32_field));
    /// assert_eq!(struct1.get(&field2_name), Some(&u64_field));
    /// ```
    pub fn new_unchecked(
        metadata: XbfStructMetadata,
        fields: impl IntoIterator<Item = XbfType>,
    ) -> Self {
        let fields = fields.into_iter().collect();
        Self { metadata, fields }
    }

    /// Serialize a struct as defined by the XBF specification.
    ///
    /// This function **does not** write out the metadata of the type. If you want to write out the
    /// metadata, get the metadata with [`Self::get_metadata`] and serialize that with
    /// [`XbfStructMetadata::serialize_struct_metadata`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// let metadata = XbfStructMetadata::new(
    ///     "test_struct".to_string(),
    ///     indexmap!{
    ///         "a".to_string() => XbfPrimitiveMetadata::I32.into(),
    ///     },
    /// );
    ///
    /// let val = XbfStruct::new(
    ///     metadata,
    ///     vec![XbfPrimitive::I32(42).into()],
    /// ).expect("a valid struct");
    ///
    /// let mut writer = vec![];
    /// val.serialize_struct_type(&mut writer).unwrap();
    ///
    /// let expected = 42i32.to_le_bytes();
    /// assert_eq!(writer, expected);
    /// ```
    pub fn serialize_struct_type(&self, writer: &mut impl Write) -> io::Result<()> {
        self.fields
            .iter()
            .try_for_each(|f| f.serialize_base_type(writer))
    }

    /// Deserialize a struct as defined by the XBF specification.
    ///
    /// This function **does not** read the metadata of the type from the reader. It is expected
    /// that to call this function the metadata for atype is already known, be that from reading it
    /// from the reader with [`deserialize_base_metadata`](crate::XbfMetadata::deserialize_base_metadata)
    /// or having it in some other manner.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// // setting up a reader with one i32 in it
    /// let mut reader = vec![];
    /// reader.extend_from_slice(&42i32.to_le_bytes());
    /// let mut reader = std::io::Cursor::new(reader);
    ///
    /// // the metadata we've gotten from somewhere describing the type
    /// let metadata = XbfStructMetadata::new(
    ///     "test_struct".to_string(),
    ///     indexmap!{
    ///         "a".to_string() => XbfPrimitiveMetadata::I32.into()
    ///     },
    /// );
    ///
    /// // deserializing the struct with the given metadata
    /// let val = XbfStruct::deserialize_struct_type(&metadata, &mut reader).unwrap();
    ///
    /// assert_eq!(val.get("a"), Some(&XbfPrimitive::I32(42).into()));
    /// ```
    pub fn deserialize_struct_type(
        metadata: &XbfStructMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XbfStruct> {
        let mut struct_fields = vec![];
        for (_, field_type) in metadata.fields.iter() {
            struct_fields.push(XbfType::deserialize_base_type(field_type, reader)?);
        }
        Ok(Self::new_unchecked(metadata.clone(), struct_fields))
    }

    /// Returns the metadata of the struct.
    ///
    /// Getting the metadata returns an owned [`XbfStructMetadata`], which requires a clone to take
    /// place. This will likely be changed in the future to be mroe efficient.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// let metadata = XbfStructMetadata::new(
    ///     "test_struct".to_string(),
    ///      indexmap!{
    ///         "a".to_string() => XbfPrimitiveMetadata::I32.into(),
    ///     },
    /// );
    ///
    /// let val = XbfStruct::new_unchecked(
    ///     metadata.clone(),
    ///     vec![XbfPrimitive::I32(42).into()],
    /// );
    ///
    /// assert_eq!(metadata, val.get_metadata())
    /// ```
    pub fn get_metadata(&self) -> XbfStructMetadata {
        self.metadata.clone()
    }

    /// Returns a reference to a field's data if the field exists, otherwise returns `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::prelude::*;
    ///
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// use indexmap::indexmap;
    ///
    /// let field_name = "a";
    /// let field_data = 42i32.into_xbf_primitive().into_base_type();
    ///
    /// let s = XbfStruct::new_unchecked(
    ///     XbfStructMetadata::new(
    ///         "test_struct".to_string(),
    ///         indexmap!{
    ///             field_name.to_string() => XbfPrimitiveMetadata::I32.to_base_metadata()
    ///         },
    ///     ),
    ///     vec![field_data.clone()],
    /// );
    /// assert_eq!(s.get(field_name), Some(&field_data));
    /// assert_eq!(s.get("b"), None);
    /// ```
    pub fn get(&self, field_name: &str) -> Option<&XbfType> {
        self.metadata
            .fields
            .get_index_of(field_name)
            .map(|i| &self.fields[i])
    }

    /// Sets the value of a field if it exists, returning the previous value, otherwise returns
    /// `None`.
    ///
    /// # Examples
    /// ```rust
    /// use xbf_rs::prelude::*;
    /// use xbf_rs::XbfStruct;
    /// use xbf_rs::XbfStructMetadata;
    /// use xbf_rs::XbfPrimitiveMetadata;
    /// use indexmap::indexmap;
    ///
    /// let metadata = XbfStructMetadata::new(
    ///     "test_struct".to_string(),
    ///     indexmap!{
    ///         "a".to_string() => XbfPrimitiveMetadata::I32.into(),
    ///     }
    /// );
    ///
    /// let expected_old_value = 42i32.to_xbf_primitive().into_base_type();
    /// let mut s = XbfStruct::new(metadata, [expected_old_value.clone()]).expect("a valid struct");
    /// let new_value = 100i32.to_xbf_primitive().into_base_type();
    ///
    /// assert_eq!(s.set("a", new_value.clone()), Some(expected_old_value));
    /// assert_eq!(s.get("a"), Some(&new_value));
    /// ```
    pub fn set(&mut self, field_name: &str, field_data: XbfType) -> Option<XbfType> {
        self.metadata.fields.get_index_of(field_name).and_then(|i| {
            let current = &mut self.fields[i];
            if XbfMetadata::from(&*current) != XbfMetadata::from(&field_data) {
                None
            } else {
                Some(std::mem::replace(current, field_data))
            }
        })
    }
}

impl XbfTypeUpcast for XbfStruct {
    fn into_base_type(self) -> XbfType {
        XbfType::Struct(self)
    }

    fn to_base_type(&self) -> XbfType {
        XbfType::Struct(self.clone())
    }
}

/// Error type for creating an [`XbfStruct`].
#[derive(Debug)]
pub enum StructError {
    FieldMismatch {
        field_name: String,
        expected_field_type: XbfMetadata,
        actual_field_type: XbfMetadata,
    },
    DifferentLengths {
        metadata_len: usize,
        fields_len: usize,
    },
}

impl Display for StructError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructError::FieldMismatch {
                field_name,
                expected_field_type,
                actual_field_type,
            } => write!(
                f,
                "Provided value for field {field_name} is of type {actual_field_type:?}, \
                expected {expected_field_type:?}"
            ),

            StructError::DifferentLengths {
                metadata_len,
                fields_len,
            } => write!(
                f,
                "Provided fields have length: {fields_len}, expected: {metadata_len}"
            ),
        }
    }
}

impl Error for StructError {}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;

    use super::*;
    use crate::{XbfMetadataUpcast, XbfPrimitive, XbfPrimitiveMetadata, XbfVec, XbfVecMetadata};
    use std::io::Cursor;

    #[test]
    fn struct_new_works() {
        let name = "test_struct";
        let field1_name = "a";
        let field2_name = "b";

        let metadata = XbfStructMetadata::new(
            name.to_string(),
            indexmap! {
                field1_name.to_string() => XbfPrimitiveMetadata::I32.into_base_metadata(),
                field2_name.to_string() => XbfPrimitiveMetadata::U64.into_base_metadata(),
            },
        );

        let with_correct_fields = XbfStruct::new(
            metadata,
            vec![
                XbfPrimitive::I32(42).into_base_type(),
                XbfPrimitive::U64(69).into_base_type(),
            ],
        )
        .expect("a valid struct");

        assert_eq!(
            with_correct_fields.get(field1_name),
            Some(&XbfPrimitive::I32(42).into())
        );
        assert_eq!(
            with_correct_fields.get(field2_name),
            Some(&XbfPrimitive::U64(69).into())
        );
    }

    #[test]
    fn struct_new_failure_works() {
        let name = "test_struct";
        let field1_name = "a";
        let field2_name = "b";

        let metadata = XbfStructMetadata::new(
            name.to_string(),
            indexmap! {
                field1_name.to_string() => XbfPrimitiveMetadata::I32.into_base_metadata(),
                field2_name.to_string() => XbfPrimitiveMetadata::U64.into_base_metadata(),
            },
        );

        let with_wrong_field1_type = XbfStruct::new(
            metadata.clone(),
            vec![
                XbfPrimitive::String("hi".to_string()).into_base_type(),
                XbfPrimitive::U64(69).into_base_type(),
            ],
        );

        assert_eq!(
            with_wrong_field1_type.unwrap_err().to_string(),
            StructError::FieldMismatch {
                field_name: field1_name.to_string(),
                expected_field_type: XbfPrimitiveMetadata::I32.into_base_metadata(),
                actual_field_type: XbfPrimitiveMetadata::String.into_base_metadata()
            }
            .to_string()
        );

        let wrong_number_of_fields =
            XbfStruct::new(metadata, vec![XbfPrimitive::I32(69).into_base_type()]);

        assert_eq!(
            wrong_number_of_fields.unwrap_err().to_string(),
            StructError::DifferentLengths {
                metadata_len: 2,
                fields_len: 1
            }
            .to_string()
        )
    }

    #[test]
    fn struct_new_unchecked_works() {
        let name = "test_struct";
        let field1_name = "a";
        let field2_name = "b";

        let metadata = XbfStructMetadata::new(
            name.to_string(),
            indexmap! {
                    field1_name.to_string() =>
                    XbfPrimitiveMetadata::String.into_base_metadata(),
                    field2_name.to_string() =>
                    XbfPrimitiveMetadata::Bytes.into_base_metadata(),
            },
        );

        let blatantly_wrong_fields = XbfStruct::new_unchecked(
            metadata,
            vec![
                XbfPrimitive::I32(42).into_base_type(),
                XbfPrimitive::U64(69).into_base_type(),
            ],
        );

        assert_eq!(
            blatantly_wrong_fields.get("a"),
            Some(&XbfPrimitive::I32(42).into())
        );
        assert_eq!(
            blatantly_wrong_fields.get("b"),
            Some(&XbfPrimitive::U64(69).into())
        )
    }

    #[test]
    fn struct_serde_works() {
        let primitive_metadata = XbfMetadata::Primitive(XbfPrimitiveMetadata::I32);
        let vec_metadata = XbfMetadata::Vec(XbfVecMetadata::new(XbfPrimitiveMetadata::I32.into()));
        let inner_struct_metadata = XbfStructMetadata::new(
            "test_struct".to_string(),
            indexmap! {
                "a".to_string() =>
                XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
            },
        );
        let outer_metadata = XbfStructMetadata::new(
            "test".to_string(),
            indexmap! {
                "a".to_string()=> primitive_metadata,
                "b".to_string()=> vec_metadata,
                "c".to_string()=> inner_struct_metadata.to_base_metadata(),
            },
        );

        let primitive = XbfPrimitive::I32(42);
        let vec = XbfVec::new_unchecked(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I32.into()),
            vec![primitive.to_base_type()],
        );
        let inner_struct = XbfStruct::new(inner_struct_metadata, vec![primitive.to_base_type()])
            .expect("a valid struct");
        let my_struct = XbfStruct::new(
            outer_metadata.clone(),
            vec![
                primitive.clone().into(),
                vec.clone().into(),
                inner_struct.clone().into(),
            ],
        )
        .expect("a valid struct");

        let mut writer = vec![];
        my_struct.serialize_struct_type(&mut writer).unwrap();

        let mut expected = vec![];
        primitive.serialize_primitive_type(&mut expected).unwrap();
        vec.serialize_vec_type(&mut expected).unwrap();
        inner_struct.serialize_struct_type(&mut expected).unwrap();

        assert_eq!(writer, expected);

        let mut reader = Cursor::new(writer);
        let deserialized =
            XbfStruct::deserialize_struct_type(&outer_metadata, &mut reader).unwrap();

        assert_eq!(my_struct, deserialized);
    }

    #[test]
    fn upcast_works() {
        let metadata = XbfStructMetadata::new(
            "my_struct".to_string(),
            indexmap! {"field1".to_string() => XbfPrimitiveMetadata::I32.into()},
        );
        let my_struct =
            XbfStruct::new(metadata, vec![XbfPrimitive::I32(42).into()]).expect("a valid struct");
        let struct_ref = &my_struct;
        let expected = XbfType::Struct(my_struct.clone());

        assert_eq!(expected, struct_ref.into());
        assert_eq!(expected, struct_ref.to_base_type());
        assert_eq!(expected, my_struct.clone().into());
        assert_eq!(expected, my_struct.into_base_type());
    }
}
