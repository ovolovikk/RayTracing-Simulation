mod hittable;
mod ray;
mod sphere;
mod vec3;

use core::f32;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

use rand::Rng;

use crate::hittable::HittableList;

const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = 1080;
const SAMPLES: i32 = 10;
const DIFFUSE_DEPTH: u32 = 50;

// Camera params
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

fn ray_color(world: &HittableList, ray: &Ray, depth: u32, rng: &mut impl Rng) -> Vec3 {
    if depth == 0 {
        return Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    match world.hit(&ray, 0.001, f32::INFINITY) {
        Some(rec) => {
            let mut rand_vec = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
            loop {
                rand_vec = Vec3 {
                        x: rng.gen_range(-1.0..1.0),
                        y: rng.gen_range(-1.0..1.0),
                        z: rng.gen_range(-1.0..1.0),
                    };
                    if rand_vec.length_squared() < 1.0 { break; }
            }

            let bounce_dir = rec.normal + rand_vec;
            let new_ray= Ray { origin: rec.p, direction: bounce_dir };

            0.5 * ray_color(world, &new_ray, depth - 1, rng)
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
            y: 0.5,
            z: -2.0,
        },
        radius: 0.5,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.0,
            z: -2.0,
        },
        radius: 100.0,
    }));

    let lower_left_corner = ORIGIN
        - HORIZONTAL * 0.5
        - VERTICAL * 0.5
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: FOCAL_LENGTH,
        };
        
    // Start of a .ppm file is always same order:
    // FORMAT, WIDTH, HEIGHT, MAX_RGB
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut sampled_color: Vec3 = Vec3{ x: 0.0, y: 0.0, z: 0.0}; 
            for k in 0..SAMPLES {
                let offset_x: f32 = rng.gen_range(0.0..1.0);
                let offset_y: f32 = rng.gen_range(0.0..1.0);
                // UV [0,1]
                let u = (i as f32 + offset_x) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + offset_y) / (IMAGE_HEIGHT - 1) as f32;
                
                let ray = Ray {
                    origin: ORIGIN,
                    direction: (lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN).normalize(),
                };
                let pixel_color = ray_color(&world, &ray, DIFFUSE_DEPTH, &mut rng);

                sampled_color = sampled_color + pixel_color;
            }
            sampled_color = sampled_color / SAMPLES as f32;
            sampled_color.x = sampled_color.x.sqrt();
            sampled_color.y = sampled_color.y.sqrt();
            sampled_color.z = sampled_color.z.sqrt();
            let r = (255.999 * sampled_color.x) as i32;
            let g = (255.999 * sampled_color.y) as i32;
            let b = (255.999 * sampled_color.z) as i32;

            println!("{r} {g} {b}");
        }
    }
}
