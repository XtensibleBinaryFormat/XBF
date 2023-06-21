use crate::xdl_vec::{XdlVec, XdlVecMetadata};

impl From<&XdlVec> for XdlVecMetadata {
    fn from(value: &XdlVec) -> Self {
        Self::from_boxed_type(value.inner_type.clone())
    }
}
