use crate::generation::data::texture::Texture;
use crate::math::size::Size;
use crate::math::vector3::Vector3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Lighting {
    light_direction: Vector3,
    normal_z: f32,
    shininess: i32,
}

impl Lighting {
    pub fn new(mut light_direction: Vector3, normal_z: u32, shininess: u32) -> Lighting {
        light_direction.normalize();
        Lighting {
            light_direction,
            normal_z: normal_z as f32,
            shininess: shininess as i32,
        }
    }

    pub fn process(&self, data: &mut Texture) {
        info!("Post Processing: Lighting");

        let size = *data.get_size();
        let mut index = 0;
        let ambient = 0.1;

        let view_direction = Vector3::new(0.0, 0.0, 1.0);
        let half_direction = (view_direction + self.light_direction).get_normalized();

        for y in 0..size.height() {
            for x in 0..size.width() {
                let normal = calculate_normal_from_heightmap(
                    &size,
                    data.get_depth_data(),
                    x,
                    y,
                    self.normal_z,
                );

                let diffuse = self.light_direction.dot(&normal).max(0.0);
                let specular = half_direction.dot(&normal).max(0.0).powi(self.shininess);

                data.get_color_data_mut()[index] *= ambient + diffuse + specular;

                index += 1;
            }
        }
    }
}

pub fn calculate_normal_from_heightmap(
    size: &Size,
    depth: &[u8],
    x: u32,
    y: u32,
    normal_z: f32,
) -> Vector3 {
    let index_left = size.convert_x_y(if x > 0 { x - 1 } else { x }, y);
    let index_right = size.convert_x_y(if x < size.width() - 1 { x + 1 } else { x }, y);
    let index_down = size.convert_x_y(x, if y > 0 { y - 1 } else { y });
    let index_up = size.convert_x_y(x, if y < size.height() - 1 { y + 1 } else { y });

    calculate_normal(
        depth[index_left],
        depth[index_right],
        depth[index_down],
        depth[index_up],
        normal_z,
    )
}

pub fn calculate_normal(
    depth_left: u8,
    depth_right: u8,
    depth_down: u8,
    depth_up: u8,
    normal_z: f32,
) -> Vector3 {
    let diff_x = depth_right as f32 - depth_left as f32;
    let diff_y = depth_up as f32 - depth_down as f32;
    Vector3::new(diff_x, diff_y, normal_z).get_normalized()
}
