use crate::hittable::{Hit, HitParams, HitResult, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use raylib::prelude::Vector3;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hits(&self, ray: &Ray, t_range: Interval<f32>) -> HitResult {
        let oc = self.center - ray.origin();

        // Quadratic equation components
        let a = ray.dir().length().powf(2.0);
        let h = ray.dir().dot(oc);
        let c = oc.length().powf(2.) - self.radius.powf(2.);

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        // Solution to the equation of the sphere, this is the value of `t`. We compute
        // each root
        let sqrd = discriminant.sqrt();

        let mut t = (h - sqrd) / a;

        if !t_range.contains(t, false) {
            t = (h + sqrd) / a;
            if !t_range.contains(t, false) {
                return None;
            }
        }

        let point = ray.eval(t);
        let normal = (point - self.center).normalized();

        Some(Hit::new(
            HitParams {
                outward_normal: normal,
                point,
                t,
            },
            ray,
        ))
    }
}
