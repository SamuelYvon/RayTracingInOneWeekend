use crate::{color::ColorV3, hittable::Hit, ray::Ray, reflect, v3_near_zero, v3_random_unit};

pub struct ScatteredLight {
    pub scattered: Ray,
    pub attenuation: ColorV3,
}

pub type ScatterLightResult = Option<ScatteredLight>;

pub trait Material {
    /// Compute the scatter on this material.
    fn scatter(&self, ray: &Ray, hit: &Hit) -> ScatterLightResult;
}

pub struct LambertianMaterial {
    albedo: ColorV3,
}

impl LambertianMaterial {
    pub fn new(albedo: ColorV3) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _: &Ray, hit: &Hit) -> ScatterLightResult {
        let direction = hit.normal() + v3_random_unit();
        let direction = if v3_near_zero(direction) {
            hit.normal()
        } else {
            direction
        };

        let scattered = Ray::new(hit.point(), direction);
        Some(ScatteredLight {
            scattered,
            attenuation: self.albedo,
        })
    }
}

pub struct MetalMaterial {
    albedo: ColorV3,
    fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: ColorV3, fuzz: f32) -> Self {
        assert!((0. ..=1.).contains(&fuzz), "Fuzz must be in [0, 1]");
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> ScatterLightResult {
        let reflected = reflect(ray.dir(), hit.normal());
        let reflected = reflected.normalized() + (v3_random_unit() * self.fuzz);

        // If the fuzz is not in the same direction, ignore it, we are heading back into the
        // object.
        if reflected.dot(hit.normal()) < 0. {
            return None;
        }

        let scattered = Ray::new(hit.point(), reflected);

        Some(ScatteredLight {
            scattered,
            attenuation: self.albedo,
        })
    }
}
