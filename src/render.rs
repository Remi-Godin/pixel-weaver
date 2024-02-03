use cgmath::{vec2, vec3, Vector2, Vector3, Vector4};
use pixel_weaver::*;
use rusty_ppm::prelude::*;
use simple_canvas::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::rc::Rc;
use std::collections::VecDeque;

// Single threaded version
pub fn main_image(
    canvas: &mut Canvas<Vector3<u8>>,
    image_data: &ImageData,
    pixel_func: impl Fn(&ImageData, &Vector2<u32>) -> Vector3<u8> + Send + 'static,
) {
    for row in 0..canvas.height {
        for col in 0..canvas.width {
            //println!("ST Writing at pixel ({row}, {col})");
            *canvas.get_mut(row, col).expect("Index out of bounds") =
                pixel_func(image_data, &vec2(row as u32, col as u32));
        }
    }
}

// Multi threaded version
pub fn main_image_mt(
    canvas: &mut Canvas<Vector3<u8>>,
    image_data: Arc<ImageData>,
    pixel_func: impl Fn(&ImageData, &Vector2<u32>) -> Vector3<u8> + Send + 'static + Clone + Copy,

) {
    if (THREAD_COUNT > image_data.size as usize) {
        panic!("Cannot have more thread than pixels");
    }

    
    // Calculate the slice size. Slices are pieces of the images that will be used by the different
    // threads.
    let slice_size = image_data.size / THREAD_COUNT as u64;

    // The diff is the the difference between the offset times the thread count, and the actual
    // size of the image. This is to adjust for rounding errors when dividing.
    let slice_diff = image_data.size - (slice_size * THREAD_COUNT as u64);

    // Init the vector that will hold the slice of the image
    let mut slices: VecDeque<Vec<Vector3<u8>>> = VecDeque::new();
    
    // Create the data slices
    for i in 0..THREAD_COUNT {
        let mut curr_size = slice_size;
        if i == THREAD_COUNT - 1 {
            curr_size += slice_diff;
        }
        let mut slice: Vec<Vector3<u8>> = Vec::with_capacity(curr_size as usize);
        for j in (i * slice_size as usize)..(i * slice_size as usize + curr_size as usize) {
            slice.push(*canvas.data.get(j).unwrap());
        }
        slices.push_front(slice);
    }

    
    // Init the threads vector
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();

    // Create the result vector
    let mut result_vec: Vec<Vec<Vector3<u8>>> = Vec::with_capacity(THREAD_COUNT);

    // Temp vector to act as placeholder for our individual slices
    let mut placeholder_vec: Vec<Vector3<u8>> = Vec::new();

    // For each thread, push the temp vector
    (0..THREAD_COUNT).for_each(|e| result_vec.push(placeholder_vec.clone()));

    // Wrap our result vector in a Mutex
    let mut result_vec = Arc::new(Mutex::new(result_vec));
    
    // Init the Arc variables to pass the offset to our threads
    let offset = Arc::new(slice_size as usize);

    // Create and run the threads
    for i in 0..THREAD_COUNT {
        
        // Clone the Arc variables for use inside the threads
        let image_data = Arc::clone(&image_data);
        let offset = Arc::clone(&offset);
        let mut slice = slices.pop_back().unwrap();
        let result_vec = Arc::clone(&result_vec);

        //Create the threads and run the function on each pixel
        threads.push(thread::spawn(move || {

            // Create a variable to store the current offset of the thread. This is only to be able
            // to add the diff we calculated when slicing our original image. I can't change the
            // offset directly since we still need it for other calculations.
            let mut curr_offset = *offset;

            // If the last thread is running, add the diff to the current offset
            if i == THREAD_COUNT - 1 {
                curr_offset += slice_diff as usize;
            }

            // Iterate over the pixel of the current slice and run the pixel function on each of
            // them.
            let mut curr_row = 0;
            let mut curr_col = 0;
            for index in ((i * *offset)..(i * *offset + curr_offset)).enumerate() {
                // Figure out the row and col of the current image from its linear representation.
                curr_row = (index.1) / image_data.resolution.x as usize;
                curr_col = (index.1) - curr_row * image_data.resolution.x as usize;

                // Run the pixel function
                let pixel = pixel_func(&image_data, &vec2(curr_row as u32, curr_col as u32));
                *slice.get_mut(index.0).unwrap() = pixel;
            }

            // Lock the result vector Mutex to the current thread to allow it to add its slice to
            // it.
            let mut result_vec = result_vec.lock().unwrap();
            *result_vec.get_mut(i).unwrap() = slice;
        }));
    };

    // Join the threads
    for thread in threads {
        thread.join().expect("Could not join thread.");
    }

    // Create a new data vector the same as the one from the Canvas
    let mut data_vec: Vec<Vector3<u8>> = Vec::with_capacity(image_data.size as usize);

    // Add all the slices from the result vector together into the previously create data vector
    for slice in result_vec.lock().unwrap().iter() {
        for pixel in slice {
            data_vec.push(*pixel);
        }
    }

    // Set the canvas data to the data calculated in the multithreaded function.
    canvas.data = data_vec;
}
