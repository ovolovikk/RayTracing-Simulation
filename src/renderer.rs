use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;
use rayon::prelude::*;

pub struct RenderConfig {
    pub width: i32,
    pub height: i32,
    pub samples_per_pixel: i32,
    pub max_depth: u32,
}

// Main ray tracing function (pure path tracing)
pub fn trace_ray<F>(
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

    match world.hit(ray, 0.00001, f32::INFINITY) {
        Some(rec) => {
            if let Some(scatter) = rec.mat.scatter(ray, &rec, rng) {
                scatter.attenuation
                    * trace_ray(world, &scatter.scattered, depth - 1, rng, background)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => background(ray),
    }
}

pub fn render_scene<F>(
    world: &HittableList,
    camera: &Camera,
    config: &RenderConfig,
    background: &F,
) -> Vec<Vec<(i32, i32, i32)>>
where
    F: Fn(&Ray) -> Vec3 + Sync,
{
    (0..config.height)
        .into_par_iter()
        .rev()
        .map(|j| {
            let mut row = Vec::with_capacity(config.width as usize);
            let mut thread_rng = rand::thread_rng();

            for i in 0..config.width {
                let mut sampled_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..config.samples_per_pixel {
                    let offset_x: f32 = thread_rng.gen_range(0.0..1.0);
                    let offset_y: f32 = thread_rng.gen_range(0.0..1.0);
                    let u = (i as f32 + offset_x) / (config.width - 1) as f32;
                    let v = (j as f32 + offset_y) / (config.height - 1) as f32;

                    let ray = camera.get_ray(u, v);
                    let pixel_color =
                        trace_ray(world, &ray, config.max_depth, &mut thread_rng, background);

                    sampled_color = sampled_color + pixel_color;
                }

                sampled_color = sampled_color / config.samples_per_pixel as f32;
                row.push(translate_to_rgb(sampled_color));
            }
            row
        })
        .collect()
}

fn translate_to_rgb(color: Vec3) -> (i32, i32, i32) {
    let r = color.x.sqrt().clamp(0.0, 0.999);
    let g = color.y.sqrt().clamp(0.0, 0.999);
    let b = color.z.sqrt().clamp(0.0, 0.999);

    ((256.0 * r) as i32, (256.0 * g) as i32, (256.0 * b) as i32)
}
