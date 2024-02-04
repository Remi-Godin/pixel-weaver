use crate::image_functions::*;
use crate::ImageData;
use cgmath::{
    Vector2,
    Vector3,
    vec3
};

/// This function renders a centered UV square
pub fn uv_square(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(image_data, coord);
    vec3(normal_to_rgb(uv.x), normal_to_rgb(uv.y), 0)
}

/// This function renders a faded black circle on a white background
pub fn faded_circle(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(image_data, coord);
    let len = length(uv);
    let col = vec3(len, len, len);
    f64_vector3_to_u8(col)
}

/// This function simulates a 1 nanosecond workload per pixel
pub fn pixel_func_sim(_: &ImageData, _: &Vector2<u32>) -> Vector3<u8> {
    std::thread::sleep(std::time::Duration::new(0, 1));
    vec3(255,255,255)
}
