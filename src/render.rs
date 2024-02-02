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

    // Calc chunk offset for each thread
    let offset = image_data.resolution.x as usize / THREAD_COUNT; //col
    // The diff is the the difference between the offset times the thread count, and the actual
    // width of the image. This is to adjust for rounding errors when dividing.
    let diff = image_data.resolution.x as usize - offset * THREAD_COUNT;
    //println!("Image size: {}", image_data.size);
    
    // Create the data slices
    let slice_size = image_data.size / THREAD_COUNT as u64;
    let slice_diff = image_data.size - (slice_size * THREAD_COUNT as u64);
    //println!("Slice size: {}", slice_size);
    //println!("Slice diff: {}", slice_diff);

    let mut slices: VecDeque<Vec<Vector3<u8>>> = VecDeque::new();
    for i in 0..THREAD_COUNT {
        let mut curr_size = slice_size;
        if i == THREAD_COUNT - 1 {
            curr_size += slice_diff;
        }
        let mut slice: Vec<Vector3<u8>> = Vec::with_capacity(curr_size as usize);
        //println!("Thread {i} slice target: from {} to {}", i * slice_size as usize, i * slice_size as usize + curr_size as usize);
        for j in (i * slice_size as usize)..(i * slice_size as usize + curr_size as usize) {
            //println!("Trying to write at pixel {j} in a canvas size of {}", image_data.size);
            slice.push(*canvas.data.get(j).unwrap());
        }
        slices.push_front(slice);
    }

    // Init the Arc variables
    let offset = Arc::new(slice_size as usize);
    
    // Init the threads vector
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();

    // Create the result vector
    let mut result_vec: Vec<Vec<Vector3<u8>>> = Vec::with_capacity(THREAD_COUNT);
    let temp_vec3: Vector3<u8> = vec3(0, 0, 0);
    let mut temp_vec: Vec<Vector3<u8>> = Vec::new();
    temp_vec.push(temp_vec3);

    (0..THREAD_COUNT).for_each(|e| result_vec.push(temp_vec.clone()));
    let mut result_vec = Arc::new(Mutex::new(result_vec));

    // Create and run the threads
    for i in 0..THREAD_COUNT {
        
        // Clone the Arc variables 
        let image_data = Arc::clone(&image_data);
        let offset = Arc::clone(&offset);
        let mut slice = slices.pop_back().unwrap();
        let result_vec = Arc::clone(&result_vec);

        //Create the threads and run the function on each pixel
        threads.push(thread::spawn(move || {
            let mut curr_offset = *offset;
            // If the last thread is running, add the diff to the offeset
            if i == THREAD_COUNT - 1 {
                curr_offset += slice_diff as usize;
            }
            let mut curr_row = 0;
            let mut curr_col = 0;
            for index in ((i * *offset)..(i * *offset + curr_offset)).enumerate() {
                // Run the pixel function
                curr_row = (index.1) / image_data.resolution.x as usize;
                curr_col = (index.1) - curr_row * image_data.resolution.x as usize;
                let pixel = pixel_func(&image_data, &vec2(curr_row as u32, curr_col as u32));
                //println!("Current thread: {i} at ({curr_row}, {curr_col}), curr_index {}/{}", index.0, slice.len()-1);
                *slice.get_mut(index.0).unwrap() = pixel;
            }
            let mut result_vec = result_vec.lock().unwrap();
            *result_vec.get_mut(i).unwrap() = slice;
        }));
    };

    // Join the threads
    for thread in threads {
        thread.join().expect("Could not join thread.");
    }
    let mut data_vec: Vec<Vector3<u8>> = Vec::with_capacity(image_data.size as usize);
    for slice in result_vec.lock().unwrap().iter() {
        for pixel in slice {
            data_vec.push(*pixel);
        }
    }
    canvas.data = data_vec;
}
