use std::ops::{Add, Mul, Sub};

#[svgbobdoc::transform]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// Defines a point or a direction in 3 dimensions.
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Returns a new vector.
    ///
    /// ```
    ///# use texture_generator::math::vector3::Vector3;
    /// let point = Vector3::new(2.0, 3.0, 4.0);
    /// assert_eq!(point.x, 2.0);
    /// assert_eq!(point.y, 3.0);
    /// assert_eq!(point.z, 4.0);
    /// ```
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Calculates the length of the vector.
    ///
    /// ```
    ///# use texture_generator::math::vector3::Vector3;
    /// let a = Vector3::new(3.0, -4.0, 0.0);
    /// let b = Vector3::new(4.0, 0.0, -3.0);
    /// let c = Vector3::new(0.0, 0.0, 0.0);
    ///
    /// assert_eq!(a.length(), 5.0);
    /// assert_eq!(b.length(), 5.0);
    /// assert_eq!(c.length(), 0.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Normalizes the vector so that its length is 1.
    ///
    /// ```
    ///# use texture_generator::math::vector3::Vector3;
    /// let mut vector = Vector3::new(3.0, -4.0, 0.0);
    /// vector.normalize();
    ///
    /// assert_eq!(vector.x, 0.6);
    /// assert_eq!(vector.y, -0.8);
    /// assert_eq!(vector.z, 0.0);
    /// ```
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    /// Calculates the euclidean distance to another vector.
    ///
    /// ```
    ///# use texture_generator::math::vector3::Vector3;
    /// let a = Vector3::new(1.0, 2.0, -3.0);
    /// let b = Vector3::new(4.0, 6.0, -3.0);
    /// let c = Vector3::new(8.0, 6.0, 0.0);
    ///
    /// assert_eq!(a.calculate_distance(&a), 0.0);
    /// assert_eq!(a.calculate_distance(&b), 5.0);
    /// assert_eq!(b.calculate_distance(&a), 5.0);
    /// assert_eq!(b.calculate_distance(&c), 5.0);
    /// ```
    pub fn calculate_distance(&self, point: &Vector3) -> f32 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2) + (self.z - point.z).powi(2))
            .sqrt()
    }

    /// Calculates the dot product.
    ///
    /// ```
    ///# use texture_generator::math::vector3::Vector3;
    /// let a = Vector3::new(1.0, 2.0, -3.0);
    /// let b = Vector3::new(4.0, 6.0, -1.0);
    ///
    /// assert_eq!(a.dot(&b), 19.0);
    /// ```
    pub fn dot(&self, point: &Vector3) -> f32 {
        self.x * point.x + self.y * point.y + self.z * point.z
    }
}

/// Adds a [`Vector3`] to another [`Vector3`].
///
/// ```
///# use texture_generator::math::vector3::Vector3;
///# use texture_generator::math::size::Size;
/// let a = Vector3::new(1.1, 2.2, 3.0);
/// let b = Vector3::new(30.0, 50.2, -3.0);
/// let result = Vector3::new(31.1, 52.4, 0.0);
///
/// assert_eq!(a + b, result);
/// assert_eq!(b + a, result);
/// ```
impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Subtracts a [`Vector3`] from another [`Vector3`].
///
/// ```
///# use texture_generator::math::vector3::Vector3;
///# use texture_generator::math::size::Size;
/// let a = Vector3::new(1.1, 2.2, 3.0);
/// let b = Vector3::new(30.0, 50.2, -3.0);
///
/// assert_eq!(a - b, Vector3::new(-28.9, -48.0, 6.0));
/// assert_eq!(b - a, Vector3::new(28.9, 48.0, -6.0));
/// ```
impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Multiplies a [`Vector3`] with a float.
///
/// ```
///# use texture_generator::math::vector3::Vector3;
///# use texture_generator::math::size::Size;
/// let vector = Vector3::new(1.1, 2.2, 3.0);
///
/// assert_eq!(vector * 0.5, Vector3::new(0.55, 1.1, 1.5));
/// ```
impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, value: f32) -> Vector3 {
        Vector3::new(self.x * value, self.y * value, self.z * value)
    }
}
