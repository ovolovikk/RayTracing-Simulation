use simple_ray_tracer::camera::Camera;
use simple_ray_tracer::hittable::HittableList;
use simple_ray_tracer::material::*;
use simple_ray_tracer::plane::Plane;
use simple_ray_tracer::ray::Ray;
use simple_ray_tracer::renderer::{RenderConfig, render_scene};
use simple_ray_tracer::sphere::Sphere;
use simple_ray_tracer::vec3::Vec3;

const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = 1080;
const SAMPLES_PER_PIXEL: i32 = 10000;
const MAX_DEPTH: u32 = 50;

fn main() {
    // Scene setup
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let material_ground = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });

    let material_center = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });

    let material_metal = Material::Metal(Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.02,
    });

    let material_glass = Material::Dielectric(Dielectric { ior: 1.5 });
    let material_glass_inner = Material::Dielectric(Dielectric { ior: 1.5 });

    world.objects.push(Box::new(Plane {
        point: Vec3::new(0.0, -0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: material_ground,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(1.5, 0.0, -1.0),
        radius: 0.5,
        material: material_center,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_metal,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(-1.5, 0.0, -1.0),
        radius: 0.5,
        material: material_glass,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(-1.5, 0.0, -1.0),
        radius: -0.4,
        material: material_glass_inner,
    }));

    let camera = Camera::new(
        Vec3::new(0.0, 1.0, 2.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );

    let day_sky = |r: &Ray| {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let sky_color = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);

        let sun_direction = Vec3::new(1.0, 1.0, -1.0).normalize();
        let sun_focus = unit_direction.dot(sun_direction).max(0.0);
        let sun_intensity = sun_focus.powf(50.0);

        sky_color + Vec3::new(20.0, 20.0, 16.0) * sun_intensity
    };

    let config = RenderConfig {
        width: IMAGE_WIDTH,
        height: IMAGE_HEIGHT,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
    };

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    let pixels = render_scene(&world, &camera, &config, &day_sky);
    
    for row in pixels {
        for (r, g, b) in row {
            println!("{r} {g} {b}");
        }
    }
}
