use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Vec3, random_unit_vector, reflect, refract};
use rand::Rng;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

// Lambertian (diffuse) material
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

// Metal (reflective) material
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        let scattered_dir = reflected + self.fuzz * random_unit_vector(rng);

        if scattered_dir.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: Ray {
                    origin: rec.p,
                    direction: scattered_dir.normalize(),
                },
            })
        } else {
            None
        }
    }
}

// Dielectric (glass) material
pub struct Dielectric {
    pub ior: f32,
}

impl Dielectric {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<ScatterRecord> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_reflect = Self::reflectance(cos_theta, refraction_ratio) > rng.r#gen();

        let direction = if cannot_refract || should_reflect {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            attenuation,
            scattered: Ray {
                origin: rec.p,
                direction: direction.normalize(),
            },
        })
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec, rng),
            Material::Metal(m) => m.scatter(r_in, rec, rng),
            Material::Dielectric(d) => d.scatter(r_in, rec, rng),
        }
    }
}
