#![allow(unused)]
mod render;

use crate::render::*;
use cgmath::{vec2, vec3, vec4, Vector2, Vector3, Vector4};
use pixel_perfect::*;
use rusty_ppm::ppm_writer::write_binary_ppm;
use simple_canvas::Canvas;
use std::path::Path;

fn main() {
    let mut image: Canvas<Vector3<u8>> = Canvas::new(1000, 1000, vec3(0, 0, 0));
    let image_data = ImageData {
        resolution: vec2(image.width as u32, image.height as u32),
        aspect_ratio: image.width as f64 / image.height as f64,
    };
    main_image(&mut image, &image_data, pixel_func_1);
    write_binary_ppm(&image, std::path::Path::new("./"), "out_1").unwrap();
    main_image(&mut image, &image_data, pixel_func_2);
    write_binary_ppm(&image, std::path::Path::new("./"), "out_2").unwrap();
}

fn pixel_func_1(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(&image_data, &coord);
    vec3(normal_to_rgb(uv.x), normal_to_rgb(uv.y), 0)
}

fn pixel_func_2(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(&image_data, &coord);
    let mut len = length(uv);
    len -= 0.5;
    let col = vec3(len, len, len);
    f64_vector3_to_u8(col)
}
