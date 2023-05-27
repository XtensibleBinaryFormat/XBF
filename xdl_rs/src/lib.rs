use std::io::{self, Read, Write};

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum XdlTypeId {
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

impl From<u8> for XdlTypeId {
    fn from(x: u8) -> Self {
        match x {
            0 => XdlTypeId::U8,
            1 => XdlTypeId::U16,
            2 => XdlTypeId::U32,
            3 => XdlTypeId::U64,
            4 => XdlTypeId::I8,
            5 => XdlTypeId::I16,
            6 => XdlTypeId::I32,
            7 => XdlTypeId::I64,
            8 => XdlTypeId::F32,
            9 => XdlTypeId::F64,
            10 => XdlTypeId::String,
            11 => XdlTypeId::Bool,
            12 => XdlTypeId::Vec,
            13 => XdlTypeId::Struct,
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

macro_rules! xdl_from_impl {
    ($ty:ty, $xdl_type:tt) => {
        impl From<$ty> for XdlType {
            fn from(x: $ty) -> Self {
                XdlType::$xdl_type(x)
            }
        }
    };
}

xdl_from_impl!(u8, U8);
xdl_from_impl!(u16, U16);
xdl_from_impl!(u32, U32);
xdl_from_impl!(u64, U64);
xdl_from_impl!(i8, I8);
xdl_from_impl!(i16, I16);
xdl_from_impl!(i32, I32);
xdl_from_impl!(i64, I64);
xdl_from_impl!(f32, F32);
xdl_from_impl!(f64, F64);
xdl_from_impl!(String, String);
xdl_from_impl!(bool, Bool);

#[derive(Debug, Clone)]
pub struct XdlVec {
    inner_type: XdlTypeId,
    data: Vec<XdlType>,
}

impl XdlVec {
    pub fn new(vec: Vec<XdlType>, inner_type: XdlTypeId) -> Self {
        // it IS possible to have a vec of vecs where the inner vecs have different inner types
        // unknown whether this is intended by the spec
        // leaving it like this would make this code significantly less complicated
        assert!(vec.iter().all(|x| x.get_type_id() == inner_type));

        Self {
            inner_type,
            data: vec,
        }
    }

    fn serialize_vec(
        &self,
        send_inner_type_id: SendInnerVecTypeId,
        buf: &mut impl Write,
    ) -> io::Result<()> {
        let len = self.data.len() as u16;
        buf.write_u16::<NetworkEndian>(len)?;
        match send_inner_type_id {
            SendInnerVecTypeId::Yes => {
                buf.write_u8(self.inner_type as u8)?;
            }
            SendInnerVecTypeId::No => {}
        }
        for x in self.data.iter() {
            x.serialize_no_type_id(buf)?;
        }
        Ok(())
    }
}

enum SendInnerVecTypeId {
    Yes,
    No,
}

impl XdlType {
    fn get_type_id(&self) -> XdlTypeId {
        match self {
            XdlType::U8(_) => XdlTypeId::U8,
            XdlType::U16(_) => XdlTypeId::U16,
            XdlType::U32(_) => XdlTypeId::U32,
            XdlType::U64(_) => XdlTypeId::U64,
            XdlType::I8(_) => XdlTypeId::I8,
            XdlType::I16(_) => XdlTypeId::I16,
            XdlType::I32(_) => XdlTypeId::I32,
            XdlType::I64(_) => XdlTypeId::I64,
            XdlType::F32(_) => XdlTypeId::F32,
            XdlType::F64(_) => XdlTypeId::F64,
            XdlType::String(_) => XdlTypeId::String,
            XdlType::Bool(_) => XdlTypeId::Bool,
            XdlType::Vec(_) => XdlTypeId::Vec,
            XdlType::Struct(_) => XdlTypeId::Struct,
        }
    }

    pub fn serialize_no_type_id(&self, buf: &mut impl Write) -> io::Result<()> {
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
            XdlType::Vec(x) => (*x).serialize_vec(SendInnerVecTypeId::No, buf)?,
            _ => todo!(),
        };

        Ok(())
    }

    pub fn serialize_with_type_id(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_u8(self.get_type_id() as u8)?;
        if let XdlType::Vec(x) = self {
            x.serialize_vec(SendInnerVecTypeId::Yes, buf)?
        } else {
            self.serialize_no_type_id(buf)?;
        }
        Ok(())
    }

    pub fn deserialize_with_id(type_id: XdlTypeId, buf: &mut impl Read) -> io::Result<Self> {
        Ok(match type_id {
            XdlTypeId::U8 => buf.read_u8()?.into(),
            XdlTypeId::U16 => buf.read_u16::<NetworkEndian>()?.into(),
            XdlTypeId::U32 => buf.read_u32::<NetworkEndian>()?.into(),
            XdlTypeId::U64 => buf.read_u64::<NetworkEndian>()?.into(),
            XdlTypeId::I8 => buf.read_i8()?.into(),
            XdlTypeId::I16 => buf.read_i16::<NetworkEndian>()?.into(),
            XdlTypeId::I32 => buf.read_i32::<NetworkEndian>()?.into(),
            XdlTypeId::I64 => buf.read_i64::<NetworkEndian>()?.into(),
            XdlTypeId::F32 => buf.read_f32::<NetworkEndian>()?.into(),
            XdlTypeId::F64 => buf.read_f64::<NetworkEndian>()?.into(),
            XdlTypeId::String => exact_string(buf)?.into(),
            XdlTypeId::Bool => match buf.read_u8()? {
                0 => false,
                1 => true,
                _ => unreachable!(),
            }
            .into(),
            XdlTypeId::Vec => {
                let len = buf.read_u16::<NetworkEndian>()?;
                let vec_inner_type_id = buf.read_u8()?.into();
                XdlType::deserialize_consecutive_with_id(vec_inner_type_id, len, buf)?
            }
            XdlTypeId::Struct => todo!(),
        })
    }

    pub fn deserialize_consecutive_with_id(
        type_id: XdlTypeId,
        number: u16,
        buf: &mut impl Read,
    ) -> io::Result<Self> {
        let mut vec = vec![];
        vec.reserve(number as usize);

        for _ in 0..number {
            let val = match type_id {
                XdlTypeId::Vec => {
                    let len = buf.read_u16::<NetworkEndian>()?;
                    let vec_inner_type_id = buf.read_u8()?.into();
                    XdlType::deserialize_consecutive_with_id(vec_inner_type_id, len, buf)?
                }
                _ => XdlType::deserialize_with_id(type_id, buf)?,
            };
            vec.push(val);
        }
        Ok(XdlType::Vec(XdlVec::new(vec, type_id)))
    }

    pub fn deserialize(buf: &mut impl Read) -> io::Result<Self> {
        let type_id = buf.read_u8()?.into();
        dbg!(type_id);
        Ok(XdlType::deserialize_with_id(type_id, buf)?)
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
