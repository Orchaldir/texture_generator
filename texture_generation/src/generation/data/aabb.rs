use crate::math::aabb::AABB;

pub enum AabbData {
    OneAabb(AABB),
    TwoAabbs { outer: AABB, inner: AABB },
}

impl AabbData {
    pub fn next(&self, inner: AABB) -> Self {
        AabbData::TwoAabbs {
            outer: match self {
                AabbData::OneAabb(aabb) => *aabb,
                AabbData::TwoAabbs { outer, .. } => *outer,
            },
            inner,
        }
    }

    pub fn combine(&self) -> Self {
        AabbData::OneAabb(match self {
            AabbData::OneAabb(aabb) => *aabb,
            AabbData::TwoAabbs { outer, inner } => outer.limit(inner),
        })
    }
}
