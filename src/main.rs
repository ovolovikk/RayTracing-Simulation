mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod camera;

use core::f32;
use material::{Lambertian, Material, Metal};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use rayon::prelude::*;

use crate::hittable::HittableList;
use crate::camera::Camera;

const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = 1080;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: u32 = 50;

fn ray_color<F>(
    world: &HittableList,
    ray: &Ray,
    depth: u32,
    rng: &mut impl Rng,
    background: &F,
) -> Vec3
where
    F: Fn(&Ray) -> Vec3,
{
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, f32::INFINITY) {
        Some(rec) => {
            if let Some(s_rec) = rec.mat.scatter(ray, &rec, rng) {
                s_rec.attenuation * ray_color(world, &s_rec.scattered, depth - 1, rng, background)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => background(ray),
    }
}

fn translate_to_rgb_color(sampled_color: &mut Vec3) -> (i32, i32, i32) {
    let r = sampled_color.x.sqrt().clamp(0.0, 0.999);
    let g = sampled_color.y.sqrt().clamp(0.0, 0.999);
    let b = sampled_color.z.sqrt().clamp(0.0, 0.999);

    (
        (256.0 * r) as i32,
        (256.0 * g) as i32,
        (256.0 * b) as i32,
    )
}
fn main() {
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let material_ground = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });

    let material_center = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });

    let material_metal  = Material::Metal(Metal { 
        albedo: Vec3::new(0.8, 0.6, 0.2) ,
        fuzz: 0.1,
    });

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_metal,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_center,
    }));

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );

    let day_sky = |r: &Ray| {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let sky_color = (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } 
                      + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    
        let sun_direction = Vec3 { x: 1.0, y: 1.0, z: -1.0 }.normalize();
        let sun_focus = unit_direction.dot(sun_direction).max(0.0);
        
        let sun_intensity = sun_focus.powf(200.0); 
        
        sky_color + Vec3 { x: 10.0, y: 10.0, z: 8.0 } * sun_intensity
    };

    let deep_space = |_r: &Ray| Vec3::new(0.0, 0.0, 0.0);

    let starry_space = |r: &Ray| {
        let unit_dir = r.direction.normalize();

        let stars =
            (unit_dir.x * 400.0).sin() * (unit_dir.y * 350.0).sin() * (unit_dir.z * 900.0).sin();
        if stars > 0.998 {
            Vec3::new(1.0, 1.0, 1.0)
        } else {
            deep_space(r)
        }
    };

    // Start of a .ppm file is always same order:
    // FORMAT, WIDTH, HEIGHT, MAX_RGB
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    let pixels: Vec<Vec<(i32, i32, i32)>> = (0.. IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(|j| {
            let mut row = Vec::with_capacity(IMAGE_WIDTH as usize);
            let mut thread_rng = rand::thread_rng();
                for i in 0..IMAGE_WIDTH {
                    
                    let mut sampled_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);

                    for _k in 0..SAMPLES_PER_PIXEL {
                        let offset_x: f32 = thread_rng.gen_range(0.0..1.0);
                        let offset_y: f32 = thread_rng.gen_range(0.0..1.0);
                        // UV [0,1]
                        let u = (i as f32 + offset_x) / (IMAGE_WIDTH - 1) as f32;
                        let v = (j as f32 + offset_y) / (IMAGE_HEIGHT - 1) as f32;
        
                        let ray = camera.get_ray(u, v);
                        let pixel_color = ray_color(&world, &ray, MAX_DEPTH, &mut thread_rng, &day_sky);
        
                        sampled_color = sampled_color + pixel_color;
                    }

                    sampled_color = sampled_color / SAMPLES_PER_PIXEL as f32;
                    row.push(translate_to_rgb_color(&mut sampled_color));
            }
            row
        })
        .collect();

    for row in pixels {
        for rgb in row {
            println!("{} {} {}", rgb.0, rgb.1, rgb.2);
        }
    }
}
