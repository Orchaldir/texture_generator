use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::size::Size;

pub enum AabbData {
    OneAabb(Size, AABB),
    TwoAabbs {
        /// The size of the whole texture needed for rotating the origin of it.
        texture_size: Size,
        outer: AABB,
        inner: AABB,
    },
}

impl AabbData {
    pub fn from_one_aabb(aabb: AABB) -> Self {
        AabbData::OneAabb(aabb.size(), aabb)
    }

    pub fn from_two_aabb(outer: AABB, inner: AABB) -> Self {
        AabbData::TwoAabbs {
            texture_size: outer.size(),
            outer,
            inner,
        }
    }

    pub fn two(texture_size: Size, outer: AABB, inner: AABB) -> Self {
        AabbData::TwoAabbs {
            texture_size,
            outer,
            inner,
        }
    }

    pub fn get_outer(&self) -> &AABB {
        match self {
            AabbData::OneAabb(_size, aabb) => aabb,
            AabbData::TwoAabbs { outer, .. } => outer,
        }
    }

    pub fn get_inner(&self) -> &AABB {
        match self {
            AabbData::OneAabb(_size, aabb) => aabb,
            AabbData::TwoAabbs { inner, .. } => inner,
        }
    }

    /// Get the start point fo the combined [`AABB`]s.
    pub fn get_start(&self) -> Point {
        match self {
            AabbData::OneAabb(_size, aabb) => aabb.start(),
            AabbData::TwoAabbs { outer, inner, .. } => outer.start().max(&inner.start()),
        }
    }

    /// Get the end point fo the combined [`AABB`]s.
    pub fn get_end(&self) -> Point {
        match self {
            AabbData::OneAabb(_size, aabb) => aabb.end(),
            AabbData::TwoAabbs { outer, inner, .. } => outer.end().min(&inner.end()),
        }
    }

    /// Add or replaces the inner [`AABB`].
    pub fn next(&self, inner: AABB) -> Self {
        match self {
            AabbData::OneAabb(texture_size, aabb) => AabbData::two(*texture_size, *aabb, inner),
            AabbData::TwoAabbs {
                texture_size,
                outer,
                ..
            } => AabbData::two(*texture_size, *outer, inner),
        }
    }

    /// Combines the 2 [`AABB`]s into 1 if available.
    pub fn combine(&self) -> Self {
        match self {
            AabbData::OneAabb(texture_size, aabb) => AabbData::OneAabb(*texture_size, *aabb),
            AabbData::TwoAabbs {
                texture_size,
                outer,
                inner,
            } => AabbData::OneAabb(*texture_size, outer.limit(inner)),
        }
    }

    /// Rotates the origin of the texture clockwise.
    pub fn rotate_origin(&self) -> Self {
        match self {
            AabbData::OneAabb(texture_size, aabb) => {
                AabbData::OneAabb(texture_size.flip(), aabb.rotate_origin(*texture_size))
            }
            AabbData::TwoAabbs {
                texture_size,
                outer,
                inner,
            } => AabbData::two(
                texture_size.flip(),
                outer.rotate_origin(*texture_size),
                inner.rotate_origin(*texture_size),
            ),
        }
    }

    /// Rotates the origin of the texture counter clockwise.
    pub fn rotate_origin_revers(&self) -> Self {
        match self {
            AabbData::OneAabb(texture_size, aabb) => AabbData::OneAabb(
                texture_size.flip(),
                aabb.rotate_origin_reverse(*texture_size),
            ),
            AabbData::TwoAabbs {
                texture_size,
                outer,
                inner,
            } => AabbData::two(
                texture_size.flip(),
                outer.rotate_origin_reverse(*texture_size),
                inner.rotate_origin_reverse(*texture_size),
            ),
        }
    }
}
