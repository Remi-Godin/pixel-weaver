use crate::image_functions::*;
use crate::ImageData;
use cgmath::*;

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

/// This function renders 3 spheres on top of a place
pub fn spheres(image_data: &ImageData, pixel_coord: &Vector2<u32>) -> Vector3<u8> {
    let uv = uv(&image_data, pixel_coord);

    let ro = vec3(0., 0., -3.); // Ray Origin
    let rd = vec3(uv.x, uv.y, 1.).normalize(); // Ray Direction
    let mut dist = 0.; 

    for _ in 0..80 {
        let p = ro + rd * dist;

        let d: f64 = dist_to_surf(p);

        dist += d;

        if d < 0.001 {break;};
        if dist > 1000.0 {break;};
    }
    dist *= 0.2;

    f64_vector3_to_u8(vec3(dist, dist, dist))
}

fn dist_to_surf(p: Vector3<f64>) -> f64 {
    let sphere = (p.x.powi(2) + p.y.powi(2) + p.z.powi(2)).sqrt() - 1.;
    let sphere_2 = ((p.x + 1.0).powi(2) + (p.y + 1.0).powi(2) + (p.z + 1.0).powi(2)).sqrt() - 1.;
    let sphere_3 = ((p.x - 1.0).powi(2) + (p.y - 1.0).powi(2) + (p.z - 1.5).powi(2)).sqrt() - 1.;
    let plane = p.y + 1.;
    f64::min(sphere, plane).min(sphere_2).min(sphere_3)
}
