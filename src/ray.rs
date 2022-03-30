use crate::v3::V3;

#[derive(Debug)]
pub struct Ray {
    pub start: V3,
    // normalized
    pub direction: V3,
}
