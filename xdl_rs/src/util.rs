use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Write};

pub fn write_string(string: &str, writer: &mut impl Write) -> io::Result<()> {
    writer.write_u16::<LittleEndian>(string.len() as u16)?;
    writer.write_all(string.as_bytes())
}
