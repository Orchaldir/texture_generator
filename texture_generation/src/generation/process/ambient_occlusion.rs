use crate::generation::data::Data;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmbientOcclusion {
    radius: u32,
    max_diff: f32,
    max_penalty: f32,
}

impl AmbientOcclusion {
    pub fn new(radius: u32, max_diff: f32, max_penalty: f32) -> AmbientOcclusion {
        AmbientOcclusion {
            radius,
            max_diff,
            max_penalty,
        }
    }

    pub fn process(&self, data: &mut dyn Data) {
        info!("Post Processing: Ambient Occlusion");

        let size = *data.get_size();
        let max_x = size.width().saturating_sub(self.radius);
        let max_y = size.height().saturating_sub(self.radius);

        for y in self.radius..max_y {
            info!("Line {}/{}", y + 1, size.height());

            let mut index = size.convert_x_y(self.radius, y);

            for x in self.radius..max_x {
                let average = self.calculate_average_depth(data, x, y);
                let current = data.get_depth_data()[index];
                let diff = (current as f32 - average as f32).min(0.0);
                let factor = diff.max(self.max_diff) / self.max_diff;
                let penalty = factor * self.max_penalty;

                //info!("average={} current={} diff={} factor={} penalty={}", average, current, diff, factor, penalty);

                data.get_color_data_mut()[index] *= 1.0 + penalty;

                index += 1;
            }
        }
    }

    fn calculate_average_depth(&self, data: &dyn Data, x: u32, y: u32) -> u8 {
        let mut sum = 0u32;
        let mut pixels = 0u32;
        let depth_data = data.get_depth_data();
        let size = data.get_size();

        let start_x = x - self.radius;
        let start_y = y - self.radius;
        let end_y = y + self.radius - 1;
        let side = self.radius * 2;

        for i in 0..side {
            let index_x = size.convert_x_y(start_x + i, y);
            let index_y = size.convert_x_y(x, start_y + i);
            let index_d0 = size.convert_x_y(start_x + i, start_y + i);
            let index_d1 = size.convert_x_y(start_x + i, end_y - i);

            sum += depth_data[index_x] as u32;
            sum += depth_data[index_y] as u32;
            sum += depth_data[index_d0] as u32;
            sum += depth_data[index_d1] as u32;

            pixels += 4;
        }

        (sum / pixels) as u8
    }
}
