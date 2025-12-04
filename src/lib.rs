use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn write_colour(&self) {
        let r = self.x;
        let g = self.y;
        let b = self.z;

        let rbyte = (255.999 * r) as usize;
        let gbyte = (255.999 * g) as usize;
        let bbyte = (255.999 * b) as usize;

        println!("{rbyte} {gbyte} {bbyte}");
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        self * (1.0 / t) // Optimization: Multiply by inverse is faster than division
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        v / self
    }
}

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[rstest]
    #[case(
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(3.0, 3.0, 3.0)
    )]
    #[case(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.5, 0.5, 0.5)
    )]
    fn test_add(#[case] mut a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} + {:?} = {:?}?", a, b, want);
        assert_eq!(a + b, want);
        a += b;
        assert_eq!(a, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0)
    )]
    #[case(
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-0.5, -0.5, -0.5)
    )]
    fn test_sub(#[case] mut a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} - {:?} = {:?}?", a, b, want);
        assert_eq!(a - b, want);
        a -= b;
        assert_eq!(a, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(2.0, 2.0, 2.0)
    )]
    #[case(
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(4.0, 4.0, 4.0)
    )]
    #[case(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(-1.0, -1.0, -1.0)
    )]
    fn test_mul(#[case] mut a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} * {:?} = {:?}?", a, b, want);
        assert_eq!(a * b, want);
        a *= b;
        assert_eq!(a, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(Vec3::new(2.0, 2.0, 2.0), 2.0, Vec3::new(4.0, 4.0, 4.0))]
    #[case(
        Vec3::new(-0.5, -0.5, -0.5),
        2.0,
        Vec3::new(-1.0, -1.0, -1.0)
    )]
    fn test_mul_f64(#[case] mut a: Vec3, #[case] b: f64, #[case] want: Vec3) {
        log::info!("{:?} * {:?} = {:?}?", a, b, want);
        assert_eq!(a * b, want);
        assert_eq!(b * a, want);
        a *= b;
        assert_eq!(a, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(Vec3::new(2.0, 2.0, 2.0), 2.0, Vec3::new(1.0, 1.0, 1.0))]
    #[case(
        Vec3::new(-0.5, -0.5, -0.5),
        2.0,
        Vec3::new(-0.25, -0.25, -0.25)
    )]
    fn test_div_f64(#[case] mut a: Vec3, #[case] b: f64, #[case] want: Vec3) {
        log::info!("{:?} / {:?} = {:?}?", a, b, want);
        assert_eq!(a / b, want);
        assert_eq!(b / a, want);
        a /= b;
        assert_eq!(a, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(2.0, 2.0, 2.0)
    )]
    #[case(
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(1.0, 1.0, 1.0)
    )]
    #[case(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(-0.25, -0.25, -0.25)
    )]
    fn test_div(#[case] mut a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} / {:?} = {:?}?", a, b, want);
        assert_eq!(a / b, want);
        a /= b;
        assert_eq!(a, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    fn test_vec3_creation() {
        assert_eq!(
            Vec3::new(1.0, 1.2, 1.4),
            Vec3 {
                x: 1.0,
                y: 1.2,
                z: 1.4
            }
        );
    }

    #[test_log::test(rstest)]
    #[rstest]
    fn test_ray_callables() {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let direction = Vec3::new(2.0, 2.0, 2.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.at(1.0), Point3::new(3.0, 3.0, 3.0));
        assert_eq!(r.at(2.0), Point3::new(5.0, 5.0, 5.0));
    }
}
