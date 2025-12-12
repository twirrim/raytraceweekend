use crate::{Point3, Ray, Vec3, dot};
use crate::hit::{HitRecord, Hittable};

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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
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
