mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;

use std::rc::Rc;

use crate::color::{as_raylib_color, normal_to_color};
use crate::sphere::Sphere;
use hittable::{HittableList, hit_scan};
use interval::Interval;
use ray::Ray;
use raylib::prelude::*;

const IMG_WIDTH: i32 = 600;
const ASPECT_RATIO: f32 = 2.;

pub fn v3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(x, y, z)
}

fn main() {
    let img_height = ((IMG_WIDTH as f32 / ASPECT_RATIO) as i32).max(1);

    // The viewport is a frame within the image that contains the actual rendered image.
    let viewport_height = 2.0;
    // Use the actual aspect ratio (in case of rounding), not the configured one
    let viewport_width = viewport_height * ((IMG_WIDTH as f32) / img_height as f32);

    let focal_length = 1.;
    // Place the camera at the center
    let camera_center = Vector3::default();

    // Vector that spans the entire viewport
    let viewport_u = v3(viewport_width, 0., 0.);
    let viewport_v = v3(0., -viewport_height, 0.);

    // Compute the ratio of each viewport pixel to the image
    let viewport_img_width_ratio = viewport_u / IMG_WIDTH as f32;
    let viewport_img_height_ratio = viewport_v / img_height as f32;

    let viewport_upper_left =
        camera_center - v3(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);

    let left_top_pixel =
        viewport_upper_left + (viewport_img_width_ratio + viewport_img_height_ratio) * 0.5;

    let (mut rl, thread) = init().size(IMG_WIDTH, img_height).title("Space").build();

    let mut world: HittableList = vec![];

    // First sphere
    world.push(Rc::new(Sphere::new(v3(0., 0., -1.), 0.5)));
    // Second sphere
    world.push(Rc::new(Sphere::new(v3(0., -101.0, -1.), 100.)));

    while !rl.window_should_close() {
        let mut dh = rl.begin_drawing(&thread);
        dh.clear_background(Color::BLACK);

        for j in 0..img_height {
            println!("Progress: {j}/{img_height}\n");
            for i in 0..IMG_WIDTH {
                let pixel_center = left_top_pixel
                    + (viewport_img_width_ratio * (i as f32))
                    + (viewport_img_height_ratio * (j as f32));

                // We go from the camera to the pixel, what's the direction?
                let ray_direction = pixel_center - camera_center;

                let ray = Ray::new(pixel_center, ray_direction);

                let color =
                    if let Some(hit) = hit_scan(&ray, Interval::new(0., f32::INFINITY), &world) {
                        normal_to_color(hit.normal())
                    } else {
                        ray.height_based_color()
                    };

                let color = as_raylib_color(color);
                dh.draw_rectangle(i, j, 1, 1, color);
            }
        }

        println!();
    }
}
