mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use rand::Rng;
use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::Hittable;
use material::{Dielectric, Lambertain, Material, Metal};
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let ground_material = Rc::new(Lambertain::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if 0.9 < (center - Point3::new(4.0, 0.2, 0.0)).length() {
                let material: Rc<dyn Material> = match rand::random::<f64>() {
                    x if x < 0.8 => {
                        let albedo = Color::random() * Color::random();
                        Rc::new(Lambertain::new(albedo))
                    }
                    x if x < 0.95 => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = rand::thread_rng().gen_range(0.5..1.0);
                        Rc::new(Metal::new(albedo, fuzz))
                    }
                    _ => Rc::new(Dielectric::new(1.5)),
                };
                world.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertain::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 1200;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
