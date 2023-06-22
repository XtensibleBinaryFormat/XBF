use crate::xbf_vec::{XbfVec, XbfVecMetadata};

impl From<&XbfVec> for XbfVecMetadata {
    fn from(value: &XbfVec) -> Self {
        Self::new(value.inner_type.clone())
    }
}
