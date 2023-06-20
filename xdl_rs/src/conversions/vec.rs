use crate::xdl_vec::{XdlVec, XdlVecMetadata};

impl From<&XdlVec> for XdlVecMetadata {
    fn from(value: &XdlVec) -> Self {
        Self::new(value.inner_type.clone())
    }
}
