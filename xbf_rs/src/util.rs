use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Write};

pub fn write_string(string: &str, writer: &mut impl Write) -> io::Result<()> {
    writer.write_u64::<LittleEndian>(string.len() as u64)?;
    writer.write_all(string.as_bytes())
}

pub fn read_string(reader: &mut impl io::Read) -> io::Result<String> {
    let len = reader.read_u64::<LittleEndian>()?;
    let mut buf = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))
}

pub fn write_bytes(bytes: &[u8], writer: &mut impl Write) -> io::Result<()> {
    writer.write_u64::<LittleEndian>(bytes.len() as u64)?;
    writer.write_all(bytes)
}

pub fn read_bytes(reader: &mut impl io::Read) -> io::Result<Vec<u8>> {
    let len = reader.read_u64::<LittleEndian>()?;
    let mut buf = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}
