mod render;

use crate::render::*;
use cgmath::{vec3, Vector2, Vector3};
use pixel_weaver::*;
use rusty_ppm::ppm_writer::write_binary_ppm;
use simple_canvas::Canvas;
use std::path::Path;
use std::time::Instant;


fn main() {
    let path = Path::new("./");

    // Single threaded image
    let mut image_st: Canvas<Vector3<u8>> = Canvas::new(WIDTH, HEIGHT, vec3(0, 0, 0));
    // Multi threaded image
    let mut image_mt: Canvas<Vector3<u8>> = Canvas::new(WIDTH, HEIGHT, vec3(0, 0, 0));

    // Produce the single threaded image 
    let st_now = Instant::now();
    main_image(&mut image_st, pixel_func_1);
    let st_now = st_now.elapsed();
    write_binary_ppm(&image_st, path, "st_img").unwrap();

    // Produce the multi-threaded image
    let mt_now = Instant::now();
    main_image_mt(&mut image_mt, pixel_func_1, THREAD_COUNT);
    let mt_now = mt_now.elapsed();
    write_binary_ppm(&image_mt, path, "mt_img").unwrap();

    // Print benchmarks
    println!("Benchmark results:");
    println!("------------------");
    println!("Image size");
    println!("Wdith:  {} pixels", image_mt.width);
    println!("Height: {} pixels", image_mt.height);
    println!("Total pixels: {} pixels", image_mt.width * image_mt.height);
    println!("Simulated workload time per pixel: {SIM_TIME:?}");
    println!("Total simulated workload: {:?}", SIM_TIME * (image_mt.width * image_mt.height) as u32);
    println!("------------------");
    println!("Single thread:\t {:?}", st_now);
    println!("Multi thread({THREAD_COUNT}): {:?}", mt_now);
}

#[allow(unused)]
fn pixel_func_1(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(image_data, coord);
    vec3(normal_to_rgb(uv.x), normal_to_rgb(uv.y), 0)
}

#[allow(unused)]
fn pixel_func_2(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(image_data, coord);
    let mut len = length(uv);
    len -= 0.5;
    let col = vec3(len, len, len);
    f64_vector3_to_u8(col)
}

#[allow(unused)]
fn pixel_func_sim(image_data: &ImageData, coord: &Vector2<u32>) -> Vector3<u8> {
    // Simulate a 1 nanosecond work load
    std::thread::sleep(std::time::Duration::new(0, 1));
    let uv = uv(image_data, coord);
    vec3(normal_to_rgb(uv.x), normal_to_rgb(uv.y), 0)
}
