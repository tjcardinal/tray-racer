pub type Color = crate::vec3::Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = linear_to_gamma(pixel_color.x * scale);
    let g = linear_to_gamma(pixel_color.y * scale);
    let b = linear_to_gamma(pixel_color.z * scale);

    let (min, max) = (0.0, 0.999);
    println!(
        "{} {} {}",
        (256.0 * r.clamp(min, max)) as u8,
        (256.0 * g.clamp(min, max)) as u8,
        (256.0 * b.clamp(min, max)) as u8,
    );
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
