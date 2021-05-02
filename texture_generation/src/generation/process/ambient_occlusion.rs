use crate::generation::data::texture::Texture;
use crate::math::size::Size;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmbientOcclusion {
    step: u8,
    max_diff: f32,
    max_penalty: f32,
}

impl AmbientOcclusion {
    pub fn new(step: u8, max_diff: f32, max_penalty: f32) -> AmbientOcclusion {
        AmbientOcclusion {
            step,
            max_diff,
            max_penalty,
        }
    }

    pub fn process(&self, data: &mut Texture) {
        info!("Post Processing: Ambient Occlusion");

        let size = *data.get_size();
        let mut depth = data.get_depth_data().to_owned();

        info!("Start blurring");

        blur_right_down(size, &mut depth, self.step);
        blur_left_up(size, &mut depth, self.step);

        for y in 0..size.height() {
            info!("Line {}/{}", y + 1, size.height());

            let mut index = size.convert_x_y(0, y);

            for _x in 0..size.width() {
                let current = data.get_depth_data()[index];
                let blurred = depth[index];

                if blurred == 0 {
                    index += 1;
                    continue;
                }

                let diff = (current as f32 - blurred as f32).min(0.0);
                let factor = diff.max(self.max_diff) / self.max_diff;
                let penalty = factor * self.max_penalty;

                //info!("average={} current={} diff={} factor={} penalty={}", average, current, diff, factor, penalty);

                data.get_color_data_mut()[index] *= 1.0 + penalty;

                index += 1;
            }
        }
    }
}

fn blur_right_down(size: Size, depth: &mut Vec<u8>, step: u8) {
    let width = size.width();

    for y in 1..size.height() {
        let mut index = size.convert_x_y(1, y);
        let mut last_depth = depth[index - 1];

        for _x in 1..width {
            let last_line = depth[index - width as usize];
            let previous = last_depth.max(last_line).saturating_sub(step);
            let current = depth[index];
            let new = previous.max(current);

            depth[index] = new;
            last_depth = new;
            index += 1;
        }
    }
}

fn blur_left_up(size: Size, depth: &mut Vec<u8>, step: u8) {
    let width = size.width();
    let start_x = width - 2;
    let start_y = size.height() - 2;

    for y in (0..start_y).rev() {
        let mut index = size.convert_x_y(start_x, y);
        let mut last_depth = depth[index + 1];

        for _x in (0..start_x).rev() {
            let last_line = depth[index + width as usize];
            let previous = last_depth.max(last_line).saturating_sub(step);
            let current = depth[index];
            let new = previous.max(current);

            depth[index] = new;
            last_depth = new;
            index -= 1;
        }
    }
}
