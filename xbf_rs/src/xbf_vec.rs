//! A vector as defined by the XBF specification.

mod vec_metadata;

pub use vec_metadata::*;

use crate::{XbfMetadata, XbfType, XbfTypeUpcast};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{self, Read, Write},
    ops::Deref,
    slice::{Iter, IterMut},
    vec::IntoIter,
};

/// A vector type as defined by the XBF specification.
#[derive(Debug, Clone, PartialEq)]
pub struct XbfVec {
    pub(crate) metadata: XbfVecMetadata,
    elements: Vec<XbfType>,
}

impl XbfVec {
    /// Tries to create a new vector based on the supplied metadata.
    ///
    /// # Errors
    ///
    /// If all elements are not the same XBF type as what's specififed in the metadata,
    /// returns an [`ElementsNotHomogenousError`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfVec;
    /// use xbf_rs::XbfVecMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32);
    /// let data = [XbfPrimitive::I32(42), XbfPrimitive::I32(69)];
    /// let vec = XbfVec::new(metadata, data);
    ///
    /// // Data contains alements which are all the same type as the metdata.
    /// assert!(vec.is_ok());
    ///
    /// let metadata2 = XbfVecMetadata::new(XbfPrimitiveMetadata::I32);
    /// let data2 = vec![
    ///     XbfPrimitive::I32(42),
    ///     XbfPrimitive::String("Hello".to_string()),
    /// ];
    /// let vec2 = XbfVec::new(metadata2, data2);
    ///
    /// // Data contains alements which are not all the same type as the metdata.
    /// assert!(vec2.is_err());
    /// ```
    pub fn new(
        metadata: XbfVecMetadata,
        elements: impl IntoIterator<Item = impl Into<XbfType>>,
    ) -> Result<Self, ElementsNotHomogenousError> {
        let elements = elements.into_iter().map(Into::into).collect::<Vec<_>>();

        let all_same_type = elements
            .iter()
            .all(|x| *metadata.inner_type == XbfMetadata::from(x));

        if all_same_type {
            Ok(Self { metadata, elements })
        } else {
            Err(ElementsNotHomogenousError)
        }
    }

    /// Creates a new vector with the supplied metadata and elements without checking for homogeneity.
    ///
    /// # Examples
    /// ```rust
    /// use xbf_rs::XbfVec;
    /// use xbf_rs::XbfVecMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32);
    /// let data = vec![
    ///     XbfPrimitive::I32(42),
    ///     XbfPrimitive::String("Hello".to_string()),
    /// ];
    /// let vec = XbfVec::new_unchecked(metadata, data);
    ///
    /// // This is not good!
    /// assert_eq!(vec[0], XbfPrimitive::I32(42).into());
    /// assert_eq!(vec[1], XbfPrimitive::String("Hello".to_string()).into());
    /// ```
    pub fn new_unchecked(
        metadata: XbfVecMetadata,
        elements: impl IntoIterator<Item = impl Into<XbfType>>,
    ) -> Self {
        let elements = elements.into_iter().map(Into::into).collect::<Vec<_>>();
        Self { metadata, elements }
    }

    /// Serialize a vector as defined by the XBF specification.
    ///
    /// This function **does not** write out the metadata of the type. If you want to write out the
    /// metadata, convert this type to a [`XbfVecMetadata`] and call
    /// [`XbfVecMetadata::serialize_vec_metadata`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfVec;
    /// use xbf_rs::XbfVecMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// let vec = XbfVec::new(
    ///     XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
    ///     vec![XbfPrimitive::I32(42)]
    /// ).unwrap();
    /// let mut writer = vec![];
    /// vec.serialize_vec_type(&mut writer).unwrap();
    ///
    /// let mut expected = vec![];
    /// expected.extend_from_slice(&1u64.to_le_bytes());
    /// expected.extend_from_slice(&42u32.to_le_bytes());
    /// assert_eq!(writer, expected);
    /// ```
    pub fn serialize_vec_type(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u64::<LittleEndian>(self.elements.len() as u64)?;
        self.elements
            .iter()
            .try_for_each(|e| e.serialize_base_type(writer))
    }

    /// Deserialize a vector as defined by the XBF specification.
    ///
    /// This function **does not** read the metadata of the type from the reader. It is
    /// expected that to call this function the metadata for a type is already known, be
    /// that from reading it from the reader with
    /// [`deserialize_base_metadata`](crate::XbfMetadata::deserialize_base_metadata)
    /// or having it in some other manner.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfVec;
    /// use xbf_rs::XbfVecMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32);
    /// let mut reader = vec![];
    /// reader.extend_from_slice(&1u64.to_le_bytes());
    /// reader.extend_from_slice(&42u32.to_le_bytes());
    /// let mut reader = std::io::Cursor::new(reader);
    ///
    /// let vec = XbfVec::deserialize_vec_type(&metadata, &mut reader).unwrap();
    ///
    /// assert_eq!(vec.len(), 1);
    /// assert_eq!(vec[0], XbfPrimitive::I32(42).into());
    /// ```
    pub fn deserialize_vec_type(
        metadata: &XbfVecMetadata,
        reader: &mut impl Read,
    ) -> io::Result<XbfVec> {
        let inner_type = &metadata.inner_type;
        let len = reader.read_u64::<LittleEndian>()? as usize;
        let mut elements = Vec::with_capacity(len);
        for _ in 0..len {
            elements.push(XbfType::deserialize_base_type(inner_type, reader)?);
        }
        Ok(XbfVec::new_unchecked(metadata.clone(), elements))
    }

    /// Returns the metadata of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbf_rs::XbfVec;
    /// use xbf_rs::XbfVecMetadata;
    /// use xbf_rs::XbfPrimitive;
    /// use xbf_rs::XbfPrimitiveMetadata;
    ///
    /// let vec = XbfVec::new(
    ///     XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
    ///     [XbfPrimitive::I32(42)]
    /// ).unwrap();
    ///
    /// let metadata = vec.get_metadata();
    ///
    /// assert_eq!(metadata, XbfVecMetadata::new(XbfPrimitiveMetadata::I32));
    /// ```
    pub fn get_metadata(&self) -> XbfVecMetadata {
        self.metadata.clone()
    }
}

impl Deref for XbfVec {
    type Target = [XbfType];

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<'a> IntoIterator for &'a XbfVec {
    type Item = &'a XbfType;

    type IntoIter = Iter<'a, XbfType>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a mut XbfVec {
    type Item = &'a mut XbfType;

    type IntoIter = IterMut<'a, XbfType>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

impl IntoIterator for XbfVec {
    type Item = XbfType;

    type IntoIter = IntoIter<XbfType>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl XbfTypeUpcast for XbfVec {
    fn into_base_type(self) -> XbfType {
        XbfType::Vec(self)
    }

    fn to_base_type(&self) -> XbfType {
        XbfType::Vec(self.clone())
    }
}

/// Error type for [`XbfVec`]
///
/// In the future this may include information about what element wasn't
/// the same type as the metadata.
#[derive(Debug, PartialEq, Eq)]
pub struct ElementsNotHomogenousError;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{XbfMetadataUpcast, XbfPrimitive, XbfPrimitiveMetadata};
    use std::io::Cursor;

    #[test]
    fn vec_new_fails_with_not_homogenous_data() {
        let data = [XbfPrimitive::I32(42), XbfPrimitive::U32(69)].map(XbfType::from);
        let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32.into_base_metadata());
        let err = XbfVec::new(metadata, data).unwrap_err();
        assert_eq!(err, ElementsNotHomogenousError);
    }

    #[test]
    fn serialize_vec_primitive_works() {
        const TEST_NUM: i32 = 42;
        let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32.into_base_metadata());
        let vec = XbfVec::new(
            metadata.clone(),
            vec![XbfType::Primitive(XbfPrimitive::I32(TEST_NUM))],
        )
        .unwrap();
        let mut writer = vec![];

        vec.serialize_vec_type(&mut writer).unwrap();

        let mut expected = vec![];
        expected.extend_from_slice(&1u64.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());

        assert_eq!(writer, expected);

        let mut reader = Cursor::new(writer);

        let deserialized = XbfVec::deserialize_vec_type(&metadata, &mut reader).unwrap();

        assert_eq!(vec, deserialized);
    }

    #[test]
    fn serialize_vec_of_vec_works() {
        const TEST_NUM: i32 = 42;

        let vec_of_two_i32 = XbfVec::new(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
            vec![
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
            ],
        )
        .unwrap();

        let vec_of_vec_of_i32 = XbfVec::new_unchecked(
            XbfVecMetadata::new(vec_of_two_i32.get_metadata()),
            [vec_of_two_i32.clone(), vec_of_two_i32].map(XbfType::from),
        );

        let mut writer = vec![];

        vec_of_vec_of_i32.serialize_vec_type(&mut writer).unwrap();

        let mut expected = vec![];
        expected.extend_from_slice(&2u64.to_le_bytes());
        expected.extend_from_slice(&2u64.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());
        expected.extend_from_slice(&2u64.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());
        expected.extend_from_slice(&TEST_NUM.to_le_bytes());

        assert_eq!(writer, expected);
    }

    #[test]
    fn deserialize_vec_primitive_works() {
        const TEST_NUM: i32 = 42;
        let mut data = vec![];
        data.extend_from_slice(&1u64.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        let mut reader = Cursor::new(data);

        let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32);
        let expected = XbfVec::new(
            metadata.clone(),
            vec![XbfType::Primitive(XbfPrimitive::I32(TEST_NUM))],
        )
        .unwrap();

        let vec = XbfType::deserialize_base_type(&(metadata.into()), &mut reader).unwrap();

        assert_eq!(vec, expected.into());
    }

    #[test]
    fn deserialize_vec_of_vec_works() {
        const TEST_NUM: i32 = 42;
        let mut data = vec![];
        data.extend_from_slice(&2u64.to_le_bytes());
        data.extend_from_slice(&2u64.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        data.extend_from_slice(&2u64.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        data.extend_from_slice(&TEST_NUM.to_le_bytes());
        let mut reader = Cursor::new(data);

        // let outer_vec_metadata = XbfVecMetadata::new(inner_vec_metadata.to_base_metadata());

        let expected_inner_vec = XbfVec::new(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
            [
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
                XbfType::Primitive(XbfPrimitive::I32(TEST_NUM)),
            ],
        )
        .unwrap();

        let expected = XbfVec::new(
            XbfVecMetadata::new(expected_inner_vec.get_metadata()),
            [expected_inner_vec.clone(), expected_inner_vec].map(XbfType::from),
        )
        .unwrap();

        let vec =
            XbfType::deserialize_base_type(&expected.get_metadata().into(), &mut reader).unwrap();

        assert_eq!(vec, expected.into());
    }

    #[test]
    fn get_metdata_works() {
        let v = XbfVec::new(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I64),
            [1i64, 2, 4].map(XbfPrimitive::from),
        )
        .unwrap();
        let metadata = v.get_metadata();

        assert_eq!(metadata, XbfVecMetadata::new(XbfPrimitiveMetadata::I64));
    }

    #[test]
    fn into_iter_ref_works() {
        let x = &XbfVec::new(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
            [1i32, 2, 4].map(XbfPrimitive::from),
        )
        .unwrap();

        for i in x.into_iter() {
            assert!(matches!(i, XbfType::Primitive(XbfPrimitive::I32(_))));
        }

        let _usable_after_loop = x;
    }

    #[test]
    fn into_iter_mut_ref_works() {
        let x = &mut XbfVec::new(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
            [1i32, 2, 4].map(XbfPrimitive::from),
        )
        .unwrap();

        for i in x.into_iter() {
            assert!(matches!(i, XbfType::Primitive(XbfPrimitive::I32(_))));
        }

        let _usable_after_loop = x;
    }

    #[test]
    fn into_iter_value_works() {
        let x = XbfVec::new(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
            [1i32, 2, 4].map(XbfPrimitive::from),
        )
        .unwrap();

        for i in x.into_iter() {
            assert!(matches!(i, XbfType::Primitive(XbfPrimitive::I32(_))));
        }
    }

    #[test]
    fn deref_works() {
        let x = XbfVec::new(
            XbfVecMetadata::new(XbfPrimitiveMetadata::I32),
            [1i32, 2, 4].map(XbfPrimitive::from),
        )
        .unwrap();

        assert_eq!(x[0], XbfPrimitive::I32(1).into());
        assert_eq!(x[1], XbfPrimitive::I32(2).into());
        assert_eq!(x[2], XbfPrimitive::I32(4).into());
    }

    #[test]
    fn upcast_works() {
        let metadata = XbfVecMetadata::new(XbfPrimitiveMetadata::I32);
        let vec = XbfVec::new(metadata, [XbfType::Primitive(XbfPrimitive::I32(42))]).unwrap();
        let vec_ref = &vec;
        let expected = XbfType::Vec(vec.clone());

        assert_eq!(expected, vec_ref.into());
        assert_eq!(expected, vec_ref.to_base_type());
        assert_eq!(expected, vec.clone().into());
        assert_eq!(expected, vec.into_base_type());
    }
}
