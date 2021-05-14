use thiserror::Error;

#[derive(Error, Debug)]
/// The different errors for loading resources.
pub enum ResourceError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_yaml::Error),
}

#[derive(Error, Debug, PartialEq)]
/// An error type for [`Shape`](crate::math:.shape::TextureGenerator).
pub enum ShapeError {
    #[error("Factor {0} is too big")]
    FactorTooBig(f32),
    #[error("Factor {0} is too small")]
    FactorTooSmall(f32),
    #[error("Radius {0} is too big")]
    RadiusTooBig(u32),
    #[error("Radius {0} is too small")]
    RadiusTooSmall(u32),
    #[error("Width {0} is too small")]
    WidthTooSmall(u32),
    #[error("Height {0} is too small")]
    HeightTooSmall(u32),
}
