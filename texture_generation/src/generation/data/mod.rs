use crate::math::aabb::AABB;
use crate::math::point::Point;

pub mod texture;

pub enum Data {
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

impl Data {
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
            Data::OneAabb { tile, depth, aabb } => Self::new_two(*tile, *depth, *aabb, inner),
            Data::TwoAabbs {
                tile, depth, outer, ..
            } => Self::new_two(*tile, *depth, *outer, inner),
        }
    }

    pub fn combine(&self) -> Self {
        match self {
            Data::OneAabb { tile, depth, aabb } => Self::new_one(*tile, *depth, *aabb),
            Data::TwoAabbs {
                tile,
                depth,
                outer,
                inner,
            } => Self::new_one(*tile, *depth, outer.limit(inner)),
        }
    }

    pub fn get_outer(&self) -> &AABB {
        match self {
            Data::OneAabb { aabb, .. } => aabb,
            Data::TwoAabbs { outer, .. } => outer,
        }
    }

    pub fn get_inner(&self) -> &AABB {
        match self {
            Data::OneAabb { aabb, .. } => aabb,
            Data::TwoAabbs { outer: inner, .. } => inner,
        }
    }

    pub fn get_start(&self) -> Point {
        match self {
            Data::OneAabb { aabb, .. } => aabb.start(),
            Data::TwoAabbs { outer, inner, .. } => outer.start().max(&inner.start()),
        }
    }

    pub fn get_end(&self) -> Point {
        match self {
            Data::OneAabb { aabb, .. } => aabb.end(),
            Data::TwoAabbs { outer, inner, .. } => outer.end().min(&inner.end()),
        }
    }
}
