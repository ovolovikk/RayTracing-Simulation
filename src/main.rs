
#[derive(Copy, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}
struct Ray {
    origin: Point3D,
    direction: Vec3D,
}

struct Sphere {
    center: Point3D,
    radius: f64
}

enum RayTraceResult {
    Hit,
    Miss
}

fn ray_trace(ray: &Ray, sph: &Sphere) -> RayTraceResult {
    let a: f64 = ray.direction * ray.direction;
    RayTraceResult::Hit
}

fn main() {
    println!("Hello, world!");
}
