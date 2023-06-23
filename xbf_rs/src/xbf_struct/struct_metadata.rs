use crate::{
    util::{read_string, write_string},
    xbf_vec::VEC_METADATA_DISCRIMINANT,
    DeserializeMetadata, Serialize, XbfMetadata,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

pub const STRUCT_METADATA_DISCRIMINANT: u8 = VEC_METADATA_DISCRIMINANT + 1;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct XbfStructMetadata {
    name: String,
    fields: Vec<(String, XbfMetadata)>,
}

impl XbfStructMetadata {
    pub fn new(name: String, fields: Vec<(String, XbfMetadata)>) -> Self {
        Self { name, fields }
    }
}

impl Serialize for XbfStructMetadata {
    fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(STRUCT_METADATA_DISCRIMINANT)?;
        write_string(&self.name, writer)?;
        writer.write_u16::<LittleEndian>(self.fields.len() as u16)?;
        self.fields
            .iter()
            .try_for_each(|(name, type_)| write_string(name, writer).and(type_.serialize(writer)))
    }
}

impl DeserializeMetadata for XbfStructMetadata {
    fn deserialize_metadata(reader: &mut impl Read) -> io::Result<XbfMetadata> {
        let name = read_string(reader)?;
        let len = reader.read_u16::<LittleEndian>()?;
        let mut fields = Vec::with_capacity(len as usize);
        for _ in 0..len {
            fields.push((
                read_string(reader)?,
                XbfMetadata::deserialize_metadata(reader)?,
            ))
        }
        Ok(XbfMetadata::Struct(XbfStructMetadata { name, fields }))
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{xbf_primitive::XbfPrimitiveMetadata, xbf_vec::XbfVecMetadata};

    use super::*;

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
                    XbfMetadata::Struct(XbfStructMetadata {
                        name: "inner".to_string(),
                        fields: vec![(
                            "d".to_string(),
                            XbfMetadata::Primitive(XbfPrimitiveMetadata::I32),
                        )],
                    }),
                ),
            ],
        );

        let mut writer = Vec::new();
        metadata.serialize(&mut writer).unwrap();

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
        let deserialized = XbfMetadata::deserialize_metadata(&mut reader).unwrap();
        assert_eq!(XbfMetadata::Struct(metadata), deserialized);
    }
}
