use crate::color::ColorV3;
use crate::v3;
use raylib::prelude::Vector3;

pub struct Ray {
    origin: Vector3,
    dir: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, dir: Vector3) -> Self {
        Self { origin, dir }
    }

    /// Evaluate the ray at the given distance. When the distance is zero, the ray
    /// evaluates as its origin. (Simple linear function)
    ///
    /// # Example
    ///
    /// ```rust
    /// use raylib::prelude::*;
    ///
    /// let r = Ray {
    ///     origin: Vector3::new(5., 5., 5.),
    ///     dir: Vector3::new(1., -1., -5.)
    /// };
    ///
    /// let pos = r.eval(0.0);
    /// assert_eq!(pos, Vector3::new(5., 5., 5.));
    /// ```
    pub fn eval(&self, dist: f32) -> Vector3 {
        self.origin + (self.dir * dist)
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn dir(&self) -> Vector3 {
        self.dir
    }

    /// Compute the color of the ray based on the y-component of the ray, but **after** normalizing.
    /// Because normalizing relies implicitly on the x value (and z), the value will be influenced
    /// by x, in lesser part.
    pub fn height_based_color(&self) -> ColorV3 {
        let dir = self.dir.normalized();
        // We want to shift a ([-1, 1] to [0, 1])
        let a = (dir.y + 1.) / 2.;
        // Lerp
        Vector3::one() * (1. - a) + v3(0.5, 0.7, 1.) * a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_origin_test() {
        let r = Ray::new(Vector3::new(5., 5., 5.), Vector3::new(1., -1., -5.));
        let pos = r.eval(0.0);
        assert_eq!(pos, Vector3::new(5., 5., 5.));
    }
}
