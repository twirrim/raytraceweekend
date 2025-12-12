use std::ops::Range;

use crate::hit::{HitRecord, Hittable};
use crate::{Point3, Ray, Vec3, dot};

#[derive(Debug)]
pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Self {
            centre,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Range<f64>) -> Option<HitRecord> {
        let oc: Vec3 = self.centre - ray.origin;
        let a = ray.direction.length_squared();
        let h = dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.contains(&root) {
            root = (h + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.centre) / self.radius;

        let mut rec = HitRecord {
            t,
            p,
            normal: outward_normal,
            front_face: false, // placeholder
        };

        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point3, Ray, Vec3};
    use rstest::rstest;

    fn almost_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn test_new_clamps_negative_radius() {
        let s = Sphere::new(Point3::new(0.0, 0.0, 0.0), -2.5);
        assert_eq!(s.radius, 0.0);
    }

    #[test]
    fn test_new_positive_radius() {
        let s = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5.0);
        assert_eq!(s.radius, 5.0);
    }

    #[rstest]
    #[case(
        // Ray hits center
        Point3::new(0.0, 0.0, 0.0), 1.0, // sphere at origin, radius 1
        Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 1.0)), // from z=-3 toward +z (center)
        Some(2.0), // first intersection at t=2.0
        Vec3::new(0.0, 0.0, -1.0), // normal at hit
        true // ray is outside in, so should be front face
    )]
    #[case(
        // Ray misses sphere
        Point3::new(0.0, 0.0, 0.0), 1.0, // sphere at origin, radius 1
        Ray::new(Vec3::new(0.0, 2.0, -3.0), Vec3::new(0.0, 0.0, 1.0)),
        None,
        Vec3::new(0.0, 0.0, 0.0),
        false
    )]
    #[case(
        // Ray tangent to the sphere at y=1
        Point3::new(0.0, 0.0, 0.0), 1.0,
        Ray::new(Vec3::new(1.0, 1.0, -3.0), Vec3::new(0.0, 0.0, 1.0)),
        None, // should be a miss because at this configuration no root in tmin..tmax
        Vec3::new(0.0, 0.0, 0.0),
        false
    )]
    #[case(
        // Ray inside sphere, pointing outward
        Point3::new(0.0, 0.0, 0.0), 1.0,
        Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Some(1.0),
        Vec3::new(0.0, 0.0, -1.0),
        false // ray starts inside, so not front face
    )]
    #[case(
        // Ray intersection, t within ray_tmax
        Point3::new(0.0, 0.0, 0.0), 1.0,
        Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 1.0)),
        Some(2.0),
        Vec3::new(0.0, 0.0, -1.0),
        true
    )]
    fn test_hit(
        #[case] centre: Point3,
        #[case] radius: f64,
        #[case] ray: Ray,
        #[case] want_t: Option<f64>,
        #[case] want_normal: Vec3,
        #[case] want_front: bool,
    ) {
        let s = Sphere::new(centre, radius);

        let res = s.hit(&ray, &(0.001..2.5));
        match want_t {
            Some(want_val) => {
                assert!(res.is_some(), "Expected a hit but got None");
                let rec = res.unwrap();
                assert!(
                    almost_eq(rec.t, want_val),
                    "t mismatch: got {}, expected {}",
                    rec.t,
                    want_val
                );
                assert!(
                    almost_eq(rec.normal.x, want_normal.x)
                        && almost_eq(rec.normal.y, want_normal.y)
                        && almost_eq(rec.normal.z, want_normal.z),
                    "normal mismatch: got {:?}, want {:?}",
                    rec.normal,
                    want_normal
                );
                assert_eq!(
                    rec.front_face, want_front,
                    "front_face expected {:?}",
                    want_front
                );
            }
            None => {
                assert!(res.is_none(), "Expected no hit, but got Some(HitRecord)");
            }
        }
    }
}
