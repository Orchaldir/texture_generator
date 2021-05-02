use crate::math::aabb::AABB;
use crate::math::point::Point;

pub enum GenerationData {
    OneAabb {
        tile: usize,
        depth: u32,
        aabb: AABB,
    },
    TwoAabbs {
        tile: usize,
        depth: u32,
        outer: AABB,
        inner: AABB,
    },
}

impl GenerationData {
    pub fn for_texture(aabb: AABB) -> Self {
        Self::new_one(0, 0, aabb)
    }

    pub fn for_tile(tile: usize, outer: AABB, inner: AABB) -> Self {
        Self::new_two(tile, 0, outer, inner)
    }

    pub fn new_one(tile: usize, depth: u32, aabb: AABB) -> Self {
        Self::OneAabb { tile, depth, aabb }
    }

    pub fn new_two(tile: usize, depth: u32, outer: AABB, inner: AABB) -> Self {
        Self::TwoAabbs {
            tile,
            depth,
            outer,
            inner,
        }
    }

    pub fn replace_inner(&self, inner: AABB) -> Self {
        match self {
            GenerationData::OneAabb { tile, depth, aabb } => {
                Self::new_two(*tile, *depth, *aabb, inner)
            }
            GenerationData::TwoAabbs {
                tile, depth, outer, ..
            } => Self::new_two(*tile, *depth, *outer, inner),
        }
    }

    pub fn combine(&self) -> Self {
        match self {
            GenerationData::OneAabb { tile, depth, aabb } => Self::new_one(*tile, *depth, *aabb),
            GenerationData::TwoAabbs {
                tile,
                depth,
                outer,
                inner,
            } => Self::new_one(*tile, *depth, outer.limit(inner)),
        }
    }

    pub fn get_outer(&self) -> &AABB {
        match self {
            GenerationData::OneAabb { aabb, .. } => aabb,
            GenerationData::TwoAabbs { outer, .. } => outer,
        }
    }

    pub fn get_inner(&self) -> &AABB {
        match self {
            GenerationData::OneAabb { aabb, .. } => aabb,
            GenerationData::TwoAabbs { outer: inner, .. } => inner,
        }
    }

    pub fn get_start(&self) -> Point {
        match self {
            GenerationData::OneAabb { aabb, .. } => aabb.start(),
            GenerationData::TwoAabbs { outer, inner, .. } => outer.start().max(&inner.start()),
        }
    }

    pub fn get_end(&self) -> Point {
        match self {
            GenerationData::OneAabb { aabb, .. } => aabb.end(),
            GenerationData::TwoAabbs { outer, inner, .. } => outer.end().min(&inner.end()),
        }
    }
}
