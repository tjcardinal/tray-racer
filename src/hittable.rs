use std::{ops::Range, rc::Rc};

use crate::{material::Material, ray::Ray, vec3::Point3, vec3::Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, r: &Ray, outward_normal: Vec3, mat: Rc<dyn Material>) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;

        Self {
            p,
            t,
            front_face,
            mat,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f64>) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        self.iter()
            .fold((None, ray_t.end), |(rec, closest), item| {
                let interval = ray_t.start..closest;
                if let Some(rec) = item.hit(r, interval) {
                    let t = rec.t;
                    (Some(rec), t)
                } else {
                    (rec, closest)
                }
            })
            .0
    }
}
