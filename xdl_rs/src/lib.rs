mod xdl_primitive;
mod xdl_struct;
mod xdl_vec;

use xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata};
use xdl_struct::{XdlStruct, XdlStructMetadata};
use xdl_vec::{XdlVec, XdlVecMetadata};

#[derive(Debug, Clone)]
pub enum XdlType {
    Primitive(XdlPrimitive),
    Vec(XdlVec),
    Struct(XdlStruct),
}

pub enum XdlMetadata {
    Primitive(XdlPrimitiveMetadata),
    Vec(XdlVecMetadata),
    Struct(XdlStructMetadata),
}

// impl XdlMetadata {
//     pub fn new_primitive(type_id: XdlTypeId) -> Self {
//         Self::Primitive(XdlPrimitiveMetadata { type_id })
//     }
//
//     pub fn new_vec(inner_type_metadata: XdlMetadata) -> Self {
//         Self::Vec(XdlVecMetadata {
//             inner_type_metadata: Box::new(inner_type_metadata),
//         })
//     }
//
//     pub fn new_struct(name: String, fields: Vec<(String, Box<XdlMetadata>)>) -> Self {
//         Self::Struct(XdlStructMetadata { name, fields })
//     }
// }

impl XdlType {
    // pub fn serialize_with_metadata(&self, writer: &mut impl Write) -> io::Result<()> {
    //     writer.write_u8(self.get_type_id() as u8)?;
    //     if let XdlType::Vec(v) = self {
    //         writer.write_u8(v.inner_type_id as u8)?;
    //     }
    //     self.serialize_no_type_id(writer)
    // }
    //
    // pub fn serialize_no_type_id(&self, writer: &mut impl Write) -> io::Result<()> {
    //     match self {
    //         XdlType::U8(x) => writer.write_u8(*x),
    //         XdlType::U16(x) => writer.write_u16::<NetworkEndian>(*x),
    //         XdlType::U32(x) => writer.write_u32::<NetworkEndian>(*x),
    //         XdlType::U64(x) => writer.write_u64::<NetworkEndian>(*x),
    //         XdlType::I8(x) => writer.write_i8(*x),
    //         XdlType::I16(x) => writer.write_i16::<NetworkEndian>(*x),
    //         XdlType::I32(x) => writer.write_i32::<NetworkEndian>(*x),
    //         XdlType::I64(x) => writer.write_i64::<NetworkEndian>(*x),
    //         XdlType::F32(x) => writer.write_f32::<NetworkEndian>(*x),
    //         XdlType::F64(x) => writer.write_f64::<NetworkEndian>(*x),
    //         XdlType::String(x) => {
    //             writer.write_u16::<NetworkEndian>(x.len() as u16)?;
    //             writer.write_all(x.as_bytes())
    //         }
    //         XdlType::Bool(x) => writer.write_u8(u8::from(*x)),
    //         XdlType::Vec(x) => {
    //             writer.write_u16::<NetworkEndian>(x.elements.len() as u16)?;
    //             for x in x.elements.iter() {
    //                 x.serialize_no_type_id(writer)?;
    //             }
    //             Ok(())
    //         }
    //         _ => todo!(),
    //     }
    // }
    //
    // pub fn deserialize_read_metadata(reader: &mut impl Read) -> io::Result<Self> {
    //     let type_id: XdlTypeId = reader.read_u8()?.into();
    //
    //     if type_id == XdlTypeId::Vec {
    //         let inner_type_id: XdlTypeId = reader.read_u8()?.into();
    //     }
    //
    //     if type_id == XdlTypeId::Struct {
    //         todo!()
    //     }
    //
    //     let metadata = XdlMetadata::new_primitive(type_id);
    //     Self::deserialize_with_metadata(metadata, reader)
    // }
    //
    // pub fn deserialize_with_metadata(
    //     metadata: XdlMetadata,
    //     reader: &mut impl Read,
    // ) -> io::Result<Self> {
    //     let type_id = metadata.type_id;
    //     Ok(match type_id {
    //         XdlTypeId::U8 => XdlType::U8(reader.read_u8()?),
    //         XdlTypeId::U16 => XdlType::U16(reader.read_u16::<NetworkEndian>()?),
    //         XdlTypeId::U32 => XdlType::U32(reader.read_u32::<NetworkEndian>()?),
    //         XdlTypeId::U64 => XdlType::U64(reader.read_u64::<NetworkEndian>()?),
    //         XdlTypeId::I8 => XdlType::I8(reader.read_i8()?),
    //         XdlTypeId::I16 => XdlType::I16(reader.read_i16::<NetworkEndian>()?),
    //         XdlTypeId::I32 => XdlType::I32(reader.read_i32::<NetworkEndian>()?),
    //         XdlTypeId::I64 => XdlType::I64(reader.read_i64::<NetworkEndian>()?),
    //         XdlTypeId::F32 => XdlType::F32(reader.read_f32::<NetworkEndian>()?),
    //         XdlTypeId::F64 => XdlType::F64(reader.read_f64::<NetworkEndian>()?),
    //         XdlTypeId::String => {
    //             let len = reader.read_u16::<NetworkEndian>()?;
    //             let mut buf = vec![0u8; len as usize];
    //             reader.read_exact(&mut buf)?;
    //             let s = String::from_utf8(buf)
    //                 .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid utf8"))?;
    //             XdlType::String(s)
    //         }
    //         XdlTypeId::Bool => {
    //             let x = reader.read_u8()?;
    //             XdlType::Bool(x != 0)
    //         }
    //         XdlTypeId::Struct => todo!(),
    //     })
    // }
}
