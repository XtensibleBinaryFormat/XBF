#[derive(Debug, Clone)]
pub struct XdlVec {
    // inner_type_id: XdlTypeId,
    // elements: Vec<XdlType>,
}

pub struct XdlVecMetadata {}
//
// impl XdlVec {
//     pub fn new(inner_type_id: XdlTypeId, elements: Vec<XdlType>) -> Self {
//         // it IS possible to have a vec of vecs where the inner vecs have different inner types
//         // unknown whether this is intended by the spec
//         // leaving it like this would make this code significantly less complicated
//         assert!(elements.iter().all(|x| x.get_type_id() == inner_type_id));
//
//         Self {
//             inner_type_id,
//             elements,
//         }
//     }
// }
