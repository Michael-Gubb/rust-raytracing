use core::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// Any value below this is considered zero for division purposes.
const EQUIV_ZERO: f64 = 0.0000001;
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]

pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn scale(&self, scaling_factor: f64) -> Self {
        Self {
            x: self.x * scaling_factor,
            y: self.y * scaling_factor,
            z: self.z * scaling_factor,
        }
    }
    pub fn dot_product(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    // Returns the length of the vector, can be equivalent to zero
    pub fn unchecked_length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    // Returns the length of the vector, None if equivalent to zero
    pub fn length(&self) -> Option<f64> {
        let length = self.unchecked_length();
        if length > EQUIV_ZERO {
            return Some(length);
        }
        None
    }
    /// Checks if the vector is equivalent length zero based on the minimum length DIV_MIN
    pub fn is_zero_length(&self) -> bool {
        self.unchecked_length() <= EQUIV_ZERO
    }
    /// Returns an Option containing normalised form of the vector alongside the length.
    ///
    /// Returns None if the length is equivalent to zero
    pub fn normalised(&self) -> Option<(Self, f64)> {
        self.length().map(|length| {
            let normalised = Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            };
            (normalised, length)
        })
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Vector3::new(value.0, value.1, value.2)
    }
}
impl From<Vector3> for (f64, f64, f64) {
    fn from(value: Vector3) -> Self {
        (value.x, value.y, value.z)
    }
}
impl From<[f64; 3]> for Vector3 {
    fn from(value: [f64; 3]) -> Self {
        Vector3::new(value[0], value[1], value[2])
    }
}
impl From<Vector3> for [f64; 3] {
    fn from(value: Vector3) -> Self {
        [value.x, value.y, value.z]
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vectors() {
        let zero_vector = Vector3::new(0.0, 0.0, 0.0);
        let test_vector = Vector3::new(1.0, 0.0, 0.0);
        let sum = zero_vector + test_vector;
        assert_eq!(sum, test_vector);
    }
    #[test]
    fn invert_vector() {
        let test_vector = Vector3::new(1.0, -3.0, 4.2);
        let expected_vector = Vector3::new(-1.0, 3.0, -4.2);
        let result = -test_vector;
        assert_eq!(result, expected_vector);
    }
    #[test]
    fn from_array() {
        let x = 0.0;
        let y = 2.0;
        let z = 0.0;
        let test_array = [x, y, z];
        let test_vector = Vector3::from(test_array);
        let expected_vector = Vector3::new(x, y, z);
        assert_eq!(test_vector, expected_vector);
    }
    #[test]
    fn from_tuple() {
        let x = 0.0;
        let y = 2.0;
        let z = -0.3;
        let test_tuple = (x, y, z);
        let test_vector = Vector3::from(test_tuple);
        let expected_vector = Vector3::new(x, y, z);
        assert_eq!(test_vector, expected_vector);
    }
    #[test]
    fn into_array() {
        let x = 0.0;
        let y = 2.0;
        let z = 0.0;
        let expected_array = [x, y, z];
        let test_vector = Vector3::new(x, y, z);
        let test_array: [f64; 3] = test_vector.into();
        assert_eq!(expected_array, test_array);
    }
    #[test]
    fn into_tuple() {
        let x = 0.0;
        let y = 2.0;
        let z = 0.0;
        let expected_tuple = (x, y, z);
        let test_vector = Vector3::new(x, y, z);
        let test_tuple = test_vector.into();
        assert_eq!(expected_tuple, test_tuple);
    }
    #[test]
    fn normalised_vector() {
        // A -3,4,0 vector (should be normalised to (-3/5,4/5,0) with length 5)
        let test_vector = Vector3::new(-3.0, 4.0, 0.0);
        let normalised_vector = test_vector.normalised();
        let expected_vector = Vector3::new(-3.0 / 5.0, 4.0 / 5.0, 0.0);
        let expected_length = 5.0;
        assert_eq!(Some((expected_vector, expected_length)), normalised_vector);
    }
    #[test]
    fn zero_length_vector() {
        let zero_length_vector = Vector3::new(0.0, 0.0, 0.0);
        let length = zero_length_vector.length();
        assert_eq!(length, None);
    }
    #[test]
    fn very_short_length_vector() {
        let short_vector = Vector3::new(0.0, 0.0, EQUIV_ZERO);
        let length = short_vector.length();
        assert_eq!(length, None);
    }
}
