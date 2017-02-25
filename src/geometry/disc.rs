use cgmath::prelude::*;

use geometry::{Geometry, Intersection, Point, Ray, Vector};

/// A two-dimensional disc.
#[derive(Clone, Debug)]
pub struct Disc {
    /// The disc's center point.
    pub center: Point,
    /// The disc's normal.
    pub normal: Vector,
    /// The disc's radius.
    pub radius: f32,
}

impl Disc {
    /// Creates a `Disc` with radius `radius` and surface normal
    /// `normal`, centered on `center`.
    ///
    /// # Examples
    /// ```
    /// use lucifer::geometry::{Point, Vector};
    /// use lucifer::geometry::disc::Disc;
    ///
    /// let center = Point::new(0.0, 0.0, 0.0);
    /// let normal = Vector::new(0.0, 0.0, 1.0);
    /// let radius = 1.0;
    /// let disc = Disc::new(center, normal, radius);
    ///
    /// assert_eq!(disc.center, center);
    /// assert_eq!(disc.normal, normal);
    /// assert_eq!(disc.radius, radius);
    /// ```
    pub fn new(center: Point, normal: Vector, radius: f32) -> Disc {
        let normal = normal.normalize();
        Disc {
            center,
            normal,
            radius,
        }
    }
}

impl Geometry for Disc {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let lo = self.normal.dot(ray.origin - self.center);
        let ld = self.normal.dot(ray.direction);

        let lambda = -lo / ld;
        let inside = ld > 0.0;

        if lambda <= 0.0 {
            return None;
        }

        let position = ray.origin + lambda * ray.direction;
        let mut normal = self.normal;

        if (position - self.center).magnitude2() > self.radius.powi(2) {
            return None;
        }

        if inside {
            normal = -normal
        }

        Some(Intersection {
            position,
            normal,
            lambda,
            inside,
        })
    }
}
