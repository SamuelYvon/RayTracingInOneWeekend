use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle, Vector3},
};

use crate::{
    color::{ColorV3, as_raylib_color, normal_to_color},
    hittable::{RcHittable, hit_scan},
    interval::Interval,
    ray::Ray,
    v3,
};

#[allow(unused)]
pub struct Camera {
    image_width: usize,
    image_height: usize,

    viewport_height: f32,
    viewport_width: f32,

    focal_length: f32,
    camera_center: Vector3,

    viewport_u: Vector3,
    viewport_v: Vector3,

    viewport_img_width_ratio: Vector3,
    viewport_img_height_ratio: Vector3,

    viewport_upper_left: Vector3,
    left_top_pixel: Vector3,

    pixel_samples: usize,
    pixel_sample_scale: f32,
}

/// Sample a point within the virtual square of each rendered point
fn sample_square() -> Vector3 {
    let x: f32 = rand::random_range(0. ..1.);
    let y: f32 = rand::random_range(0. ..1.);
    v3(x - 0.5, y - 0.5, 0.)
}

impl Camera {
    pub fn new(width: usize, aspect_ratio: f32, pixel_samples: usize) -> Self {
        let image_height = ((width as f32 / aspect_ratio) as i32).max(1) as usize;

        // The viewport is a frame within the image that contains the actual rendered image.
        let viewport_height = 2.0;
        // Use the actual aspect ratio (in case of rounding), not the configured one
        let viewport_width = viewport_height * ((width as f32) / image_height as f32);

        let focal_length = 1.;
        // Place the camera at the center
        let camera_center = Vector3::default();

        // Vector that spans the entire viewport
        let viewport_u = v3(viewport_width, 0., 0.);
        let viewport_v = v3(0., -viewport_height, 0.);

        // Compute the ratio of each viewport pixel to the image
        let viewport_img_width_ratio = viewport_u / width as f32;
        let viewport_img_height_ratio = viewport_v / image_height as f32;

        let viewport_upper_left =
            camera_center - v3(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);

        let left_top_pixel =
            viewport_upper_left + (viewport_img_width_ratio + viewport_img_height_ratio) * 0.5;

        let pixel_sample_scale = 1. / (pixel_samples as f32);

        Self {
            image_width: width,
            image_height,
            viewport_height,
            viewport_width,
            focal_length,
            camera_center,
            viewport_u,
            viewport_v,
            viewport_img_width_ratio,
            viewport_img_height_ratio,
            viewport_upper_left,
            left_top_pixel,
            pixel_samples,
            pixel_sample_scale,
        }
    }

    pub fn image_width(&self) -> usize {
        self.image_width
    }

    pub fn image_height(&self) -> usize {
        self.image_height
    }

    /// Compute a [ColorV3] that matches the color of the sampled ray at the given position.
    fn colorize(&self, world: &[RcHittable], ray: &Ray) -> ColorV3 {
        if let Some(hit) = hit_scan(ray, Interval::new(0., f32::INFINITY), world) {
            normal_to_color(hit.normal())
        } else {
            ray.height_based_color()
        }
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = sample_square();

        let pixel_sample = self.left_top_pixel
            + (self.viewport_img_width_ratio * (x as f32 + offset.x))
            + (self.viewport_img_height_ratio * (y as f32 + offset.y));

        // We go from the camera to the pixel, what's the direction?
        let ray_direction = pixel_sample - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }

    pub fn render(&self, world: &[RcHittable], dh: &mut RaylibDrawHandle) {
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                // Compute the color by averaging [Camera::pixel_samples] pixels together, when launched from slightly random
                // positions
                let color: ColorV3 = (0..self.pixel_samples)
                    .map(|_| self.get_ray(x, y))
                    .map(|ray| self.colorize(world, &ray))
                    .fold(Vector3::default(), |a, e| a + e)
                    * self.pixel_sample_scale;

                let color = as_raylib_color(color);

                dh.draw_rectangle(x as i32, y as i32, 1, 1, color);
            }
        }
    }
}
