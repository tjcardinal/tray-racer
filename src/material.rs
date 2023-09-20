use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertain {
    pub albedo: Color,
}

impl Material for Lambertain {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction), rec.normal);
        Some((self.albedo, Ray::new(rec.p, reflected)))
    }
}
