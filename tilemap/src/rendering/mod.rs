use crate::rendering::node::calculate_node_styles;
use crate::rendering::resource::Resources;
use crate::rendering::style::node::NodeStyle;
use crate::tilemap::border::{get_horizontal_borders_size, get_vertical_borders_size, Border};
use crate::tilemap::node::{
    get_end_of_horizontal_border, get_end_of_vertical_border, get_nodes_size,
    get_start_of_horizontal_border, get_start_of_vertical_border,
};
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::{AabbData, Data};
use texture_generation::math::aabb::AABB;
use texture_generation::math::color::BLACK;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

pub mod node;
pub mod resource;
pub mod style;

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

    /// Renders a [`Tilemap2d`].
    pub fn render(&self, tilemap: &Tilemap2d) -> Texture {
        let tile_size = Size::square(self.tile_size);
        let mut texture = Texture::for_tilemap(tilemap.get_size(), tile_size, BLACK);

        self.render_tiles(tilemap, tile_size, &mut texture);
        self.render_borders(tilemap, &mut texture);

        texture.apply(&self.resources.post_processes);

        texture
    }

    fn render_tiles(&self, tilemap: &Tilemap2d, tile_size: Size, texture: &mut Texture) {
        let tiles = tilemap.get_size();
        let mut start = Point::default();
        let mut index = 0;

        for _y in 0..tiles.height() {
            start.x = 0;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);
                let aabb = AABB::new(start, tile_size);
                let data = Data::with_global_id(index, aabb);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(id) => self.render_texture(texture, data, id, 1),
                    Tile::Solid(id) => self.render_texture(texture, data, id, self.wall_height),
                }

                start.x += tile_size.width() as i32;
                index += 1;
            }

            start.y += tile_size.height() as i32;
        }
    }

    fn render_borders(&self, tilemap: &Tilemap2d, mut texture: &mut Texture) {
        texture.set_base_depth(1);
        let nodes = calculate_node_styles(
            &self.resources.node_styles,
            &self.resources.wall_styles,
            tilemap,
        );
        self.render_horizontal_borders(tilemap, &nodes, &mut texture);
        self.render_vertical_borders(tilemap, &nodes, &mut texture);
        self.render_nodes(tilemap, &nodes, &mut texture);
    }

    fn render_horizontal_borders(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Option<&NodeStyle>],
        texture: &mut Texture,
    ) {
        let size = get_horizontal_borders_size(tilemap.get_size());
        let borders = tilemap.get_horizontal_borders();
        let mut start = Point::default();
        let step = self.tile_size as i32;
        let mut index = 0;
        let mut start_instance_id = 222;

        for y in 0..size.height() {
            start.x = 0;

            for _x in 0..size.width() {
                let data = Data::new(
                    index,
                    start_instance_id,
                    AabbData::OneAabb(texture.get_aabb()),
                );
                let border = borders[index];

                match border {
                    Border::Empty => {}
                    Border::Wall(id) => {
                        let wall_style = self.resources.wall_styles.get(id);

                        wall_style.get_edge_style().render_horizontal(
                            &data,
                            start,
                            self.calculate_horizontal_edge(nodes, index, y),
                            0,
                            texture,
                        );
                    }
                    Border::Door {
                        wall_id,
                        door_id,
                        is_front,
                    } => {
                        let wall_style = self.resources.wall_styles.get(wall_id);
                        let door_style = self.resources.door_styles.get(door_id);
                        let offset = door_style
                            .get_offset(wall_style.get_edge_style().get_thickness(), is_front);

                        door_style.get_edge_style().render_horizontal(
                            &data,
                            start,
                            self.calculate_horizontal_edge(nodes, index, y),
                            offset,
                            texture,
                        );
                    }
                    Border::Window { window_id, .. } => {
                        let window_style = self.resources.window_styles.get(window_id);

                        window_style.render_horizontal(
                            &data,
                            start,
                            self.calculate_horizontal_edge(nodes, index, y),
                            texture,
                        );
                    }
                }

                start.x += step;
                index += 1;
                start_instance_id += 1000;
            }

            start.y += step;
        }
    }

    fn render_vertical_borders(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Option<&NodeStyle>],
        texture: &mut Texture,
    ) {
        let size = get_vertical_borders_size(tilemap.get_size());
        let borders = tilemap.get_vertical_borders();
        let mut start = Point::default();
        let step = self.tile_size as i32;
        let mut index = 0;
        let mut start_instance_id = 111;

        for _y in 0..size.height() {
            start.x = 0;

            for _x in 0..size.width() {
                let data = Data::new(
                    index,
                    start_instance_id,
                    AabbData::OneAabb(texture.get_aabb()),
                );
                let border = borders[index];

                match border {
                    Border::Empty => {}
                    Border::Wall(id) => {
                        let wall_style = self.resources.wall_styles.get(id);
                        let start_index = get_start_of_vertical_border(index);
                        let end_index = get_end_of_vertical_border(size, index);

                        wall_style.get_edge_style().render_vertical(
                            &data,
                            start,
                            self.tile_size,
                            0,
                            nodes[start_index],
                            nodes[end_index],
                            texture,
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
                            &data,
                            start,
                            self.tile_size,
                            offset,
                            nodes[start_index],
                            nodes[end_index],
                            texture,
                        );
                    }
                    Border::Window { window_id, .. } => {
                        let window_style = self.resources.window_styles.get(window_id);
                        let start_index = get_start_of_vertical_border(index);
                        let end_index = get_end_of_vertical_border(size, index);

                        window_style.render_vertical(
                            &data,
                            start,
                            self.tile_size,
                            nodes[start_index],
                            nodes[end_index],
                            texture,
                        );
                    }
                }

                start.x += step;
                index += 1;
                start_instance_id += 1000;
            }

            start.y += step;
        }
    }

    fn render_nodes(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Option<&NodeStyle>],
        texture: &mut Texture,
    ) {
        let size = get_nodes_size(tilemap.get_size());
        let mut point = Point::default();
        let data = Data::for_texture(texture.get_aabb());
        let step = self.tile_size as i32;
        let mut index = 0;

        for _y in 0..size.height() {
            point.x = 0;

            for _x in 0..size.width() {
                if let Some(generator) = nodes[index] {
                    generator.render(&data, point, texture);
                }

                point.x += step;
                index += 1;
            }

            point.y += step;
        }
    }

    fn render_texture(&self, texture: &mut Texture, data: Data, generator_id: usize, height: u8) {
        let generator = self.resources.textures.get(generator_id);
        texture.set_base_depth(height);
        generator.render(texture, &data);
    }

    fn calculate_horizontal_edge(
        &self,
        nodes: &[Option<&NodeStyle>],
        border_index: usize,
        y: u32,
    ) -> (i32, u32) {
        let start_index = get_start_of_horizontal_border(border_index, y);
        let end_index = get_end_of_horizontal_border(border_index, y);
        let start_half = nodes[start_index].map(|n| n.get_half()).unwrap_or(0);
        let end_half = nodes[end_index].map(|n| n.get_half()).unwrap_or(0);
        (start_half, self.tile_size - (start_half + end_half) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::component::Component;
    use texture_generation::generation::TextureGenerator;
    use texture_generation::math::color::{Color, BLACK, BLUE, PINK, RED};
    use texture_generation::utils::resource::ResourceManager;

    #[test]
    fn test_render_tiles() {
        let mut resources = Resources::empty();
        let texture0 = create_texture("texture0", RED, 98);
        let texture1 = create_texture("texture0", BLUE, 41);
        resources.textures =
            ResourceManager::new(vec![texture0, texture1], TextureGenerator::default());
        let renderer = Renderer::new(2, 101, resources);
        let tiles = vec![
            Tile::Empty,
            Tile::Floor(0),
            Tile::Solid(1),
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
        let rendering = RenderingComponent::new_fill_area(color, depth);
        let component = Component::Rendering(Box::new(rendering));
        TextureGenerator::new(name, Size::default(), PINK, component)
    }
}
