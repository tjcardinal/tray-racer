use crate::interval::Interval;

pub type Color = crate::vec3::Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color.x * scale;
    let g = pixel_color.y * scale;
    let b = pixel_color.z * scale;

    let intensity = Interval::new(0.0, 0.999);
    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as u8,
        (256.0 * intensity.clamp(g)) as u8,
        (256.0 * intensity.clamp(b)) as u8,
    );
}
