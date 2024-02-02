use cgmath::{vec2, vec3, Vector2, Vector3, Vector4};
use pixel_weaver::*;
use rusty_ppm::prelude::*;
use simple_canvas::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::rc::Rc;

// Single threaded version
pub fn main_image(
    canvas: &mut Canvas<Vector3<u8>>,
    image_data: &ImageData,
    pixel_func: impl Fn(&ImageData, &Vector2<u32>) -> Vector3<u8> + Send + 'static,
) {
    for row in 0..canvas.height {
        for col in 0..canvas.width {
            *canvas.get_mut(row, col).expect("Index out of bounds") =
                pixel_func(image_data, &vec2(row as u32, col as u32));
        }
    }
}

// Multi threaded version
pub fn main_image_mt(
    canvas: &mut Arc<Mutex<Canvas<Vector3<u8>>>>,
    image_data: Arc<ImageData>,
    pixel_func: impl Fn(&ImageData, &Vector2<u32>) -> Vector3<u8> + Send + 'static + Clone + Copy,

) {
    // Calc chunk offset for each thread
    let offset = image_data.resolution.x as usize / THREAD_COUNT; //col
    // The diff is the the difference between the offset times the thread count, and the actual
    // width of the image. This is to adjust for rounding errors when dividing.
    let diff = image_data.resolution.x as usize - offset * THREAD_COUNT;
    
    // Init the Arc variables
    let offset = Arc::new(offset);
    
    // Init the threads vector
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();

    // Create and run the threads
    for i in 0..THREAD_COUNT {
        
        // Clone the Arc variables
        let canvas = Arc::clone(canvas);
        let image_data = Arc::clone(&image_data);
        let offset = Arc::clone(&offset);

        //Create the threads and run the function on each pixel
        threads.push(thread::spawn(move || {
            let mut curr_offset = *offset;
            // If the last thread is running, add the diff to the offeset
            if i == THREAD_COUNT - 1 {
                curr_offset += diff;
            }
            let mut canvas = canvas.lock().unwrap();
            for col in (i * *offset)..(i * *offset + curr_offset) {
                for row in 0..image_data.resolution.y {
                    // Run the pixel function
                    //println!("Thread ({i}) working on pixel ({row}, {col})");
                    let pixel = pixel_func(&image_data, &vec2(row, col as u32));
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
