use crate::rendering::node::calculate_node_styles;
use crate::rendering::style::door::DoorStyle;
use crate::rendering::style::wall::{NodeStyle, WallStyle};
use crate::rendering::style::window::WindowStyle;
use crate::tilemap::border::{get_horizontal_borders_size, get_vertical_borders_size, Border};
use crate::tilemap::node::{
    get_end_of_horizontal_border, get_end_of_vertical_border, get_nodes_size,
    get_start_of_horizontal_border, get_start_of_vertical_border,
};
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;
use crate::tilemap::Side;
use crate::tilemap::Side::*;
use texture_generation::generation::data::RuntimeData;
use texture_generation::generation::process::PostProcess;
use texture_generation::generation::TextureGenerator;
use texture_generation::math::aabb::AABB;
use texture_generation::math::color::BLACK;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;
use texture_generation::utils::resource::ResourceManager;

pub mod node;
pub mod style;

pub struct Resources {
    pub textures: ResourceManager<TextureGenerator>,
    pub wall_styles: ResourceManager<WallStyle<NodeStyle>>,
    pub door_styles: ResourceManager<DoorStyle>,
    pub window_styles: ResourceManager<WindowStyle>,
    pub post_processes: Vec<PostProcess>,
}

impl Resources {
    pub fn empty() -> Resources {
        Self::new(
            ResourceManager::default(),
            ResourceManager::default(),
            ResourceManager::default(),
            ResourceManager::default(),
            Vec::default(),
        )
    }

    pub fn new(
        textures: ResourceManager<TextureGenerator>,
        wall_styles: ResourceManager<WallStyle<NodeStyle>>,
        door_styles: ResourceManager<DoorStyle>,
        window_styles: ResourceManager<WindowStyle>,
        post_processes: Vec<PostProcess>,
    ) -> Self {
        Resources {
            textures,
            wall_styles,
            door_styles,
            window_styles,
            post_processes,
        }
    }
}

/// Renders a [`Tilemap2d`] in a specific style.
pub struct Renderer {
    tile_size: u32,
    wall_height: u8,
    resources: Resources,
}

impl Renderer {
    pub fn new(tile_size: u32, wall_height: u8, resources: Resources) -> Self {
        Renderer {
            tile_size,
            wall_height,
            resources,
        }
    }

    pub fn get_tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn get_tile_index(&self, tilemap: &Tilemap2d, x: u32, y: u32) -> usize {
        let tile_x = x / self.tile_size;
        let tile_y = y / self.tile_size;
        tilemap.get_size().convert_x_y(tile_x, tile_y)
    }

    pub fn get_side(&self, tilemap: &Tilemap2d, x: u32, y: u32, tile_index: usize) -> Option<Side> {
        let tile_size = self.tile_size;
        let start = tilemap.get_size().to_point(tile_index);
        let x = (x - start.x as u32 * tile_size) as f32 / tile_size as f32;
        let y = (y - start.y as u32 * tile_size) as f32 / tile_size as f32;
        let border = 0.1;
        let is_top = y < border;
        let is_left = x < border;
        let is_bottom = y > (1.0 - border);
        let is_right = x > (1.0 - border);

        Some(if is_top && !is_left && !is_right {
            Top
        } else if is_left && !is_top && !is_bottom {
            Left
        } else if is_bottom && !is_left && !is_right {
            Bottom
        } else if is_right && !is_top && !is_bottom {
            Right
        } else {
            return None;
        })
    }

    /// Renders a [`Tilemap2d`].
    pub fn render(&self, tilemap: &Tilemap2d) -> RuntimeData {
        let tile_size = Size::square(self.tile_size);
        let size = tile_size * tilemap.get_size();
        let mut data = RuntimeData::new(size, BLACK);

        self.render_tiles(tilemap, tile_size, &mut data);
        self.render_borders(tilemap, &mut data);
        self.post_process(&mut data);

        data
    }

    fn render_tiles(&self, tilemap: &Tilemap2d, tile_size: Size, data: &mut RuntimeData) {
        let tiles = tilemap.get_size();
        let mut start = Point::default();
        let mut index = 0;

        for _y in 0..tiles.height() {
            start.x = 0;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);
                let aabb = AABB::new(start, tile_size);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(id) => self.render_texture(id, 0, data, &aabb),
                    Tile::Full(id) => self.render_texture(id, self.wall_height, data, &aabb),
                }

                start.x += tile_size.width() as i32;
                index += 1;
            }

            start.y += tile_size.height() as i32;
        }
    }

    fn render_borders(&self, tilemap: &Tilemap2d, mut data: &mut RuntimeData) {
        data.set_base_depth(self.wall_height);
        let nodes = calculate_node_styles(&self.resources.wall_styles, tilemap);
        self.render_horizontal_borders(tilemap, &nodes, &mut data);
        self.render_vertical_borders(tilemap, &nodes, &mut data);
        self.render_nodes(tilemap, &nodes, &mut data);
    }

    fn render_horizontal_borders(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Option<&NodeStyle>],
        data: &mut RuntimeData,
    ) {
        let size = get_horizontal_borders_size(tilemap.get_size());
        let borders = tilemap.get_horizontal_borders();
        let mut start = Point::default();
        let aabb = data.get_aabb();
        let step = self.tile_size as i32;
        let mut index = 0;

        for y in 0..size.height() {
            start.x = 0;

            for _x in 0..size.width() {
                let border = borders[index];

                match border {
                    Border::Empty => {}
                    Border::Wall(id) => {
                        let wall_style = self.resources.wall_styles.get(id);
                        let start_index = get_start_of_horizontal_border(index, y);
                        let end_index = get_end_of_horizontal_border(index, y);

                        wall_style.get_edge_style().render_horizontal(
                            &aabb,
                            start,
                            self.tile_size,
                            0,
                            nodes[start_index],
                            nodes[end_index],
                            data,
                        );
                    }
                    Border::Door {
                        wall_id,
                        door_id,
                        is_front,
                    } => {
                        let wall_style = self.resources.wall_styles.get(wall_id);
                        let door_style = self.resources.door_styles.get(door_id);
                        let start_index = get_start_of_horizontal_border(index, y);
                        let end_index = get_end_of_horizontal_border(index, y);
                        let offset = door_style
                            .get_offset(wall_style.get_edge_style().get_thickness(), is_front);

                        door_style.get_edge_style().render_horizontal(
                            &aabb,
                            start,
                            self.tile_size,
                            offset,
                            nodes[start_index],
                            nodes[end_index],
                            data,
                        );
                    }
                    Border::Window { window_id, .. } => {
                        let window_style = self.resources.window_styles.get(window_id);
                        let start_index = get_start_of_horizontal_border(index, y);
                        let end_index = get_end_of_horizontal_border(index, y);

                        window_style.render_horizontal(
                            &aabb,
                            start,
                            self.tile_size,
                            nodes[start_index],
                            nodes[end_index],
                            data,
                        );
                    }
                }

                start.x += step;
                index += 1;
            }

            start.y += step;
        }
    }

    fn render_vertical_borders(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Option<&NodeStyle>],
        data: &mut RuntimeData,
    ) {
        let size = get_vertical_borders_size(tilemap.get_size());
        let borders = tilemap.get_vertical_borders();
        let mut start = Point::default();
        let aabb = data.get_aabb();
        let step = self.tile_size as i32;
        let mut index = 0;

        for _y in 0..size.height() {
            start.x = 0;

            for _x in 0..size.width() {
                let border = borders[index];

                match border {
                    Border::Empty => {}
                    Border::Wall(id) => {
                        let wall_style = self.resources.wall_styles.get(id);
                        let start_index = get_start_of_vertical_border(index);
                        let end_index = get_end_of_vertical_border(size, index);

                        wall_style.get_edge_style().render_vertical(
                            &aabb,
                            start,
                            self.tile_size,
                            0,
                            nodes[start_index],
                            nodes[end_index],
                            data,
                        );
                    }
                    Border::Door {
                        wall_id,
                        door_id,
                        is_front,
                    } => {
                        let wall_style = self.resources.wall_styles.get(wall_id);
                        let door_style = self.resources.door_styles.get(door_id);
                        let start_index = get_start_of_vertical_border(index);
                        let end_index = get_end_of_vertical_border(size, index);
                        let offset = door_style
                            .get_offset(wall_style.get_edge_style().get_thickness(), is_front);

                        door_style.get_edge_style().render_vertical(
                            &aabb,
                            start,
                            self.tile_size,
                            offset,
                            nodes[start_index],
                            nodes[end_index],
                            data,
                        );
                    }
                    Border::Window { window_id, .. } => {
                        let window_style = self.resources.window_styles.get(window_id);
                        let start_index = get_start_of_vertical_border(index);
                        let end_index = get_end_of_vertical_border(size, index);

                        window_style.render_vertical(
                            &aabb,
                            start,
                            self.tile_size,
                            nodes[start_index],
                            nodes[end_index],
                            data,
                        );
                    }
                }

                start.x += step;
                index += 1;
            }

            start.y += step;
        }
    }

    fn render_nodes(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Option<&NodeStyle>],
        data: &mut RuntimeData,
    ) {
        let size = get_nodes_size(tilemap.get_size());
        let mut point = Point::default();
        let aabb = data.get_aabb();
        let step = self.tile_size as i32;
        let mut index = 0;

        for _y in 0..size.height() {
            point.x = 0;

            for _x in 0..size.width() {
                if let Some(generator) = nodes[index] {
                    generator.render(&aabb, point, data);
                }

                point.x += step;
                index += 1;
            }

            point.y += step;
        }
    }

    fn post_process(&self, data: &mut RuntimeData) {
        for post_process in self.resources.post_processes.iter() {
            post_process.process(data);
        }
    }

    fn render_texture(&self, texture_id: usize, height: u8, data: &mut RuntimeData, aabb: &AABB) {
        let texture = self.resources.textures.get(texture_id);
        data.set_base_depth(height);
        texture.render(data, aabb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::component::Component;
    use texture_generation::generation::data::Data;
    use texture_generation::generation::TextureGenerator;
    use texture_generation::math::color::{Color, BLACK, BLUE, PINK, RED};

    #[test]
    fn test_get_tile_index() {
        let renderer = Renderer::new(100, 100, Resources::empty());
        let tilemap = Tilemap2d::default(Size::new(2, 3), Tile::Empty);

        assert_eq!(renderer.get_tile_index(&tilemap, 50, 50), 0);
        assert_eq!(renderer.get_tile_index(&tilemap, 150, 50), 1);
        assert_eq!(renderer.get_tile_index(&tilemap, 50, 150), 2);
        assert_eq!(renderer.get_tile_index(&tilemap, 150, 150), 3);
        assert_eq!(renderer.get_tile_index(&tilemap, 50, 250), 4);
        assert_eq!(renderer.get_tile_index(&tilemap, 150, 250), 5);
    }

    #[test]
    fn test_get_side() {
        let renderer = Renderer::new(100, 100, Resources::empty());
        let tilemap = Tilemap2d::default(Size::new(2, 3), Tile::Empty);

        assert_eq!(renderer.get_side(&tilemap, 50, 150, 2), None);
        assert_eq!(renderer.get_side(&tilemap, 50, 105, 2), Some(Top));
        assert_eq!(renderer.get_side(&tilemap, 5, 150, 2), Some(Left));
        assert_eq!(renderer.get_side(&tilemap, 50, 195, 2), Some(Bottom));
        assert_eq!(renderer.get_side(&tilemap, 95, 150, 2), Some(Right));
    }

    #[test]
    fn test_render_tiles() {
        let mut resources = Resources::empty();
        let texture0 = create_texture("texture0", RED, 99);
        let texture1 = create_texture("texture0", BLUE, 42);
        resources.textures =
            ResourceManager::new(vec![texture0, texture1], TextureGenerator::default());
        let renderer = Renderer::new(2, 100, resources);
        let tiles = vec![
            Tile::Empty,
            Tile::Floor(0),
            Tile::Full(1),
            Tile::Empty,
            Tile::Empty,
            Tile::Floor(1),
        ];
        let tilemap = Tilemap2d::new(Size::new(2, 3), tiles).unwrap();

        let data = renderer.render(&tilemap);

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK,   RED,   RED,
            BLACK, BLACK,   RED,   RED,
             BLUE,  BLUE, BLACK, BLACK,
             BLUE,  BLUE, BLACK, BLACK,
            BLACK, BLACK,  BLUE,  BLUE,
            BLACK, BLACK,  BLUE,  BLUE,
        ];

        assert_eq!(data.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
              0,   0, 99, 99,
              0,   0, 99, 99,
            142, 142,  0,  0,
            142, 142,  0,  0,
              0,   0, 42, 42,
              0,   0, 42, 42,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }

    fn create_texture(name: &str, color: Color, depth: u8) -> TextureGenerator {
        let rendering = RenderingComponent::new_fill_area(name, color, depth);
        let component = Component::Rendering(Box::new(rendering));
        TextureGenerator::new(name, Size::default(), PINK, component)
    }
}
