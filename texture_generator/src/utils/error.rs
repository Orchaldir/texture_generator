use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerationError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Component '{component:?}' has an invalid shape")]
    InvalidShape {
        component: String,
        error: ShapeError,
    },
    #[error(transparent)]
    SerdeError(#[from] serde_yaml::Error),
    #[error("Value '{name:?}' of component '{component:?}' is too small ({value})")]
    ValueTooSmall {
        component: String,
        name: String,
        value: u32,
    },
}

impl GenerationError {
    pub fn invalid_shape<S: Into<String>>(component: S, error: ShapeError) -> GenerationError {
        GenerationError::InvalidShape {
            component: component.into(),
            error,
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
pub enum ShapeError {
    #[error("Radius {0} is too small")]
    RadiusTooSmall(u32),
    #[error("Width {0} is too small")]
    WidthTooSmall(u32),
    #[error("Height {0} is too small")]
    HeightTooSmall(u32),
}
