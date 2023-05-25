use std::io::{self, Read, Write};

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub enum XdlType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Bool(bool),
    Vec(Vec<XdlType>),
    Struct(XdlStruct),
}

impl From<&XdlType> for u8 {
    fn from(xdltype: &XdlType) -> Self {
        match xdltype {
            XdlType::U8(_) => 0,
            XdlType::U16(_) => 1,
            XdlType::U32(_) => 2,
            XdlType::U64(_) => 3,
            XdlType::I8(_) => 4,
            XdlType::I16(_) => 5,
            XdlType::I32(_) => 6,
            XdlType::I64(_) => 7,
            XdlType::F32(_) => 8,
            XdlType::F64(_) => 9,
            XdlType::String(_) => 10,
            XdlType::Bool(_) => 11,
            XdlType::Vec(_) => 12,
            XdlType::Struct(_) => 13,
        }
    }
}

impl XdlType {
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_u8(self.into())?;

        match self {
            XdlType::U8(x) => buf.write_u8(*x)?,
            XdlType::U16(x) => buf.write_u16::<NetworkEndian>(*x)?,
            XdlType::U32(x) => buf.write_u32::<NetworkEndian>(*x)?,
            XdlType::U64(x) => buf.write_u64::<NetworkEndian>(*x)?,
            XdlType::I8(x) => buf.write_i8(*x)?,
            XdlType::I16(x) => buf.write_i16::<NetworkEndian>(*x)?,
            XdlType::I32(x) => buf.write_i32::<NetworkEndian>(*x)?,
            XdlType::I64(x) => buf.write_i64::<NetworkEndian>(*x)?,
            XdlType::F32(x) => buf.write_f32::<NetworkEndian>(*x)?,
            XdlType::F64(x) => buf.write_f64::<NetworkEndian>(*x)?,
            XdlType::String(x) => {
                let message = x.as_bytes();
                let len = message.len() as u16; // TODO: handle possibly longer or shorter string sizes?
                buf.write_u16::<NetworkEndian>(len)?;
                buf.write_all(message)?
            }
            XdlType::Bool(x) => buf.write_u8(u8::from(*x))?,
            _ => todo!(),
        };

        Ok(())
    }

    pub fn deserialize(buf: &mut impl Read) -> io::Result<Self> {
        match buf.read_u8()? {
            0 => Ok(XdlType::U8(buf.read_u8()?)),
            1 => Ok(XdlType::U16(buf.read_u16::<NetworkEndian>()?)),
            2 => Ok(XdlType::U32(buf.read_u32::<NetworkEndian>()?)),
            3 => Ok(XdlType::U64(buf.read_u64::<NetworkEndian>()?)),
            4 => Ok(XdlType::I8(buf.read_i8()?)),
            5 => Ok(XdlType::I16(buf.read_i16::<NetworkEndian>()?)),
            6 => Ok(XdlType::I32(buf.read_i32::<NetworkEndian>()?)),
            7 => Ok(XdlType::I64(buf.read_i64::<NetworkEndian>()?)),
            8 => Ok(XdlType::F32(buf.read_f32::<NetworkEndian>()?)),
            9 => Ok(XdlType::F64(buf.read_f64::<NetworkEndian>()?)),
            10 => {
                let string = exact_string(buf)?;
                Ok(XdlType::String(string))
            }
            11 => {
                let num = buf.read_u8()?;
                match num {
                    0 => Ok(XdlType::Bool(false)),
                    1 => Ok(XdlType::Bool(true)),
                    _ => todo!(), // TODO: this should theoretically be unreachable
                }
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct XdlStruct {}

fn exact_string(buf: &mut impl Read) -> io::Result<String> {
    let length = buf.read_u16::<NetworkEndian>()?;
    let mut bytes = vec![0u8; length as usize];
    buf.read_exact(&mut bytes)?;

    String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf8"))
}
