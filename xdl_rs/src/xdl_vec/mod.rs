mod vec;
mod vec_metadata;

pub use vec_metadata::*;

use crate::{XdlMetadata, XdlType};

#[derive(Debug, Clone, PartialEq)]
pub struct XdlVec {
    inner_type: Box<XdlMetadata>,
    elements: Vec<XdlType>,
}

impl XdlVec {
    pub fn new(
        inner_type: XdlMetadata,
        elements: Vec<XdlType>,
    ) -> Result<Self, ElementsNotHomogenousError> {
        let all_same_type = elements.iter().all(|x| inner_type == x.into());
        if all_same_type {
            Ok(Self {
                inner_type: Box::new(inner_type),
                elements,
            })
        } else {
            Err(ElementsNotHomogenousError)
        }
    }

    pub fn new_unchecked(inner_type: XdlMetadata, elements: Vec<XdlType>) -> Self {
        Self {
            inner_type: Box::new(inner_type),
            elements,
        }
    }
}

pub struct ElementsNotHomogenousError;
