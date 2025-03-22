pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default> Default for Vec2<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T: PartialEq + Into<f32> + Copy> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        let diff_x: f32 = (self.x.into() - other.x.into()).abs();
        let diff_y: f32 = (self.y.into() - other.y.into()).abs();
        diff_x < 0.0001 && diff_y < 0.0001
    }
}

impl<T> Vec2<T>
where
    T: Into<f32> + Copy,
{
    pub fn magnitude(&self) -> f32 {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        (x * x + y * y).sqrt()
    }

    pub fn normalized(&mut self) -> Vec2<f32> {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vec2::new(0.0, 0.0)
        } else {
            Vec2::new(self.x.into() / mag, self.y.into() / mag)
        }
    }

    pub fn dot(&self, other: &Vec2<T>) -> f32 {
        let x1: f32 = self.x.into();
        let y1: f32 = self.y.into();
        let x2: f32 = other.x.into();
        let y2: f32 = other.y.into();
        x1 * x2 + y1 * y2
    }

    pub fn cross(&self, other: &Vec2<T>) -> f32 {
        let x1: f32 = self.x.into();
        let y1: f32 = self.y.into();
        let x2: f32 = other.x.into();
        let y2: f32 = other.y.into();
        x1 * y2 - y1 * x2
    }

    pub fn angle(&self, other: &Vec2<T>) -> f32 {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();
        if mag1 == 0.0 || mag2 == 0.0 {
            0.0
        } else {
            let cos_theta = self.dot(other) / (mag1 * mag2);
            cos_theta.acos()
        }
    }

    pub fn rotate_around(&self, pivot: &Vec2<T>, angle: f32) -> Vec2<f32> {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        let x = self.x.into() - pivot.x.into();
        let y = self.y.into() - pivot.y.into();
        let rotated_x = x * cos_theta - y * sin_theta;
        let rotated_y = x * sin_theta + y * cos_theta;
        Vec2::new(rotated_x + pivot.x.into(), rotated_y + pivot.y.into())
    }
}

impl<T> Clone for Vec2<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl<T> Copy for Vec2<T> where T: Copy {}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::ops::Add;
impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

use std::ops::Sub;
impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

use std::ops::Mul;
impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Mul for Vec2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

use std::ops::Div;
impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Div for Vec2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_f32() {
        let vec = Vec2::new(1.0, 2.0);

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
    }

    #[test]
    fn test_new_integer() {
        let vec = Vec2::new(1, 2);

        assert_eq!(vec.x, 1);
        assert_eq!(vec.y, 2);
    }

    #[test]
    fn test_addition() {
        let vec1 = Vec2::new(2, 3);
        let vec2 = Vec2::new(4, 5);
        let result = vec1 + vec2;
        assert_eq!(result.x, 6);
        assert_eq!(result.y, 8);
    }

    #[test]
    fn test_subtraction() {
        let vec1 = Vec2::new(5, 7);
        let vec2 = Vec2::new(2, 3);
        let result = vec1 - vec2;
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 4);
    }

    #[test]
    fn test_multiplication() {
        let vec = Vec2::new(3, 4);
        let result = vec * 2;
        assert_eq!(result.x, 6);
        assert_eq!(result.y, 8);

        let result = vec * result;
        assert_eq!(result.x, 18);
        assert_eq!(result.y, 32);
    }

    #[test]
    fn test_division() {
        let vec = Vec2::new(10.0, 20.0);
        let result = vec / 2.0;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 10.0);

        let result = vec / result;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    fn test_magnitude() {
        let vec = Vec2::new(3.0, 4.0);
        let mag = vec.magnitude();
        assert!((mag - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalized_nonzero() {
        let mut vec = Vec2::new(3.0, 4.0);
        let norm = vec.normalized();
        let mag = norm.magnitude();
        // Normalized vector should have magnitude 1 (or close due to floating point precision)
        assert!((mag - 1.0).abs() < 1e-6);
        // Check direction is maintained: dot product should be close to original magnitude.
        let dot = vec.dot(&Vec2::new(norm.x, norm.y));
        assert!((dot / vec.magnitude() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalized_zero() {
        let mut vec = Vec2::new(0.0, 0.0);
        let norm = vec.normalized();
        assert_eq!(norm.x, 0.0);
        assert_eq!(norm.y, 0.0);
    }

    #[test]
    fn test_dot() {
        let vec1 = Vec2::new(1.0, 2.0);
        let vec2 = Vec2::new(3.0, 4.0);
        let dot = vec1.dot(&vec2);
        assert!((dot - 11.0).abs() < 1e-6);
    }

    #[test]
    fn test_cross() {
        let vec1 = Vec2::new(1.0, 2.0);
        let vec2 = Vec2::new(3.0, 4.0);
        let cross = vec1.cross(&vec2);
        assert!((cross - (-2.0)).abs() < 1e-6);
    }

    #[test]
    fn test_angle() {
        let vec1 = Vec2::new(1.0, 0.0);
        let vec2 = Vec2::new(0.0, 1.0);
        let angle = vec1.angle(&vec2);
        // Angle between (1,0) and (0,1) should be 90 degrees or PI/2 radians.
        assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
    }
}
