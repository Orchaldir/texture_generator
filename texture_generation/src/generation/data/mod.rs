use crate::math::aabb::AABB;
use crate::math::point::Point;

pub mod texture;

enum AabbData {
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

pub struct Data {
    /// The `global_id` is 0, if the plan is to generate a simple texture and not a tilemap.
    /// Otherwise it is the id of the current tile or edge.
    global_id: usize,
    instance_id: usize,
    aabb_data: AabbData,
}

impl Data {
    pub fn for_texture(aabb: AABB) -> Self {
        Self::new(0, 0, AabbData::OneAabb(aabb))
    }

    pub fn with_global_id(global_id: usize, aabb: AABB) -> Self {
        Self::new(global_id, 0, AabbData::OneAabb(aabb))
    }

    pub fn for_two_aabb(global_id: usize, outer: AABB, inner: AABB) -> Self {
        Self::new(global_id, 0, AabbData::TwoAabbs { outer, inner })
    }

    fn new(global_id: usize, instance_id: usize, aabb_data: AabbData) -> Self {
        Self {
            global_id,
            instance_id,
            aabb_data,
        }
    }

    pub fn next(&self, inner: AABB) -> Self {
        Self::new(self.global_id, self.instance_id, self.aabb_data.next(inner))
    }

    pub fn combine(&self) -> Self {
        Self::new(self.global_id, self.instance_id, self.aabb_data.combine())
    }

    pub fn get_global_id(&self) -> usize {
        self.global_id
    }

    pub fn get_outer(&self) -> &AABB {
        match &self.aabb_data {
            AabbData::OneAabb(aabb) => aabb,
            AabbData::TwoAabbs { outer, .. } => outer,
        }
    }

    pub fn get_inner(&self) -> &AABB {
        match &self.aabb_data {
            AabbData::OneAabb(aabb) => aabb,
            AabbData::TwoAabbs { inner, .. } => inner,
        }
    }

    pub fn get_start(&self) -> Point {
        match &self.aabb_data {
            AabbData::OneAabb(aabb) => aabb.start(),
            AabbData::TwoAabbs { outer, inner, .. } => outer.start().max(&inner.start()),
        }
    }

    pub fn get_end(&self) -> Point {
        match &self.aabb_data {
            AabbData::OneAabb(aabb) => aabb.end(),
            AabbData::TwoAabbs { outer, inner, .. } => outer.end().min(&inner.end()),
        }
    }
}
