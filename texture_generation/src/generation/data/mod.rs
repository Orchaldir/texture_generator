use crate::generation::data::aabb::AabbData;
use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::side::Side;

pub mod aabb;
pub mod texture;

pub struct Data {
    /// The `global_id` is 0, if the plan is to generate a simple texture and not a tilemap.
    /// Otherwise it is the id of the current tile or edge.
    global_id: usize,
    /// Each instance of a [`Component`] used by a [`LayoutComponent`] has its own id.
    /// Used for variations between instances.
    instance_id: usize,
    aabb_data: AabbData,
    orientation: Side,
    is_horizontal: bool,
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

    pub fn only_instance_id(instance_id: usize) -> Self {
        Self::new(0, instance_id, AabbData::OneAabb(AABB::default()))
    }

    pub fn new(global_id: usize, instance_id: usize, aabb_data: AabbData) -> Self {
        Self::full(global_id, instance_id, aabb_data, Side::Right, true)
    }

    pub fn full(
        global_id: usize,
        instance_id: usize,
        aabb_data: AabbData,
        orientation: Side,
        is_horizontal: bool,
    ) -> Self {
        Self {
            global_id,
            instance_id,
            aabb_data,
            orientation,
            is_horizontal,
        }
    }

    /// Updates the inner [`AABB`] for a [`Component`]. Keeps `instance_id`.
    pub fn transform(&self, inner: AABB) -> Self {
        Self::new(self.global_id, self.instance_id, self.aabb_data.next(inner))
    }

    /// Replace the inner [`AABB`] for the next instance of the same [`Component`]. Increases `instance_id`.
    pub fn next(&mut self, inner: AABB) -> Self {
        let old_id = self.instance_id;
        self.instance_id += 1;
        Self::new(self.global_id, old_id, self.aabb_data.next(inner))
    }

    /// Replaces the inner [`AABB`] for the next instance of the same [`Component`]. Overwrites `instance_id`.
    pub fn set(&self, instance_id: usize, inner: AABB) -> Self {
        Self::new(self.global_id, instance_id, self.aabb_data.next(inner))
    }

    /// Combines the inner & outer [`AABB`]s into the inner one,
    pub fn combine(&self) -> Self {
        Self::new(self.global_id, self.instance_id, self.aabb_data.combine())
    }

    pub fn get_global_id(&self) -> usize {
        self.global_id
    }

    pub fn get_instance_id(&self) -> usize {
        self.instance_id
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

    /// Get the start point fo the combined [`AABB`]s.
    pub fn get_start(&self) -> Point {
        match &self.aabb_data {
            AabbData::OneAabb(aabb) => aabb.start(),
            AabbData::TwoAabbs { outer, inner, .. } => outer.start().max(&inner.start()),
        }
    }

    /// Get the end point fo the combined [`AABB`]s.
    pub fn get_end(&self) -> Point {
        match &self.aabb_data {
            AabbData::OneAabb(aabb) => aabb.end(),
            AabbData::TwoAabbs { outer, inner, .. } => outer.end().min(&inner.end()),
        }
    }
}
