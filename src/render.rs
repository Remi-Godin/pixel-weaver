#![allow(unused)]
use cgmath::{vec2, vec3, Vector2, Vector3, Vector4};
use pixel_weaver::ImageData;
use rusty_ppm::prelude::*;
use simple_canvas::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::rc::Rc;

const THREAD_COUNT: usize = 10;

pub fn main_image(
    canvas: &mut Canvas<Vector3<u8>>,
    image_data: &ImageData,
    pixel_func: impl Fn(&ImageData, &Vector2<u32>) -> Vector3<u8> + Sync + Send + 'static,
) {
    let now = Instant::now();
    for row in 0..canvas.height {
        for col in 0..canvas.width {
            *canvas.get_mut(row, col).expect("Index out of bounds") =
                pixel_func(&image_data, &vec2(row as u32, col as u32));
        }
    }
    let now = now.elapsed();
    println!("It took {:?} to render this image.", now);
}
