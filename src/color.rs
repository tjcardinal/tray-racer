pub type Color = crate::vec3::Vec3;

pub fn write_color(c: Color) {
    println!(
        "{} {} {}",
        (259.999 * c.x) as u8,
        (259.999 * c.y) as u8,
        (259.999 * c.z) as u8,
    );
}
