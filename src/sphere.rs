use crate::hittable::{HitRecord, Hittable, RayTraceResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a: f32 = ray.direction.length_squared();
        let b: f32 = 2.0 * oc.dot(ray.direction);
        let c: f32 = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-b - sqrtd) / 2.0 * a;
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / 2.0 * a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.origin + ray.direction * root;
        let outward_normal = (p - self.center) * (1.0 / self.radius);

        Some(HitRecord {
            p,
            normal: outward_normal,
            t: root,
        })
    }
}
