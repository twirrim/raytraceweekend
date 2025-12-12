use std::ops::Range;

use crate::ray::Ray;
use crate::{Point3, Vec3, dot};

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
    fn hit(&self, r: &Ray, interval: &Range<f64>) -> Option<HitRecord>;
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
    fn hit(&self, r: &Ray, interval: &Range<f64>) -> Option<HitRecord> {
        let mut local_interval = interval.clone();
        let mut hit_record = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, &local_interval) {
                local_interval.end = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
