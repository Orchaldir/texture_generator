use crate::generation::component::Component;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::occupancy::tile::{check_column, check_row, fill_area, OccupancyTile, START};
use crate::math::point::Point;
use crate::math::size::Size;
use rand::distributions::{Distribution, Uniform};

#[derive(Clone, Debug, PartialEq)]
pub struct RandomAshlarPattern {
    cells_per_side: u32,
    max_size: u32,
    component: Component,
}

impl RandomAshlarPattern {
    pub fn new(cells_per_side: u32, max_size: u32, component: Component) -> RandomAshlarPattern {
        RandomAshlarPattern {
            cells_per_side,
            max_size,
            component,
        }
    }

    /// Generates the pattern in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, outer: &AABB, inner: &AABB) {
        let mut rng = rand::thread_rng();
        let size_distribution = Uniform::from(1..(self.max_size + 1));
        let mut occupancy_tile = OccupancyTile::new_active(self.cells_per_side as usize);
        let mut cell_index = 0;
        let mut area_index = START;
        let tile_size = Size::square(self.cells_per_side as u32);
        let cell_size = inner.size().divide(self.cells_per_side as u32);
        let start = inner.start();
        let limited = outer.limit(inner);

        for y in 0..self.cells_per_side {
            for x in 0..self.cells_per_side {
                if occupancy_tile.is_free(cell_index) {
                    let remaining_x = self.cells_per_side - x;
                    let remaining_y = self.cells_per_side - y;
                    let max_size_x = size_distribution.sample(&mut rng).min(remaining_x as u32);
                    let max_size_y = size_distribution.sample(&mut rng).min(remaining_y as u32);
                    let mut size_x = 1;
                    let mut size_y = 1;
                    let mut is_x_ongoing = size_x < max_size_x;
                    let mut is_y_ongoing = size_y < max_size_y;

                    while is_x_ongoing || is_y_ongoing {
                        if is_x_ongoing {
                            is_x_ongoing = if check_column(
                                &occupancy_tile,
                                tile_size,
                                x + size_x,
                                y,
                                size_y,
                            ) {
                                size_x += 1;
                                size_x < max_size_x
                            } else {
                                false
                            };
                        }

                        if is_y_ongoing {
                            is_y_ongoing =
                                if check_row(&occupancy_tile, tile_size, x, y + size_y, size_x) {
                                    size_y += 1;
                                    size_y < max_size_y
                                } else {
                                    false
                                };
                        }
                    }

                    fill_area(
                        &mut occupancy_tile,
                        tile_size,
                        x,
                        y,
                        size_x,
                        size_y,
                        area_index,
                    );
                    area_index += 1;

                    let point = Point::new(
                        start.x + (x * cell_size.width()) as i32,
                        start.y + (y * cell_size.width()) as i32,
                    );
                    let size = Size::new(cell_size.width() * size_x, cell_size.height() * size_y);

                    self.component
                        .generate(data, &limited, &AABB::new(point, size));
                }

                cell_index += 1;
            }
        }
    }
}
