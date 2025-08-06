use crate::{interval::Interval, material::Material, ray::Ray};
use raylib::prelude::Vector3;
use std::rc::{Rc, Weak};

pub struct Hit {
    t: f32,
    point: Vector3,
    normal: Vector3,
    material: Weak<dyn Material>,

    /// If we hit the face of the hit. Will be unset until
    /// [Hit::compute_normal_dir] is called
    front_face: bool,
}

pub struct HitParams {
    /// The distance traveled by the ray to get to the point
    pub t: f32,
    /// The point in the space where the ray hit. Satisfies:
    /// ```
    /// assert_eq!(p, ray.eval(t))
    /// ```
    pub point: Vector3,
    /// The surface normal of the surface hit at that point (pointing outwards of the surface)
    pub outward_normal: Vector3,
}

impl Hit {
    pub fn new(params: HitParams, material: Weak<dyn Material>, ray: &Ray) -> Self {
        let HitParams {
            t,
            point,
            outward_normal,
        } = params;

        let mut h = Hit {
            point,
            t,
            material,
            normal: outward_normal,
            front_face: false,
        };

        h.compute_normal_dir(ray, outward_normal);

        h
    }

    /// Compute the direction of the normal with respect to the direction of the hit.
    /// This will set [Hit::front_face] and may re-orient [Hit::normal]
    fn compute_normal_dir(&mut self, ray: &Ray, outward_normal: Vector3) {
        // The dot product computes the direction. When less than zero, this means the outward normal
        // goes in the opposite direction as the ray, meaning we've hit the exterior (the front)
        self.front_face = ray.dir().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn point(&self) -> Vector3 {
        self.point
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.upgrade().unwrap()
    }
}

pub type HitResult = Option<Hit>;

pub trait Hittable {
    /// Check if the ray hits the object. The parameter [t_range] defines the valid
    /// range of values of [HitResult::t]
    fn hits(&self, ray: &Ray, t_range: Interval<f32>) -> HitResult;
}

/// A hittable struct in an [Rc]
pub type RcHittable = Rc<dyn Hittable>;

/// A list of items that can be hit
pub type HittableList = Vec<RcHittable>;

/// Given a ray, check the appropriate surface being hit, if any.
pub fn hit_scan(ray: &Ray, t_range: Interval<f32>, hittables: &[RcHittable]) -> HitResult {
    let mut best_hit: HitResult = None;

    let low = t_range.low();
    let mut t_range = t_range;

    for hittable in hittables {
        if let Some(hit) = hittable.hits(ray, t_range) {
            t_range = Interval::new(low, hit.t());
            best_hit = Some(hit);
        }
    }

    best_hit
}
