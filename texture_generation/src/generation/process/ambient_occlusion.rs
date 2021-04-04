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
        let mut index = 0;

        for y in 0..size.height() {
            let start_y = y.saturating_sub(self.radius);
            let end_y = (y + self.radius).min(size.height());

            info!("Line {}/{}: {}-{}", y + 1, size.height(), start_y, end_y);

            for x in 0..size.width() {
                let start_x = x.saturating_sub(self.radius);
                let end_x = (x + self.radius).min(size.width());

                let average = self.calculate_average_depth(data, start_x, start_y, end_x, end_y);
                let current = data.get_depth_data()[index];
                let diff = (current as f32 - average as f32).min(0.0);
                let factor = diff.max(self.max_penalty) / self.max_penalty;
                let penalty = factor * self.max_penalty;

                data.get_color_data_mut()[index] *= penalty;

                index += 1;
            }
        }
    }

    fn calculate_average_depth(
        &self,
        data: &dyn Data,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    ) -> u8 {
        let mut sum = 0u32;
        let mut pixels = 0u32;
        let depth_data = data.get_depth_data();

        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = data.get_size().convert_x_y(x, y);
                sum += depth_data[index] as u32;
                pixels += 1;
            }
        }

        (sum / pixels) as u8
    }
}
