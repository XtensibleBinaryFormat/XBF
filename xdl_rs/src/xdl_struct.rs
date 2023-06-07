use crate::{XdlMetadata, XdlType};

#[derive(Debug, Clone, PartialEq)]
pub struct XdlStructMetadata {
    _name: String,
    // TODO: should this be a hashmap not a vec?
    _fields: Vec<(String, Box<XdlMetadata>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XdlStruct {
    _name: String,
    _fields: Vec<(String, XdlType)>,
}
