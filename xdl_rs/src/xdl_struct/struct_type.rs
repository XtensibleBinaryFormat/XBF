use super::XdlStructMetadata;
use crate::{XdlMetadata, XdlType, XdlTypeUpcast};

#[derive(Debug, Clone, PartialEq)]
pub struct XdlStruct {
    pub(crate) metadata: XdlStructMetadata,
    fields: Vec<XdlType>,
}

impl XdlStruct {
    pub fn new(metadata: XdlStructMetadata, fields: Vec<XdlType>) -> Self {
        metadata
            .fields
            .iter()
            .zip(fields.iter())
            .all(|((_, x), y)| *x == XdlMetadata::from(y));
        Self { metadata, fields }
    }

    pub fn serialize_struct_type(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        self.fields
            .iter()
            .try_for_each(|f| f.serialize_base_type(writer))
    }

    pub fn deserialize_struct_type(
        metadata: &XdlStructMetadata,
        reader: &mut impl std::io::Read,
    ) -> std::io::Result<XdlStruct> {
        let mut struct_fields = vec![];
        for (_, field_type) in metadata.fields.iter() {
            struct_fields.push(XdlType::deserialize_base_type(&field_type, reader)?);
        }
        Ok(Self::new(metadata.clone(), struct_fields))
    }
}

impl XdlTypeUpcast for XdlStruct {}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::XdlStruct;
    use crate::{
        XdlMetadata, XdlMetadataUpcast, XdlPrimitive, XdlPrimitiveMetadata, XdlStructMetadata,
        XdlType, XdlTypeUpcast, XdlVec, XdlVecMetadata,
    };

    #[test]
    fn test_struct_serde_works() {
        let primitive_metadata = XdlMetadata::Primitive(XdlPrimitiveMetadata::I32);
        let vec_metadata = XdlMetadata::Vec(XdlVecMetadata::new(XdlPrimitiveMetadata::I32.into()));
        let inner_struct_metadata = XdlStructMetadata::new(
            "test_struct".to_string(),
            vec![(
                "a".to_string(),
                XdlMetadata::Primitive(XdlPrimitiveMetadata::I32),
            )],
        );
        let outer_metadata = XdlStructMetadata::new(
            "test".to_string(),
            vec![
                ("a".to_string(), primitive_metadata),
                ("b".to_string(), vec_metadata),
                (
                    "c".to_string(),
                    inner_struct_metadata.to_base_metadata().clone(),
                ),
            ],
        );

        let primitive = XdlPrimitive::I32(42);
        let vec = XdlVec::new_unchecked(
            XdlPrimitiveMetadata::I32.into(),
            vec![primitive.to_base_type()],
        );
        let inner_struct = XdlStruct::new(inner_struct_metadata, vec![primitive.to_base_type()]);
        let my_struct = XdlStruct::new(
            outer_metadata.clone(),
            vec![
                primitive.clone().into(),
                vec.clone().into(),
                inner_struct.clone().into(),
            ],
        );

        let mut writer = vec![];
        my_struct.serialize_struct_type(&mut writer).unwrap();

        let mut expected = vec![];
        primitive.serialize_primitive_type(&mut expected).unwrap();
        vec.serialize_vec_type(&mut expected).unwrap();
        inner_struct.serialize_struct_type(&mut expected).unwrap();

        assert_eq!(writer, expected);

        let mut reader = Cursor::new(writer);
        let deserialized =
            XdlStruct::deserialize_struct_type(&outer_metadata, &mut reader).unwrap();

        assert_eq!(my_struct, deserialized);
    }

    #[test]
    fn upcast_works() {
        let my_struct = XdlStruct::new(
            XdlStructMetadata::new(
                "my_struct".to_string(),
                vec![("field1".to_string(), XdlPrimitiveMetadata::I32.into())],
            ),
            vec![XdlPrimitive::I32(42).into()],
        );

        assert_eq!(
            XdlType::Struct(my_struct.clone()),
            (&my_struct).to_base_type()
        );
        assert_eq!(
            XdlType::Struct(my_struct.clone()),
            my_struct.into_base_type()
        );
    }
}
