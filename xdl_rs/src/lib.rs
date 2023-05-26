use std::io::{self, Read, Write};

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum XdlDiscriminant {
    U8 = 0,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Bool,
    Vec,
    Struct,
}

impl From<u8> for XdlDiscriminant {
    fn from(x: u8) -> Self {
        match x {
            0 => XdlDiscriminant::U8,
            1 => XdlDiscriminant::U16,
            2 => XdlDiscriminant::U32,
            3 => XdlDiscriminant::U64,
            4 => XdlDiscriminant::I8,
            5 => XdlDiscriminant::I16,
            6 => XdlDiscriminant::I32,
            7 => XdlDiscriminant::I64,
            8 => XdlDiscriminant::F32,
            9 => XdlDiscriminant::F64,
            10 => XdlDiscriminant::String,
            11 => XdlDiscriminant::Bool,
            12 => XdlDiscriminant::Vec,
            13 => XdlDiscriminant::Struct,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
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
    Vec(XdlVec),
    Struct(XdlStruct),
}

#[derive(Debug, Clone)]
pub struct XdlVec {
    dis: XdlDiscriminant,
    data: Vec<XdlType>,
}

impl XdlVec {
    pub fn new(vec: Vec<XdlType>, discriminant: XdlDiscriminant) -> Self {
        assert!(vec.iter().all(|x| x.get_discriminant() == discriminant));
        Self {
            dis: discriminant,
            data: vec,
        }
    }
}

impl XdlType {
    pub fn get_discriminant(&self) -> XdlDiscriminant {
        match self {
            XdlType::U8(_) => XdlDiscriminant::U8,
            XdlType::U16(_) => XdlDiscriminant::U16,
            XdlType::U32(_) => XdlDiscriminant::U32,
            XdlType::U64(_) => XdlDiscriminant::U64,
            XdlType::I8(_) => XdlDiscriminant::I8,
            XdlType::I16(_) => XdlDiscriminant::I16,
            XdlType::I32(_) => XdlDiscriminant::I32,
            XdlType::I64(_) => XdlDiscriminant::I64,
            XdlType::F32(_) => XdlDiscriminant::F32,
            XdlType::F64(_) => XdlDiscriminant::F64,
            XdlType::String(_) => XdlDiscriminant::String,
            XdlType::Bool(_) => XdlDiscriminant::Bool,
            XdlType::Vec(_) => XdlDiscriminant::Vec,
            XdlType::Struct(_) => XdlDiscriminant::Struct,
        }
    }
    pub fn serialize_no_discriminant(&self, buf: &mut impl Write) -> io::Result<()> {
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
            XdlType::Vec(x) => {
                let len = x.data.len() as u16;
                buf.write_u16::<NetworkEndian>(len)?;
                let discriminant = x.dis;
                buf.write_u8(discriminant as u8)?;
                for x in x.data.iter() {
                    x.serialize_no_discriminant(buf)?
                }
            }
            _ => todo!(),
        };

        Ok(())
    }

    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_u8(self.get_discriminant() as u8)?;
        self.serialize_no_discriminant(buf)?;
        Ok(())
    }

    fn deserialize_vec(
        discriminant: XdlDiscriminant,
        len: u16,
        buf: &mut impl Read,
    ) -> io::Result<Self> {
        let mut vec = vec![];
        vec.reserve(len as usize);

        for _ in 0..len {
            match discriminant {
                XdlDiscriminant::U8 => vec.push(XdlType::U8(buf.read_u8()?)),
                XdlDiscriminant::U16 => vec.push(XdlType::U16(buf.read_u16::<NetworkEndian>()?)),
                XdlDiscriminant::U32 => vec.push(XdlType::U32(buf.read_u32::<NetworkEndian>()?)),
                XdlDiscriminant::U64 => vec.push(XdlType::U64(buf.read_u64::<NetworkEndian>()?)),
                XdlDiscriminant::I8 => vec.push(XdlType::I8(buf.read_i8()?)),
                XdlDiscriminant::I16 => vec.push(XdlType::I16(buf.read_i16::<NetworkEndian>()?)),
                XdlDiscriminant::I32 => vec.push(XdlType::I32(buf.read_i32::<NetworkEndian>()?)),
                XdlDiscriminant::I64 => vec.push(XdlType::I64(buf.read_i64::<NetworkEndian>()?)),
                XdlDiscriminant::F32 => vec.push(XdlType::F32(buf.read_f32::<NetworkEndian>()?)),
                XdlDiscriminant::F64 => vec.push(XdlType::F64(buf.read_f64::<NetworkEndian>()?)),
                XdlDiscriminant::String => vec.push(XdlType::String(exact_string(buf)?)),
                XdlDiscriminant::Bool => vec.push(XdlType::Bool(match buf.read_u8()? {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                })),
                XdlDiscriminant::Vec => {
                    let len = buf.read_u16::<NetworkEndian>()?;
                    let dis = buf.read_u8()?.into();
                    vec.push(XdlType::deserialize_vec(dis, len, buf)?);
                }
                _ => todo!(),
            }
        }
        Ok(XdlType::Vec(XdlVec::new(vec, discriminant)))
    }

    pub fn deserialize(buf: &mut impl Read) -> io::Result<Self> {
        let discriminant = buf.read_u8()?.into();
        match discriminant {
            XdlDiscriminant::U8 => Ok(XdlType::U8(buf.read_u8()?)),
            XdlDiscriminant::U16 => Ok(XdlType::U16(buf.read_u16::<NetworkEndian>()?)),
            XdlDiscriminant::U32 => Ok(XdlType::U32(buf.read_u32::<NetworkEndian>()?)),
            XdlDiscriminant::U64 => Ok(XdlType::U64(buf.read_u64::<NetworkEndian>()?)),
            XdlDiscriminant::I8 => Ok(XdlType::I8(buf.read_i8()?)),
            XdlDiscriminant::I16 => Ok(XdlType::I16(buf.read_i16::<NetworkEndian>()?)),
            XdlDiscriminant::I32 => Ok(XdlType::I32(buf.read_i32::<NetworkEndian>()?)),
            XdlDiscriminant::I64 => Ok(XdlType::I64(buf.read_i64::<NetworkEndian>()?)),
            XdlDiscriminant::F32 => Ok(XdlType::F32(buf.read_f32::<NetworkEndian>()?)),
            XdlDiscriminant::F64 => Ok(XdlType::F64(buf.read_f64::<NetworkEndian>()?)),
            XdlDiscriminant::String => {
                let string = exact_string(buf)?;
                Ok(XdlType::String(string))
            }
            XdlDiscriminant::Bool => {
                let num = buf.read_u8()?;
                match num {
                    0 => Ok(XdlType::Bool(false)),
                    1 => Ok(XdlType::Bool(true)),
                    _ => unreachable!(),
                }
            }
            XdlDiscriminant::Vec => {
                let len = buf.read_u16::<NetworkEndian>()?;
                let discriminant = buf.read_u8()?.into();
                XdlType::deserialize_vec(discriminant, len, buf)
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XdlStruct {
    name: String,
    fields: Vec<(String, XdlType)>,
}

// TODO: equivalent of this that's more generic for a list
// ie. return a vec<u8> instead of String, then have separate fn
// to convert those bytes to a string with a wrapper fn
// TODO: should this follow conventions of other read and write functions and take a mutable
// reference?
fn exact_string(buf: &mut impl Read) -> io::Result<String> {
    let length = buf.read_u16::<NetworkEndian>()?;
    let mut bytes = vec![0u8; length as usize];
    buf.read_exact(&mut bytes)?;

    String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf8"))
}
