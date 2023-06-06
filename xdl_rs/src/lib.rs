mod xdl_primitive;
mod xdl_struct;
mod xdl_vec;

use byteorder::WriteBytesExt;
use std::io::{self, Read, Write};
use xdl_primitive::{XdlPrimitive, XdlPrimitiveId, XdlPrimitiveMetadata};
use xdl_struct::{XdlStruct, XdlStructMetadata};
use xdl_vec::{XdlVec, XdlVecMetadata};

pub enum XdlMetadata {
    Primitive(XdlPrimitiveMetadata),
    Vec(XdlVecMetadata),
    Struct(XdlStructMetadata),
}

impl XdlMetadata {
    pub fn new_primitive_metadata(type_id: XdlPrimitiveId) -> Self {
        XdlMetadata::Primitive(XdlPrimitiveMetadata(type_id))
    }

    pub fn new_vec_metadata(_inner_type: XdlMetadata) -> Self {
        todo!()
    }

    pub fn new_struct_metadata(_spec: XdlStructMetadata) -> Self {
        todo!()
    }

    pub fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            XdlMetadata::Primitive(x) => writer.write_u8(x.0 as u8),
            XdlMetadata::Vec(_x) => todo!(),
            XdlMetadata::Struct(_x) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum XdlType {
    Primitive(XdlPrimitive),
    Vec(XdlVec),
    Struct(XdlStruct),
}

impl XdlType {
    pub fn deserialize_unknown_metadata(_reader: &mut impl Read) -> io::Result<Self> {
        todo!()
    }
}
