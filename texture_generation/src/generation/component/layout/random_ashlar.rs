use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::math::aabb::{AxisAlignedBoundingBox, AABB};
use crate::math::occupancy::tile::{check_column, check_row, fill_area, OccupancyTile, START};
use crate::math::point::Point;
use crate::math::size::Size;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

#[derive(Clone, Debug, PartialEq)]
pub struct RandomAshlarPattern {
    cells_per_side: u32,
    min_size: u32,
    max_size: u32,
    component: Component,
}

impl RandomAshlarPattern {
    pub fn new(
        cells_per_side: u32,
        min_size: u32,
        max_size: u32,
        component: Component,
    ) -> RandomAshlarPattern {
        RandomAshlarPattern {
            cells_per_side,
            min_size,
            max_size,
            component,
        }
    }

    /// Generates the pattern in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut Texture, outer: &AABB, inner: &AABB) {
        let mut creator = RandomAshlarCreator::new(
            self.cells_per_side,
            self.min_size,
            self.max_size,
            outer,
            inner,
        );

        for y in 0..self.cells_per_side {
            for x in 0..self.cells_per_side {
                creator.try_generate(data, &self.component, x, y);
            }
        }
    }
}

struct RandomAshlarCreator {
    cells_per_side: u32,
    occupancy_tile: OccupancyTile,
    rng: ThreadRng,
    size_distribution: Uniform<u32>,
    tile_size: Size,
    cell_size: Size,
    start: Point,
    limited: AABB,
    ashlar_index: usize,
}

impl RandomAshlarCreator {
    pub fn new(
        cells_per_side: u32,
        min_size: u32,
        max_size: u32,
        outer: &AABB,
        inner: &AABB,
    ) -> Self {
        Self {
            cells_per_side,
            occupancy_tile: OccupancyTile::new_active(cells_per_side as usize),
            rng: rand::thread_rng(),
            size_distribution: Uniform::from(min_size..(max_size + 1)),
            tile_size: Size::square(cells_per_side),
            cell_size: inner.size().divide(cells_per_side),
            start: inner.start(),
            limited: outer.limit(inner),
            ashlar_index: START,
        }
    }

    pub fn try_generate(&mut self, data: &mut Texture, component: &Component, x: u32, y: u32) {
        let cell_index = self.tile_size.convert_x_y(x, y);

        if self.occupancy_tile.is_free(cell_index) {
            let ashlar_size = self.grow_ashlar(x, y);

            fill_area(
                &mut self.occupancy_tile,
                self.tile_size,
                x,
                y,
                ashlar_size,
                self.ashlar_index,
            );
            self.ashlar_index += 1;

            let aabb = create_aabb(self.start, ashlar_size, self.cell_size, x, y);
            component.generate(data, &self.limited, &aabb);
        }
    }

    /// Grows the current ashlar from the empty starting cell until it reaches its random max size or hits an occupied cell.
    fn grow_ashlar(&mut self, x: u32, y: u32) -> Size {
        let max_size_x = self.calculate_max_size(x);
        let max_size_y = self.calculate_max_size(y);
        let mut size_x = 1;
        let mut size_y = 1;
        let mut is_x_ongoing = size_x < max_size_x;
        let mut is_y_ongoing = size_y < max_size_y;

        while is_x_ongoing || is_y_ongoing {
            if is_x_ongoing {
                is_x_ongoing =
                    if check_column(&self.occupancy_tile, self.tile_size, x + size_x, y, size_y) {
                        size_x += 1;
                        size_x < max_size_x
                    } else {
                        false
                    };
            }

            if is_y_ongoing {
                is_y_ongoing =
                    if check_row(&self.occupancy_tile, self.tile_size, x, y + size_y, size_x) {
                        size_y += 1;
                        size_y < max_size_y
                    } else {
                        false
                    };
            }
        }

        Size::new(size_x, size_y)
    }

    /// Returns the random max width or height of the current ashlar.
    fn calculate_max_size(&mut self, pos: u32) -> u32 {
        let remaining = self.cells_per_side - pos;
        self.size_distribution
            .sample(&mut self.rng)
            .min(remaining as u32)
    }
}

fn create_aabb(
    tile_start: Point,
    size_in_cells: Size,
    cell_size: Size,
    x: u32,
    y: u32,
) -> AxisAlignedBoundingBox {
    let point = Point::new(
        tile_start.x + (x * cell_size.width()) as i32,
        tile_start.y + (y * cell_size.height()) as i32,
    );
    let size = cell_size * size_in_cells;

    AABB::new(point, size)
}
