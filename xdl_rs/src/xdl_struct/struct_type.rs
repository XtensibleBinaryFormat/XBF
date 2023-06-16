use super::XdlStructMetadata;
use crate::XdlType;

#[derive(Debug, Clone, PartialEq)]
pub struct XdlStruct {
    metadata: XdlStructMetadata,
    fields: Vec<XdlType>,
}
