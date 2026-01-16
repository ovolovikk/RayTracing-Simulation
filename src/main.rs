mod hittable;
mod ray;
mod sphere;
mod vec3;

use core::f32;

use hittable::Hittable;
use hittable::RayTraceResult;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

use crate::hittable::HittableList;

const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = 300;

const SPHERE: Sphere = Sphere {
    center: Vec3 {
        x: 0.0,
        y: 0.0,
        z: -2.0,
    },
    radius: 0.5,
};

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32) * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0; // basically FOV

const ORIGIN: Vec3 = Vec3 {
    x: 0.0,
    y: 0.5,
    z: 0.0,
};
const HORIZONTAL: Vec3 = Vec3 {
    x: VIEWPORT_WIDTH,
    y: 0.0,
    z: 0.0,
};
const VERTICAL: Vec3 = Vec3 {
    x: 0.0,
    y: VIEWPORT_HEIGHT,
    z: 0.0,
};

fn ray_color(mut world: &HittableList, ray: &Ray) -> Vec3 {
    match world.hit(&ray, 0.001, f32::INFINITY) {
        Some(rec) => {
            0.5 * (rec.normal
                + Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }) // Mapping normal to [0, 1]
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0); // Mapping distance to [0, 1]
            (1.0 - t)
                * Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }
                + t * Vec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                } // lerp between white and blue
        }
    }
}

fn main() {
    let mut world = HittableList {
        objects: Vec::new(),
    };

    world.objects.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        radius: 0.5,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 2.0,
            z: -2.0,
        },
        radius: 0.5,
    }));

    let lower_left_corner = ORIGIN
        - HORIZONTAL * 0.5
        - VERTICAL * 0.5
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: FOCAL_LENGTH,
        };

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

            let pixel_color = ray_color(&world, &ray);

            let r = (255.999 * pixel_color.x) as i32;
            let g = (255.999 * pixel_color.y) as i32;
            let b = (255.999 * pixel_color.z) as i32;

            println!("{r} {g} {b}");
        }
    }
}
