#![allow(unused)]
use cgmath::{vec2, vec3, Vector2, Vector3};

#[derive(Clone)]
pub struct ImageData {
    pub resolution: Vector2<u32>,
    pub aspect_ratio: f64,
}

pub fn uv(image_data: &ImageData, coord: &Vector2<u32>) -> Vector2<f64> {
    let mut uv: Vector2<f64> = vec2(0., 0.);
    uv.y = ((coord.x as f64 / image_data.resolution.y as f64) * 2. - 1.0) * -1.;
    uv.x = ((coord.y as f64 / image_data.resolution.x as f64) * 2. - 1.0) * image_data.aspect_ratio;
    uv
}

pub fn length(coord: Vector2<f64>) -> f64 {
    (coord.x.powi(2) + coord.y.powi(2)).sqrt()
}

pub fn f64_vector3_to_u8(vector: Vector3<f64>) -> Vector3<u8> {
    vec3(
        (vector.x * 255.).clamp(0.0, 255.0) as u8,
        (vector.y * 255.).clamp(0.0, 255.0) as u8,
        (vector.z * 255.).clamp(0.0, 255.0) as u8,
    )
}

pub fn normal_to_rgb(val: f64) -> u8 {
    (val * 255.0).clamp(0., 255.) as u8
}
