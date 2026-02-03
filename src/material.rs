use crate::ray::Ray;
use crate::vec3::{Vec3, random_unit_vector, reflect};
use crate::hittable::HitRecord;
use rand::Rng;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

// diffuse material
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + random_unit_vector(rng);

        let direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction.normalize()
        };


        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray {
                origin: rec.p,
                direction,
            },
        })
    }
}

// mirror material
pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, _rng: &mut impl Rng) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}


pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec, rng),
            Material::Metal(m) => m.scatter(r_in, rec, rng),
        }
    }
}