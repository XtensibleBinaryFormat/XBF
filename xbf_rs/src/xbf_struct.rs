use crate::{XbfMetadata, XbfType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbfStructMetadata {
    _name: String,
    // TODO: add a hashmap for quick lookups
    _fields: Vec<(String, Box<XbfMetadata>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XbfStruct {
    _name: String,
    _fields: Vec<(String, XbfType)>,
}
