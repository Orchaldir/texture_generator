use crate::generation::component::rendering::color::ColorSelector;
use crate::generation::data::Data;
use crate::generation::random::{Random, COLOR_INDEX};
use crate::math::color::Color;
use anyhow::{bail, Result};
use noise::{Perlin, Seedable};

#[derive(Clone, Debug, PartialEq)]
pub struct WoodRing {
    color: Color,
    color_variation: f32,
    ring_size: u32,
    ring_size_variation: u32,
}

impl WoodRing {
    pub fn new(
        color: Color,
        color_variation: f32,
        ring_size: u32,
        ring_size_variation: u32,
    ) -> Result<WoodRing> {
        if color_variation <= 0.0 {
            bail!("Argument 'color_variation' needs to be greater than 0");
        } else if color_variation > 0.5 {
            bail!("Argument 'color_variation' needs to be less than 0.5");
        }

        Ok(WoodRing {
            color,
            color_variation,
            ring_size,
            ring_size_variation,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColorFactory {
    /// Everything has the same color.
    ConstantColor(Color),
    /// A sequence of colors that repeats.
    Sequence(Vec<Color>),
    /// Randomly select a color from a list with equal probability.
    Random { random: Random, colors: Vec<Color> },
    /// Randomly select a color from a list based on probability.
    Probability {
        random: Random,
        colors: Vec<(usize, Color)>,
        max_number: usize,
    },
    /// Uses a noise function to interpolate between 2 colors.
    Noise {
        color0: Color,
        color1: Color,
        scale: f64,
    },
    /// Uses a noise function to interpolate between 2 colors randomly selected from a list.
    NoiseWithRandomColors {
        random: Random,
        colors: Vec<(usize, Color)>,
        max_number: usize,
        scale: f64,
    },
    WoodRings {
        early_wood: WoodRing,
        late_wood: WoodRing,
        noise_amplitude: f32,
        noise_scale: f64,
    },
}

impl ColorFactory {
    pub fn new_sequence(colors: Vec<Color>) -> Result<ColorFactory> {
        if colors.len() < 2 {
            bail!("ColorFactory::Sequence requires at least 2 colors");
        }

        Ok(ColorFactory::Sequence(colors))
    }

    pub fn new_random(colors: Vec<Color>) -> Result<ColorFactory> {
        if colors.len() < 2 {
            bail!("ColorFactory::Random requires at least 2 colors");
        }

        Ok(ColorFactory::Random {
            random: Random::Hash,
            colors,
        })
    }

    pub fn mock_random(numbers: Vec<u64>, colors: Vec<Color>) -> Result<ColorFactory> {
        if colors.len() < 2 {
            bail!("ColorFactory::Random requires at least 2 colors");
        }

        Ok(ColorFactory::Random {
            random: Random::Mock(numbers),
            colors,
        })
    }

    pub fn new_probability(random: Random, colors: Vec<(usize, Color)>) -> Result<ColorFactory> {
        let (threshold, converted_colors) = convert_probability("Probability", colors)?;

        Ok(ColorFactory::Probability {
            random,
            colors: converted_colors,
            max_number: threshold,
        })
    }

    pub fn new_noise(
        random: Random,
        colors: Vec<(usize, Color)>,
        scale: u32,
    ) -> Result<ColorFactory> {
        let (threshold, converted_colors) = convert_probability("Noise", colors)?;

        Ok(ColorFactory::NoiseWithRandomColors {
            random,
            colors: converted_colors,
            max_number: threshold,
            scale: scale as f64,
        })
    }

    /// Creates a ['ColorSelector'].
    pub fn create(&self, data: &Data) -> ColorSelector {
        match self {
            ColorFactory::ConstantColor(color) => ColorSelector::ConstantColor(*color),
            ColorFactory::Sequence(colors) => {
                let index = data.get_instance_id() % colors.len();
                ColorSelector::ConstantColor(colors[index])
            }
            ColorFactory::Random { random, colors } => {
                let index = random.get_random_instance_usize(data, colors.len(), COLOR_INDEX);
                ColorSelector::ConstantColor(colors[index])
            }
            ColorFactory::Probability {
                random,
                colors,
                max_number,
            } => {
                let index = random.get_random_instance_usize(data, *max_number, COLOR_INDEX);

                for (threshold, color) in colors {
                    if index < *threshold {
                        return ColorSelector::ConstantColor(*color);
                    }
                }

                ColorSelector::ConstantColor(colors[0].1)
            }
            ColorFactory::Noise {
                color0,
                color1,
                scale,
            } => {
                let noise = Perlin::new().set_seed(data.get_instance_id() as u32);
                ColorSelector::Noise {
                    color0: *color0,
                    color1: *color1,
                    noise: Box::new(noise),
                    scale: *scale,
                }
            }
            ColorFactory::NoiseWithRandomColors {
                random,
                colors,
                max_number,
                scale,
            } => {
                let random0 = random.get_random_instance_usize(data, *max_number, 0);
                let random1 = random.get_random_instance_usize(data, *max_number, 1);

                let index0 = get_color_index(colors, random0);
                let mut index1 = get_color_index(colors, random1);

                if index0 == index1 {
                    index1 = (index0 + 1) % colors.len();
                }

                let noise = Perlin::new().set_seed(data.get_instance_id() as u32);

                ColorSelector::Noise {
                    color0: colors[index0].1,
                    color1: colors[index1].1,
                    noise: Box::new(noise),
                    scale: *scale,
                }
            }
            ColorFactory::WoodRings {
                early_wood,
                late_wood,
                noise_amplitude,
                noise_scale,
            } => {
                let data1 = data.get_aabbs_in_texture_space();
                let aabb = data1.get_inner();
                let center = aabb.center();
                let noise = Perlin::new().set_seed(data.get_instance_id() as u32);
                let diff = aabb.end() - center;
                let ring_sizes =
                    calculate_ring_sizes(data, (diff.x + diff.y) as u32, early_wood, late_wood);

                ColorSelector::WoodRings {
                    center,
                    ring_sizes,
                    early_wood_color: early_wood.color,
                    late_wood_color: late_wood.color,
                    noise: Box::new(noise),
                    noise_amplitude: *noise_amplitude,
                    noise_scale: *noise_scale,
                }
            }
        }
    }
}

fn calculate_ring_sizes(
    data: &Data,
    max_distance: u32,
    early_wood: &WoodRing,
    late_wood: &WoodRing,
) -> Vec<(f32, f32)> {
    let random = Random::Hash;
    let mut ring_sizes = Vec::new();

    let mut i = 0;
    let mut distance = 0;
    let mut is_early = true;

    while distance < max_distance {
        let definition = if is_early { early_wood } else { late_wood };
        let color_variation = random.get_random_instance_f32(data, definition.color_variation, i);
        let mut ring_size = definition.ring_size;

        if definition.ring_size_variation > 0 {
            ring_size += random.get_random_instance_u32(data, definition.ring_size_variation, i);
        }

        distance += ring_size;
        ring_sizes.push((color_variation, ring_size as f32));
        is_early = !is_early;
        i += 1;
    }

    ring_sizes
}

fn convert_probability(
    parent: &str,
    colors: Vec<(usize, Color)>,
) -> Result<(usize, Vec<(usize, Color)>)> {
    if colors.len() < 2 {
        bail!("ColorFactory::{} requires at least 2 colors", parent);
    }

    let mut converted_colors = Vec::with_capacity(colors.len());
    let mut threshold = 0;

    for (i, (probability, color)) in colors.into_iter().enumerate() {
        if probability == 0 {
            bail!("{}.probability of ColorFactory::{} is 0", i + 1, parent);
        }

        threshold += probability;
        converted_colors.push((threshold, color));
    }

    Ok((threshold, converted_colors))
}

fn get_color_index(colors: &[(usize, Color)], index: usize) -> usize {
    for (i, (threshold, _color)) in colors.iter().enumerate() {
        if index < *threshold {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::{BLUE, GREEN, RED};
    use Random::Hash;

    #[test]
    #[should_panic]
    fn test_new_sequence_too_few_colors() {
        ColorFactory::new_sequence(vec![RED]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_random_too_few_colors() {
        ColorFactory::new_random(vec![RED]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_probability_too_colors() {
        ColorFactory::new_probability(Hash, vec![(100, RED)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_probability_is_zero() {
        let colors = vec![(1, RED), (0, GREEN), (3, BLUE)];
        ColorFactory::new_probability(Hash, colors).unwrap();
    }

    #[test]
    fn test_constant() {
        let factory = ColorFactory::ConstantColor(RED);

        assert_cost(factory.create(&Data::only_instance_id(0)), RED);
        assert_cost(factory.create(&Data::only_instance_id(1)), RED);
        assert_cost(factory.create(&Data::only_instance_id(2)), RED);
    }

    #[test]
    fn test_sequence() {
        let factory = ColorFactory::new_sequence(vec![RED, GREEN, BLUE]).unwrap();

        assert_cost(factory.create(&Data::only_instance_id(0)), RED);
        assert_cost(factory.create(&Data::only_instance_id(1)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(2)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(3)), RED);
        assert_cost(factory.create(&Data::only_instance_id(4)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(5)), BLUE);
    }

    #[test]
    fn test_random() {
        let numbers = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let colors = vec![RED, GREEN, BLUE];
        let factory = ColorFactory::mock_random(numbers, colors).unwrap();

        assert_cost(factory.create(&Data::only_instance_id(0)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(1)), RED);
        assert_cost(factory.create(&Data::only_instance_id(2)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(3)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(4)), RED);
    }

    #[test]
    fn test_probability() {
        let random = Random::Mock(vec![3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let colors = vec![(1, RED), (2, GREEN), (3, BLUE)];
        let factory = ColorFactory::new_probability(random, colors).unwrap();

        assert_cost(factory.create(&Data::only_instance_id(0)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(1)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(2)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(3)), RED);
        assert_cost(factory.create(&Data::only_instance_id(4)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(5)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(6)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(7)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(8)), BLUE);
    }

    fn assert_cost(factory: ColorSelector, color: Color) {
        assert_eq!(factory, ColorSelector::ConstantColor(color));
    }
}
