use raylib::color::Color;
use raylib::prelude::Vector3;

pub type ColorV3 = Vector3;

pub fn as_raylib_color(color: ColorV3) -> Color {
    let (r, g, b) = (color.x, color.y, color.z);

    let r = linear_space_to_gamma_space(r);
    let g = linear_space_to_gamma_space(g);
    let b = linear_space_to_gamma_space(b);

    Color::new(
        // Allow "above" 255 for numerical error. It
        (255.999 * r) as u8,
        (255.9999 * g) as u8,
        (255.9999 * b) as u8,
        0xFF,
    )
}

/// Maps a surface normal to a color
pub fn normal_to_color(normal: Vector3) -> ColorV3 {
    let (x, y, z) = (normal.x, normal.y, normal.z);
    ColorV3::new(x + 1., y + 1., z + 1.) * 0.5
}

fn linear_space_to_gamma_space(s: f32) -> f32 {
    if s > 0. { s.sqrt() } else { 0. }
}
