use thiserror::Error;

#[derive(Error, Debug)]
/// The different errors for reading & converting definitions.
pub enum DefinitionError {
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
    #[error(transparent)]
    ValueError(#[from] ValueError),
}

#[derive(Error, Debug)]
/// The different errors for creating new objects.
pub enum ValueError {
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

impl DefinitionError {
    pub fn invalid_color<S: Into<String>>(name: S, value: S) -> DefinitionError {
        DefinitionError::InvalidColor {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn invalid_shape<S: Into<String>>(component: S, source: ShapeError) -> DefinitionError {
        DefinitionError::InvalidShape {
            component: component.into(),
            source,
        }
    }
}

impl ValueError {
    pub fn value_too_big<S: Into<String>, T: Into<String>>(
        component: S,
        name: T,
        value: u32,
    ) -> ValueError {
        ValueError::ValueTooBig {
            component: component.into(),
            name: name.into(),
            value,
        }
    }

    pub fn value_too_small<S: Into<String>, T: Into<String>>(
        component: S,
        name: T,
        value: u32,
    ) -> ValueError {
        ValueError::ValueTooSmall {
            component: component.into(),
            name: name.into(),
            value,
        }
    }
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
