use cgmath::prelude::*;
use cgmath::Vector4;

use geometry::{Geometry, Intersection, Ray, Vector};

/// An infinite, two-dimensional plane.
#[derive(Clone, Debug)]
pub struct Plane {
    /// The plane equation.
    pub equation: Vector4<f32>,
}

impl Plane {
    /// Creates a `Plane` with and surface normal `normal`, distanced
    /// `distance` from the origin in the direction of the surface
    /// normal.
    ///
    /// # Examples
    /// ```
    /// use lucifer::geometry::Vector;
    /// use lucifer::geometry::plane::Plane;
    ///
    /// let normal = Vector::new(0.0, 0.0, 1.0);
    /// let distance = 1.0;
    /// let plane = Plane::new(normal, distance);
    ///
    /// assert_eq!(plane.equation, normal.extend(-distance));
    /// ```
    pub fn new(normal: Vector, distance: f32) -> Plane {
        let equation = normal.normalize().extend(-distance);
        Plane { equation }
    }
}

impl Geometry for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let lo = self.equation.dot(ray.origin.to_homogeneous());
        let ld = self.equation.dot(ray.direction.extend(0.0));

        let lambda = -lo / ld;
        let inside = ld > 0.0;

        if lambda <= 0.0 {
            return None;
        }

        let position = ray.origin + lambda * ray.direction;
        let mut normal = self.equation.truncate();

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
