use crate::{
    color::{self, Color},
    hittable::Hittable,
    interval::Interval,
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            height: 100,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn render(mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.width, self.height);

        for y in 0..self.height {
            eprintln!("Scanlines remaining :{}", self.height - y);
            for x in 0..self.width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color = pixel_color + Self::ray_color(&r, self.max_depth, world);
                }
                color::write_color(pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("Done");
    }

    fn initialize(&mut self) {
        self.height = (self.width as f64 / self.aspect_ratio) as i32;
        if self.height < 1 {
            self.height = 1;
        }

        self.center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / (self.width as f64);
        self.pixel_delta_v = viewport_v / (self.height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let interval = Interval::new(0.001, f64::INFINITY);
        if let Some(rec) = world.hit(r, interval) {
            let direction = Vec3::random_on_hemisphere(rec.normal);
            0.5 * Camera::ray_color(&Ray::new(rec.p, direction), depth - 1, world)
        } else {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::random::<f64>();
        let py = -0.5 + rand::random::<f64>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
}
