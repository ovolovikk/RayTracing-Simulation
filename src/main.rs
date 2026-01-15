use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = 300;

const SPHERE:Sphere = Sphere {
    center: Vec3 { x: 0.0, y: 0.0, z: -2.0 },
    radius: 0.5,
};

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32) * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0; // basically FOV

const ORIGIN: Vec3 = Vec3 { x: 0.0, y: 0.5, z: 0.0 };
const HORIZONTAL: Vec3 = Vec3 { x: VIEWPORT_WIDTH, y: 0.0, z: 0.0 };
const VERTICAL: Vec3 = Vec3 { x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0 };

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

    fn length(&self) ->f32 {
        self.length_squared().sqrt()
    }

    fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 { *self } else { *self * (1.0 / len)}
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

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
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

fn ray_color (ray: &Ray, sphere: &Sphere) -> Vec3 {
    match ray_trace(ray, sphere) {
        RayTraceResult::Hit(rec) => {
            0.5 * (rec.hit_normal + Vec3 { x: 1.0, y: 1.0, z: 1.0}) // Mapping normal to [0, 1]
        }
        RayTraceResult::Miss => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0); // Mapping distance to [0, 1]
            (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0} + t * Vec3{x: 0.5, y: 0.7, z: 1.0 } // lerp between white and blue
        }
    }
}

fn main() {
    let lower_left_corner = ORIGIN - HORIZONTAL * 0.5 - VERTICAL * 0.5 - Vec3 { x: 0.0, y: 0.0, z: FOCAL_LENGTH };

    // FORMAT, WIDTH, HEIGHT, MAX_RGB 
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n 255");

    for j in (0..IMAGE_HEIGHT).rev() {

        for i in 0..IMAGE_WIDTH {
            // UV [0,1]
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let ray = Ray {
                origin: ORIGIN,
                direction: lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            };

            let pixel_color = ray_color(&ray, &SPHERE);

            let r = (255.999 * pixel_color.x) as i32;
            let g = (255.999 * pixel_color.y) as i32;
            let b = (255.999 * pixel_color.z) as i32;

            println!("{r} {g} {b}");
        }
    }
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