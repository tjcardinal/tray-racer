mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::Hittable;
use material::{Lambertain, Material, Metal};
use sphere::Sphere;
use vec3::Point3;

fn main() {
    let material_ground = Rc::new(Lambertain {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertain {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    });

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            material_center,
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ];

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
