use crate::{XdlMetadata, XdlType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XdlStructMetadata {
    _name: String,
    // TODO: add a hashmap for quick lookups
    _fields: Vec<(String, Box<XdlMetadata>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XdlStruct {
    _name: String,
    _fields: Vec<(String, XdlType)>,
}
