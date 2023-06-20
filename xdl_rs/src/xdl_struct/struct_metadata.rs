use crate::{
    util::{read_string, write_string},
    xdl_vec::VEC_METADATA_DISCRIMINANT,
    XdlMetadata,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

pub const STRUCT_METADATA_DISCRIMINANT: u8 = VEC_METADATA_DISCRIMINANT + 1;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct XdlStructMetadata {
    name: String,
    fields: Vec<(String, XdlMetadata)>,
}

impl XdlStructMetadata {
    pub fn new(name: String, fields: Vec<(String, XdlMetadata)>) -> Self {
        Self { name, fields }
    }

    pub fn serialize_struct_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(STRUCT_METADATA_DISCRIMINANT)?;
        write_string(&self.name, writer)?;
        writer.write_u16::<LittleEndian>(self.fields.len() as u16)?;
        self.fields.iter().try_for_each(|(name, type_)| {
            write_string(name, writer).and_then(|_| type_.serialize_base_metadata(writer))
        })
    }

    pub fn deserialize_struct_metadata(reader: &mut impl Read) -> io::Result<XdlMetadata> {
        let name = read_string(reader)?;
        let len = reader.read_u16::<LittleEndian>()?;
        let mut fields = Vec::with_capacity(len as usize);
        for _ in 0..len {
            fields.push((
                read_string(reader)?,
                XdlMetadata::deserialize_base_metadata(reader)?,
            ))
        }
        Ok(XdlMetadata::Struct(XdlStructMetadata { name, fields }))
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{xdl_primitive::XdlPrimitiveMetadata, xdl_vec::XdlVecMetadata};

    use super::*;

    #[test]
    fn metadata_serde_works() {
        let metadata = XdlStructMetadata::new(
            "test".to_string(),
            vec![
                (
                    "a".to_string(),
                    XdlMetadata::Primitive(XdlPrimitiveMetadata::I32),
                ),
                (
                    "b".to_string(),
                    XdlMetadata::Vec(XdlVecMetadata::new(XdlPrimitiveMetadata::I32.into())),
                ),
                (
                    "c".to_string(),
                    XdlMetadata::Struct(XdlStructMetadata {
                        name: "inner".to_string(),
                        fields: vec![(
                            "d".to_string(),
                            XdlMetadata::Primitive(XdlPrimitiveMetadata::I32),
                        )],
                    }),
                ),
            ],
        );

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
        expected.write_u8(XdlPrimitiveMetadata::I32 as u8).unwrap();
        // field b
        write_string("b", &mut expected).unwrap();
        expected.write_u8(VEC_METADATA_DISCRIMINANT).unwrap();
        expected.write_u8(XdlPrimitiveMetadata::I32 as u8).unwrap();
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
        expected.write_u8(XdlPrimitiveMetadata::I32 as u8).unwrap();

        assert_eq!(expected, writer);

        let mut reader = Cursor::new(writer);
        let deserialized = XdlMetadata::deserialize_base_metadata(&mut reader).unwrap();
        assert_eq!(XdlMetadata::Struct(metadata), deserialized);
    }
}
