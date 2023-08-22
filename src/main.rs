mod camera;
mod color;
mod hittable;
mod interval;
mod point3;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use point3::Point3;
use sphere::Sphere;

fn main() {
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
