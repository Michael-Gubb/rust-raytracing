use core::fmt;
use std::ops::{Add, Neg, Sub};
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
    pub fn length_squared(&self) -> f64{
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }
    pub fn normalised(&self) -> Option<(Self,f64)>{
        const DIV_MIN:f64 = 0.0;
        let length = self.length();
        match length <= DIV_MIN {
            true=>None,
            false=>{
                let normalised = Self{x:self.x/length,y:self.y/length,z:self.z/length};
                return Some((normalised,length))
            }
        }
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Vector3::new(value.0, value.1, value.2)
    }
}

impl From<[f64; 3]> for Vector3 {
    fn from(value: [f64; 3]) -> Self {
        Vector3::new(value[0], value[1], value[2])
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
}
