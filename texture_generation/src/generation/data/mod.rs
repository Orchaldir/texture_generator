use crate::generation::data::aabb::AabbData;
use crate::math::aabb::AABB;
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
        Self::new(0, 0, AabbData::from_one_aabb(aabb))
    }

    pub fn with_global_id(global_id: usize, aabb: AABB) -> Self {
        Self::new(global_id, 0, AabbData::from_one_aabb(aabb))
    }

    pub fn for_two_aabb(global_id: usize, outer: AABB, inner: AABB) -> Self {
        Self::new(global_id, 0, AabbData::from_two_aabb(outer, inner))
    }

    pub fn only_instance_id(instance_id: usize) -> Self {
        Self::new(0, instance_id, AabbData::from_one_aabb(AABB::default()))
    }

    pub fn new(global_id: usize, instance_id: usize, aabb_data: AabbData) -> Self {
        Self::full(global_id, instance_id, aabb_data, Side::Right, true)
    }

    pub fn with_orientation(
        global_id: usize,
        instance_id: usize,
        aabb_data: AabbData,
        orientation: Side,
    ) -> Self {
        info!("with_orientation(): aabb {:?}", aabb_data);
        let aabbs = match orientation {
            Side::Top => aabb_data.rotate_origin_revers(),
            Side::Left => aabb_data.rotate_origin().rotate_origin(),
            Side::Bottom => aabb_data.rotate_origin(),
            Side::Right => aabb_data,
        };
        info!("with_orientation(): rotated {:?}", aabbs);
        Self {
            global_id,
            instance_id,
            aabb_data: aabbs,
            orientation,
            is_horizontal: true,
        }
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
        Self {
            aabb_data: self.aabb_data.next(inner),
            ..*self
        }
    }

    /// Replace the inner [`AABB`] for the next instance of the same [`Component`]. Increases `instance_id`.
    pub fn next(&mut self, inner: AABB) -> Self {
        let old_id = self.instance_id;
        self.instance_id += 1;

        Self {
            instance_id: old_id,
            aabb_data: self.aabb_data.next(inner),
            ..*self
        }
    }

    /// Replaces the inner [`AABB`] for the next instance of the same [`Component`]. Overwrites `instance_id`.
    pub fn set(&self, instance_id: usize, inner: AABB) -> Self {
        Self {
            instance_id,
            aabb_data: self.aabb_data.next(inner),
            ..*self
        }
    }

    /// Combines the inner & outer [`AABB`]s into the inner one,
    pub fn combine(&self) -> Self {
        Self {
            aabb_data: self.aabb_data.combine(),
            ..*self
        }
    }

    pub fn make_horizontal(&self) -> Self {
        if self.is_horizontal {
            Self {
                aabb_data: self.aabb_data.clone(),
                ..*self
            }
        } else {
            info!("make_horizontal(): rotate reverse");
            Self {
                aabb_data: self.aabb_data.rotate_origin_revers(),
                is_horizontal: true,
                ..*self
            }
        }
    }

    pub fn make_vertical(&self) -> Self {
        if !self.is_horizontal {
            Self {
                aabb_data: self.aabb_data.clone(),
                ..*self
            }
        } else {
            info!("make_vertical(): rotate reverse");
            Self {
                aabb_data: self.aabb_data.rotate_origin(),
                is_horizontal: false,
                ..*self
            }
        }
    }

    pub fn get_global_id(&self) -> usize {
        self.global_id
    }

    pub fn get_instance_id(&self) -> usize {
        self.instance_id
    }

    pub fn get_aabbs(&self) -> &AabbData {
        &self.aabb_data
    }

    pub fn get_aabbs_in_texture(&self) -> AabbData {
        info!("get_aabbs_in_texture(): {:?}", self.aabb_data);
        let aabbs = match self.orientation {
            Side::Top => self.aabb_data.rotate_origin(),
            Side::Left => self.aabb_data.rotate_origin().rotate_origin(),
            Side::Bottom => self.aabb_data.rotate_origin_revers(),
            Side::Right => self.aabb_data.clone(),
        };
        info!("get_aabbs_in_texture(): aabbs {:?}", aabbs);

        if self.is_horizontal {
            aabbs
        } else {
            info!("get_aabbs_in_texture(): rotate");
            let r = aabbs.rotate_origin_revers();

            info!("get_aabbs_in_texture(): rotate: {:?}", r);

            r
        }
    }
}
