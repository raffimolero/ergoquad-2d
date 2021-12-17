use macroquad::prelude::*;

pub fn shift(x: f32, y: f32) -> Mat4 {
    Mat4::from_translation(vec3(x, y, 0.0))
}
pub fn further(z: f32) -> Mat4 {
    Mat4::from_translation(vec3(0.0, 0.0, z))
}
pub fn closer(z: f32) -> Mat4 {
    further(-z)
}

#[allow(dead_code)]
pub fn flip_x() -> Mat4 {
    Mat4::from_scale(vec3(-1.0, 1.0, 1.0))
}
pub fn flip_y() -> Mat4 {
    Mat4::from_scale(vec3(1.0, -1.0, 1.0))
}
#[allow(dead_code)]
pub fn flip_xy() -> Mat4 {
    Mat4::from_scale(vec3(-1.0, -1.0, 1.0))
}

pub fn rotate_x(pitch: f32) -> Mat4 {
    Mat4::from_rotation_x(pitch)
}
pub fn rotate_y(yaw: f32) -> Mat4 {
    Mat4::from_rotation_y(yaw)
}
pub fn rotate_z(roll: f32) -> Mat4 {
    Mat4::from_rotation_z(roll)
}

pub fn upscale(scale: f32) -> Mat4 {
    Mat4::from_scale(vec3(scale, scale, 1.0))
}
pub fn downscale(scale: f32) -> Mat4 {
    upscale(1.0 / scale)
}
