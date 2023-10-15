use crate::{interval::Interval, material::Material, ray::Ray, vec3::Point3, vec3::Vec3};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        t: f64,
        r: &Ray,
        outward_normal: Vec3,
        mat: &'a dyn Material,
    ) -> HitRecord<'a> {
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

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Hittable for Vec<&(dyn Hittable + '_)> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_t = ray_t.max;
        let mut closest_hit = None;
        for hittable in self {
            let interval = Interval::new(ray_t.min, closest_t);
            if let Some(rec) = hittable.hit(r, interval) {
                closest_t = rec.t;
                closest_hit = Some(rec);
            }
        }
        closest_hit
    }
}
