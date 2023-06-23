use super::XbfStructMetadata;
use crate::XbfType;

#[derive(Debug, Clone, PartialEq)]
pub struct XbfStruct {
    metadata: XbfStructMetadata,
    fields: Vec<XbfType>,
}
