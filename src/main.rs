use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length_squared(&self) -> f32 {
        self.dot(*self)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

struct HitRecord {
    hit_position: Vec3,
    hit_normal: Vec3,
    hit_distance: f32,
}

enum RayTraceResult {
    Hit(HitRecord),
    Miss,
}

fn ray_trace(ray: &Ray, sphere: &Sphere) -> RayTraceResult {
    let oc = ray.origin - sphere.center;
    let a: f32 = ray.direction.length_squared();
    let b: f32 = 2.0 * oc.dot(ray.direction);
    let c: f32 = oc.length_squared() - sphere.radius * sphere.radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return RayTraceResult::Miss;
    }
    let t: f32 = (-b - discriminant.sqrt()) / (2.0 * a);
    let hit_pos: Vec3 = ray.origin + ray.direction * t;
    RayTraceResult::Hit(HitRecord {
        hit_position: (hit_pos),
        hit_normal: ((hit_pos - sphere.center) * (1.0 / sphere.radius)),
        hit_distance: (t),
    })
}

fn main() {
    // Use ```cargo test```
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPHERE: Sphere = Sphere {
        center: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        radius: 3.0,
    };
    
    const RAY: Ray = Ray {
        origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
    };

    #[test]
    fn test_sphere_hit_variant() {
        assert!(matches!(ray_trace(&RAY, &SPHERE), RayTraceResult::Hit(_)));
    }

    #[test]
    fn test_sphere_hit_data() {
        let result = ray_trace(&RAY, &SPHERE);

        if let RayTraceResult::Hit(rec) = result {
            assert!((rec.hit_distance - 2.0).abs() < 0.001);
            assert_eq!(rec.hit_position.z, -2.0);

            assert_eq!(rec.hit_normal.x, 0.0);
            assert_eq!(rec.hit_normal.y, 0.0);
            assert_eq!(rec.hit_normal.z, 1.0);
        } else {
            panic!("Expected Hit, but got Miss");
        }
    }
}