use thiserror::Error;

#[derive(Error, Debug)]
/// An error type for [`TextureGenerator`] and its components.
pub enum GenerationError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Color {name:?} has an invalid value {value:?}")]
    InvalidColor { name: String, value: String },
    #[error("Component {component:?} has an invalid shape")]
    InvalidShape {
        component: String,
        source: ShapeError,
    },
    #[error(transparent)]
    SerdeError(#[from] serde_yaml::Error),
    #[error("Value {name:?} of component {component:?} is too big ({value})")]
    ValueTooBig {
        component: String,
        name: String,
        value: u32,
    },
    #[error("Value {name:?} of component {component:?} is too small ({value})")]
    ValueTooSmall {
        component: String,
        name: String,
        value: u32,
    },
}

impl GenerationError {
    pub fn invalid_colo<S: Into<String>>(name: S, value: S) -> GenerationError {
        GenerationError::InvalidColor {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn invalid_shape<S: Into<String>>(component: S, source: ShapeError) -> GenerationError {
        GenerationError::InvalidShape {
            component: component.into(),
            source,
        }
    }

    pub fn value_too_big<S: Into<String>, T: Into<String>>(
        component: S,
        name: T,
        value: u32,
    ) -> GenerationError {
        GenerationError::ValueTooBig {
            component: component.into(),
            name: name.into(),
            value,
        }
    }

    pub fn value_too_small<S: Into<String>, T: Into<String>>(
        component: S,
        name: T,
        value: u32,
    ) -> GenerationError {
        GenerationError::ValueTooSmall {
            component: component.into(),
            name: name.into(),
            value,
        }
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
/// An error type for [`Shape`].
pub enum ShapeError {
    #[error("Radius {0} is too small")]
    RadiusTooSmall(u32),
    #[error("Width {0} is too small")]
    WidthTooSmall(u32),
    #[error("Height {0} is too small")]
    HeightTooSmall(u32),
}
