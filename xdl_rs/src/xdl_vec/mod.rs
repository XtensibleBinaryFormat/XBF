use crate::{XdlMetadata, XdlType};

#[derive(Debug, Clone, PartialEq)]
pub struct XdlVecMetadata {
    inner_type: Box<XdlMetadata>,
}

impl XdlVecMetadata {
    pub fn new(inner_type: XdlMetadata) -> Self {
        Self {
            inner_type: Box::new(inner_type),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct XdlVec {
    _inner_type: Box<XdlMetadata>,
    _elements: Vec<XdlType>,
}

// impl XdlVec {
//     pub fn new(inner_type_id: XdlTypeId, elements: Vec<XdlType>) -> Self {
//         // it IS possible to have a vec of vecs where the inner vecs have different inner types
//         // unknown whether this is intended by the spec
//         // leaving it like this would make this code significantly less complicated
//         assert!(elements.iter().all(|x| x.get_type_id() == inner_type_id));
//
//         todo!()
//     }
// }
