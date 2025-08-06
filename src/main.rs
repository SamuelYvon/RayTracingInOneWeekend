#![allow(unused)]
mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;

use std::rc::Rc;

use crate::sphere::Sphere;
use hittable::HittableList;
use raylib::prelude::*;

const IMG_WIDTH: usize = 1500;
const ASPECT_RATIO: f32 = 2.;

pub fn v3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(x, y, z)
}

fn main() {
    let camera = camera::Camera::new(IMG_WIDTH, ASPECT_RATIO, 10);

    let (mut rl, thread) = init()
        .size(camera.image_width() as i32, camera.image_height() as i32)
        .title("Space")
        .build();

    let mut world: HittableList = vec![];

    // First sphere
    world.push(Rc::new(Sphere::new(v3(0., 0., -1.), 0.5)));
    // Second sphere
    world.push(Rc::new(Sphere::new(v3(0., -101.0, -1.), 100.)));

    while !rl.window_should_close() {
        let mut dh = rl.begin_drawing(&thread);
        dh.clear_background(Color::BLACK);
        camera.render(&world, &mut dh);
    }
}
