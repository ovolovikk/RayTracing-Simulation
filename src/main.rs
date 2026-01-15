use std::ops::Sub;
use std::ops::Mul;
use std::ops::Add;

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

enum RayTraceResult {
    Hit(f32), // distance t to point
    Miss,
}

fn ray_trace(ray: &Ray, sph: &Sphere) -> RayTraceResult {
    let oc = ray.origin - sph.center;
    let a: f32 = ray.direction.length_squared();
    let b: f32 = 2.0 * oc.dot(ray.direction);
    let c: f32 = oc.length_squared() - sph.radius * sph.radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return RayTraceResult::Miss;
    }
    RayTraceResult::Hit((-b -discriminant.sqrt()) / (2.0 * a))
}

fn main() {
    // Use ```cargo test```
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        };

        assert!(matches!(ray_trace(&ray, &sphere), RayTraceResult::Hit(_f32)));
    }

    #[test]
    fn test_sphere_miss() {
        let sphere = Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 5.0,
                y: 0.0,
                z: 0.0,
            },
        };

        assert!(matches!(ray_trace(&ray, &sphere), RayTraceResult::Miss));
    }

    #[test]
    fn test_sphere_hit_distance() {
        let sphere = Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        };

        let result = ray_trace(&ray, &sphere);

        if let RayTraceResult::Hit(t) = result {
            let expected_t = 4.0;
            assert!((t - expected_t).abs() < 0.001, "Distance {t} was not close to {expected_t}");
        } else {
            panic!("Expected Hit, but got Miss");
        }
    }

    #[test]
    fn test_hit_point_coordinates() {
        let sphere = Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        };

        if let RayTraceResult::Hit(t) = ray_trace(&ray, &sphere) {
            let hit_point = ray.origin + ray.direction * t;
            assert_eq!(hit_point.x, 0.0);
            assert_eq!(hit_point.y, 0.0);
            assert_eq!(hit_point.z, 0.0);
        }
    }
}
