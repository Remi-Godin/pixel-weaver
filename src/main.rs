#![allow(unused)]
mod render;

use crate::render::*;
use cgmath::{vec2, vec3, vec4, Vector2, Vector3, Vector4};
use pixel_weaver::*;
use rusty_ppm::ppm_writer::write_binary_ppm;
use simple_canvas::Canvas;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;


fn main() {
    let path = Path::new("./");

    // Single threaded image
    let mut image_st: Canvas<Vector3<u8>> = Canvas::new(WIDTH, HEIGHT, vec3(0, 0, 0));
    // Multi threaded image
    let mut image_mt: Canvas<Vector3<u8>> = Canvas::new(WIDTH, HEIGHT, vec3(0, 0, 0));
    let image_data: ImageData = ImageData {
        resolution: vec2(WIDTH as u32, HEIGHT as u32),
        aspect_ratio: (WIDTH as f64 / HEIGHT as f64),
        size: (WIDTH * HEIGHT) as u64

    };

    // Produce the image 
    let st_now = Instant::now();
    main_image(&mut image_st, &image_data, pixel_func_1);
    let st_now = st_now.elapsed();
    //write_binary_ppm(&image_st, path, "st_img");

    // Produce the image
    let mt_now = Instant::now();
    let image_data_arc = Arc::new(image_data.clone());
    main_image_mt(&mut image_mt, image_data_arc, pixel_func_1);
    let mt_now = mt_now.elapsed();


    //write_binary_ppm(&image_mt, path, "mt_img").unwrap();

    println!("Benchmark results:");
    println!("------------------");
    println!("Image size");
    println!("Wdith:  {} pixels", image_data.clone().resolution.x);
    println!("Height: {} pixels", image_data.clone().resolution.y);
    println!("Total pixels: {} pixels", image_data.size);
    println!("Simulated workload time per pixel: {SIM_TIME:?}");
    println!("Total simulated workload: {:?}", SIM_TIME * image_data.size as u32);
    println!("------------------");
    println!("Single thread:\t {:?}", st_now);
    println!("Multi thread({THREAD_COUNT}): {:?}", mt_now);
}

fn pixel_func_1(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(image_data, coord);
    vec3(normal_to_rgb(uv.x), normal_to_rgb(uv.y), 0)
}

fn pixel_func_2(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(image_data, coord);
    let mut len = length(uv);
    len -= 0.5;
    let col = vec3(len, len, len);
    f64_vector3_to_u8(col)
}

fn pixel_func_sim(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    // Simulate a 1 nanosecond work load
    std::thread::sleep(std::time::Duration::new(0, 1));
    vec3(0,0,0)
}
