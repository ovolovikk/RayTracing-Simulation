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

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Sphere {
    center: Vec3,
    radius: f32
}

enum RayTraceResult {
    Hit,
    Miss
}

fn ray_trace(ray: &Ray, sph: &Sphere) -> RayTraceResult {
    let oc = ray.origin - sph.center;
    let a: f32 = ray.direction.length_squared();
    let b: f32 = 2.0 * oc.dot(ray.direction);
    let c: f32 = oc.length_squared() - sph.radius * sph.radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return RayTraceResult::Miss
    }
    RayTraceResult::Hit
}

fn main() {
    let sphere = Sphere {
        center: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        radius: 1.0,
    };

    // Ray into the center
    let ray_hit = Ray {
        origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
    };

    // Ray beside the sphere
    let ray_miss = Ray {
        origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        direction: Vec3 { x: 5.0, y: 0.0, z: 0.0 },
    };

    match ray_trace(&ray_hit, &sphere) {
        RayTraceResult::Hit => println!("Ray 1: Hit!"),
        RayTraceResult::Miss => println!("Ray 1: Missed"),
    }

    match ray_trace(&ray_miss, &sphere) {
        RayTraceResult::Hit => println!("Ray 2: Hit!"),
        RayTraceResult::Miss => println!("Ray 2: Missed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        };

        assert!(matches!(ray_trace(&ray, &sphere), RayTraceResult::Hit));
    }

    #[test]
    fn test_sphere_miss() {
        let sphere = Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 5.0, y: 0.0, z: 0.0 },
        };

        assert!(matches!(ray_trace(&ray, &sphere), RayTraceResult::Miss));
    }
}