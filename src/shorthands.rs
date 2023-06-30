use macroquad::prelude::*;

#[inline]
pub fn flip_x() -> Mat4 {
    Mat4::from_scale(vec3(-1.0, 1.0, 1.0))
}
#[inline]
pub fn flip_y() -> Mat4 {
    Mat4::from_scale(vec3(1.0, -1.0, 1.0))
}
#[inline]
pub fn flip_xy() -> Mat4 {
    Mat4::from_scale(vec3(-1.0, -1.0, 1.0))
}

#[inline]
pub fn scale(x: f32, y: f32) -> Mat4 {
    Mat4::from_scale(vec3(x, y, 1.0))
}
#[inline]
pub fn upscale(factor: f32) -> Mat4 {
    scale(factor, factor)
}
#[inline]
pub fn downscale(factor: f32) -> Mat4 {
    upscale(1.0 / factor)
}

#[inline]
pub fn shift(x: f32, y: f32) -> Mat4 {
    Mat4::from_translation(vec3(x, y, 0.0))
}
#[inline]
pub fn rotate_cw(radians: f32) -> Mat4 {
    Mat4::from_rotation_z(radians)
}
#[inline]
pub fn rotate_cc(radians: f32) -> Mat4 {
    rotate_cw(-radians)
}
