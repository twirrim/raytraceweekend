use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod ray;
pub mod sphere;

use crate::ray::Ray;

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

    pub fn write_colour(&self) -> String {
        let clamp = |v: f64| v.clamp(0.0, 0.999);
        let rbyte = (256.0 * clamp(self.x)) as usize;
        let gbyte = (256.0 * clamp(self.y)) as usize;
        let bbyte = (256.0 * clamp(self.z)) as usize;
        format!("{rbyte} {gbyte} {bbyte}")
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

pub fn dot(left: &Vec3, right: &Vec3) -> f64 {
    left.x * right.x + left.y * right.y + left.z * right.z
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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

// Add sphere related methods.
pub fn hit_sphere(centre: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *centre - r.origin;
    let a = r.direction.length_squared();
    let h = dot(&r.direction, &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter 'outward_normal' is assumed to have unit length

        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[rstest]
    #[case(Vec3::new(2.0, 3.0, 4.0),Vec3::new(-2.0, -3.0, -4.0))] // all positives
    #[case(Vec3::new(-2.0, -3.0, -4.0),Vec3::new(2.0, 3.0, 4.0))] // all negatives
    #[case(Vec3::new(2.0, -3.0, 4.0),Vec3::new(-2.0, 3.0, -4.0))] // mix positive/negative
    fn test_neg_vec3(#[case] give: Vec3, #[case] want: Vec3) {
        log::info!("Give: {:?}, Want: {:?}", give, want);
        let negated = -give;
        assert_eq!(negated, want);
        // double negated should match "give"
        let double_negated = -negated;
        assert_eq!(double_negated, give);
    }

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
    fn test_vec3_length_squared() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v.length_squared(), 3.0);
        let v = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(v.length_squared(), 48.0);
    }

    #[test_log::test(rstest)]
    #[rstest]
    fn test_vec3_cross() {
        let v = Vec3::new(2.0, 3.0, 4.0);
        let w = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.cross(w), Vec3::new(-2.0, 4.0, -2.0));
    }

    #[test_log::test(rstest)]
    #[rstest]
    fn test_vec3_dot() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(dot(&a, &b), 20.0);
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
