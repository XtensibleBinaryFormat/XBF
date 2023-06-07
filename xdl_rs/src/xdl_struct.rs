use crate::{XdlMetadata, XdlType};

#[derive(Debug, Clone)]
pub struct XdlStruct {
    _name: String,
    _fields: Vec<(String, XdlType)>,
}

pub struct XdlStructMetadata {
    _name: String,
    _fields: Vec<(String, Box<XdlMetadata>)>,
}
