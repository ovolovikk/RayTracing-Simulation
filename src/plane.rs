use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let denominator = r.direction.dot(self.normal);

        if denominator.abs() < 1e-6 {
            return None;
        }

        let t = (self.point - r.origin).dot(self.normal) / denominator;

        if t < t_min || t > t_max {
            return None;
        }

        let p = r.origin + r.direction * t;

        let front_face = denominator < 0.0;
        let normal = if front_face {
            self.normal
        } else {
            -self.normal
        };

        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            mat: &self.material,
        })
    }
}
