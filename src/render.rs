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
    pixel_func: impl Fn(&ImageData, &Vector2<u32>) -> Vector3<u8> + Send + 'static,
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

pub fn main_image_mt<'a>(
    canvas: &mut Arc<Mutex<Canvas<Vector3<u8>>>>,
    image_data: Arc<ImageData>,
    pixel_func: impl Fn(&ImageData, &Vector2<u32>) -> Vector3<u8> + Send + 'static + Clone + Copy,

) {

    // Calc chunk offset for each thread
    let offset = image_data.resolution.x as usize / THREAD_COUNT; //col

    // Init the Arc variables
    // Canvas is already a Arc<Mutex<_>>
    let image_data = Arc::new(image_data);
    let offset = Arc::new(offset);
    
    // Init the threads vector
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();

    // Create and run the threads
    for i in 0..THREAD_COUNT {
        
        // Clone the Arc variables
        let canvas = Arc::clone(&canvas);
        let image_data = Arc::clone(&image_data);
        let offset = Arc::clone(&offset);

        //Create the threads and run the function on each pixel
        threads.push(thread::spawn(move || {
            let mut canvas = canvas.lock().unwrap();
            let cloned_func = pixel_func.clone();
            for col in (i * *offset)..(i * *offset + *offset) {
                for row in 0..image_data.resolution.y {
                    let pixel = pixel_func(&image_data, &vec2(col as u32, row as u32));
                    *canvas.get_mut(row as usize, col).unwrap() = pixel;
                }
            }
        }));
    };

    // Join the threads
    for thread in threads {
        thread.join().expect("Could not join thread.");
    }
}
