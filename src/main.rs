mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use rand::Rng;

use camera::Camera;
use color::Color;
use hittable::Hittable;
use material::{Dielectric, Lambertain, Metal};
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let mut world: Vec<&dyn Hittable> = vec![];

    let ground_material = Lambertain::new(Color::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &ground_material);
    world.push(&ground_sphere);

    let mut lambs: Vec<Lambertain> = vec![];
    let mut mets: Vec<Metal> = vec![];
    let mut diels: Vec<Dielectric> = vec![];
    let mut test = vec![];

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if 0.9 < (center - Point3::new(4.0, 0.2, 0.0)).length() {
                match rand::random::<f64>() {
                    x if x < 0.8 => {
                        let albedo = Color::random() * Color::random();
                        lambs.push(Lambertain::new(albedo));
                        test.push((center, x));
                    }
                    x if x < 0.95 => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = rand::thread_rng().gen_range(0.5..1.0);
                        mets.push(Metal::new(albedo, fuzz));
                        test.push((center, x));
                    }
                    x => {
                        diels.push(Dielectric::new(1.5));
                        test.push((center, x));
                    }
                };
                //small_spheres.push(Sphere::new(center, 0.2, material));
            }
        }
    }

    let mut lamb_iter = lambs.iter();
    let mut mets_iter = mets.iter();
    let mut diels_iter = diels.iter();
    let mut small_spheres = vec![];
    for (center, rng) in test {
        match rng {
            x if x < 0.8 => {
                small_spheres.push(Sphere::new(center, 0.2, lamb_iter.next().unwrap()));
            }
            x if x < 0.95 => {
                small_spheres.push(Sphere::new(center, 0.2, mets_iter.next().unwrap()));
            }
            _ => {
                small_spheres.push(Sphere::new(center, 0.2, diels_iter.next().unwrap()));
            }
        }
    }

    for sphere in small_spheres.iter() {
        world.push(sphere);
    }

    let material1 = Dielectric::new(1.5);
    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &material1);
    world.push(&sphere1);

    let material2 = Lambertain::new(Color::new(0.4, 0.2, 0.1));
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &material2);
    world.push(&sphere2);

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &material3);
    world.push(&sphere3);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 960;
    cam.samples_per_pixel = 50;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
