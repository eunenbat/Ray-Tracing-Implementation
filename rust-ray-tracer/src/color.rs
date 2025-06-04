use crate::vec3::Vec3;
pub type Color = Vec3;

pub fn write_color(pixel_color: Color) -> (f64, f64, f64) {
    let rbyte = 255.999 * pixel_color.x();
    let gbyte = 255.999 * pixel_color.y();
    let bbyte = 255.999 * pixel_color.z();

    println!("{} {} {}", rbyte, gbyte, bbyte);
    (rbyte, gbyte, bbyte)
}
