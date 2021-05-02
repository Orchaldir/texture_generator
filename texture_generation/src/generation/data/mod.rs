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
        AabbData::OneAabb(match &self.aabb_data {
            AabbData::OneAabb(aabb) => *aabb,
            AabbData::TwoAabbs { outer, .. } => outer.limit(inner),
        })
    }
}

pub struct Data {
    tile: usize,
    depth: u32,
    aabb_data: AabbData,
}

impl Data {
    pub fn for_texture(aabb: AABB) -> Self {
        Self::new(0, 0, AabbData::OneAabb(aabb))
    }

    pub fn for_tile(tile: usize, outer: AABB, inner: AABB) -> Self {
        Self::new(tile, 0, AabbData::TwoAabbs { outer, inner })
    }

    fn new(tile: usize, depth: u32, aabb_data: AabbData) -> Self {
        Self {
            tile,
            depth,
            aabb_data,
        }
    }

    pub fn next(&self, inner: AABB) -> Self {
        Self::new(self.tile, self.depth, self.aabb_data.next(inner))
    }

    pub fn combine(&self) -> Self {
        Self::new(self.tile, self.depth, self.aabb_data.combine())
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
