mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;

use std::{ops::Range, rc::Rc};

use crate::sphere::Sphere;
use hittable::HittableList;
use material::{LambertianMaterial, MetalMaterial};
use raylib::prelude::*;

const IMG_WIDTH: usize = 1500;
const ASPECT_RATIO: f32 = 2.;

pub fn v3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(x, y, z)
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    let b_len = v.dot(n);
    let two_bs = n * 2.0 * b_len;
    v - two_bs
}

pub fn v3_near_zero(v: Vector3) -> bool {
    [v.x, v.y, v.z].map(f32::abs).iter().all(|n| *n < 1e-8)
}

pub fn v3_rnd_rng(rng: Range<f32>) -> Vector3 {
    let x = rand::random_range(rng.clone());
    let y = rand::random_range(rng.clone());
    let z = rand::random_range(rng.clone());
    v3(x, y, z)
}

pub fn v3_rnd() -> Vector3 {
    v3_rnd_rng(0. ..1.)
}

pub fn v3_random_unit() -> Vector3 {
    loop {
        let p = v3_rnd_rng(-1. ..1.);
        let len_squared = p.dot(p);
        // Unsure about the actual value, but we are trying to avoid bogus vectors due to
        // a FPE
        if 1e-20 < len_squared && len_squared <= 1. {
            break p / len_squared.sqrt(); // normalize
        }
    }
}

pub fn v3_random_unit_hemisphere(normal: Vector3) -> Vector3 {
    let on_u_sphere = v3_random_unit();

    // Aiming in the same general direction as the normal
    if on_u_sphere.dot(normal) > 0. {
        on_u_sphere
    } else {
        on_u_sphere * -1.
    }
}

fn main() {
    let camera = camera::Camera::new(IMG_WIDTH, ASPECT_RATIO, 10);

    let (mut rl, thread) = init()
        .size(camera.image_width() as i32, camera.image_height() as i32)
        .title("Space")
        .build();

    let mut world: HittableList = vec![];

    let metal: Rc<dyn material::Material> = Rc::new(MetalMaterial::new(v3(0.8, 0.8, 0.8), 0.5));

    let middle_material: Rc<dyn material::Material> =
        Rc::new(LambertianMaterial::new(v3(0.1, 0.2, 0.5)));

    let floor_material: Rc<dyn material::Material> =
        Rc::new(LambertianMaterial::new(v3(0.8, 0.8, 0.)));

    // First sphere
    world.push(Rc::new(Sphere::new(
        v3(0., 0., -1.2),
        0.5,
        Rc::clone(&middle_material),
    )));

    // Second sphere
    world.push(Rc::new(Sphere::new(
        v3(0., -101.0, -1.),
        100.,
        Rc::clone(&floor_material),
    )));

    // Left
    world.push(Rc::new(Sphere::new(
        v3(-1.0, 0., -1.0),
        0.5,
        Rc::clone(&metal),
    )));

    // Right
    world.push(Rc::new(Sphere::new(
        v3(1.0, 0., -1.0),
        0.5,
        Rc::clone(&metal),
    )));

    while !rl.window_should_close() {
        let mut dh = rl.begin_drawing(&thread);
        dh.clear_background(Color::BLACK);
        camera.render(&world, &mut dh);
    }
}
