use crate::xbf_vec::{XbfVec, XbfVecMetadata};

impl From<&XbfVec> for XbfVecMetadata {
    fn from(value: &XbfVec) -> Self {
        Self::from_boxed_type(value.inner_type.clone())
    }
}
