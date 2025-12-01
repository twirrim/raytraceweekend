use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        log::debug!("Making Vec3 using x: {x}, y: {y}, z: {z}");
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

    pub fn x(self) -> f32 {
        self.x
    }
    pub fn y(self) -> f32 {
        self.y
    }
    pub fn z(self) -> f32 {
        self.z
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
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

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self {
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        // P(t) = A + tb
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
    fn test_add(#[case] a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} + {:?} = {:?}?", a, b, want);
        assert_eq!(a + b, want);
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
    fn test_sub(#[case] a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} - {:?} = {:?}?", a, b, want);
        assert_eq!(a - b, want);
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
    fn test_mul(#[case] a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} * {:?} = {:?}?", a, b, want);
        assert_eq!(a * b, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(Vec3::new(2.0, 2.0, 2.0), 2.0, Vec3::new(4.0, 4.0, 4.0))]
    #[case(
        Vec3::new(-0.5, -0.5, -0.5),
        2.0,
        Vec3::new(-1.0, -1.0, -1.0)
    )]
    fn test_mul_f32(#[case] a: Vec3, #[case] b: f32, #[case] want: Vec3) {
        log::info!("{:?} * {:?} = {:?}?", a, b, want);
        assert_eq!(a * b, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(Vec3::new(2.0, 2.0, 2.0), 2.0, Vec3::new(1.0, 1.0, 1.0))]
    #[case(
        Vec3::new(-0.5, -0.5, -0.5),
        2.0,
        Vec3::new(-0.25, -0.25, -0.25)
    )]
    fn test_div_f32(#[case] a: Vec3, #[case] b: f32, #[case] want: Vec3) {
        log::info!("{:?} / {:?} = {:?}?", a, b, want);
        assert_eq!(a / b, want);
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
    fn test_div(#[case] a: Vec3, #[case] b: Vec3, #[case] want: Vec3) {
        log::info!("{:?} / {:?} = {:?}?", a, b, want);
        assert_eq!(a / b, want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    #[case(Vec3::new(2.0, 2.0, 2.0), Vec3::new(1.0, 1.0, 1.0), false)]
    #[case(Vec3::new(2.0, 2.0, 2.0), Vec3::new(2.0, 2.0, 2.0), true)]
    fn test_equality(#[case] a: Vec3, #[case] b: Vec3, #[case] want: bool) {
        log::info!("({:?} == {:?}) == {}?", a, b, want);
        assert_eq!(a == b, want);
        assert_eq!(a != b, !want);
    }

    #[test_log::test(rstest)]
    #[rstest]
    fn test_callables() {
        let v = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 3.0);
        assert_eq!(v.z(), 4.0);
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
        assert_eq!(r.direction(), direction);
        assert_eq!(r.origin(), origin);
    }
}
