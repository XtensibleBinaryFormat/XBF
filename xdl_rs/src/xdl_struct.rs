use crate::{XdlMetadata, XdlType};

#[derive(Debug, Clone)]
pub struct XdlStruct {
    name: String,
    fields: Vec<(String, XdlType)>,
}

pub struct XdlStructMetadata {
    name: String,
    fields: Vec<(String, Box<XdlMetadata>)>,
}
