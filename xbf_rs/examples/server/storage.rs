use std::io::{self, Read, Write};
use xbf_rs::{XbfMetadata, XbfType};

pub struct SerializedStorage {
    value: XbfType,
    serialized_metadata: Vec<u8>,
    serialized_value: Vec<u8>,
}

impl SerializedStorage {
    pub fn new(value: impl Into<XbfType>) -> Self {
        let value = value.into();

        let mut serialized_metadata = vec![];
        XbfMetadata::from(&value)
            .serialize_base_metadata(&mut serialized_metadata)
            .expect("serialization of metadata to internal buffer failed");

        let mut serialized_value = vec![];
        value
            .serialize_base_type(&mut serialized_value)
            .expect("serialization of value to internal buffer failed");

        Self {
            value,
            serialized_metadata,
            serialized_value,
        }
    }

    pub fn write_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.serialized_metadata)
    }

    pub fn write_value(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.serialized_value)
    }

    pub fn from_reader(reader: &mut impl Read) -> io::Result<Self> {
        let metadata = XbfMetadata::deserialize_base_metadata(reader)?;
        let value = XbfType::deserialize_base_type(&metadata, reader)?;

        Ok(Self::new(value))
    }
}
