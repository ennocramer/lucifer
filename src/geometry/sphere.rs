use cgmath::prelude::*;
use cgmath::BaseFloat;

use geometry::{Geometry, Intersection, Point, Ray};

#[inline]
fn project<V: InnerSpace>(x: V, y: V) -> V::Scalar
where
    V::Scalar: BaseFloat,
{
    x.dot(y) / y.magnitude2()
}

/// A sphere.
#[derive(Clone, Debug)]
pub struct Sphere {
    /// The sphere's center point.
    pub center: Point,
    /// The sphere's radius.
    pub radius: f32,
}

impl Sphere {
    /// Creates a `Sphere` with radius `radius`, centered on `center`.
    ///
    /// # Examples
    /// ```
    /// use lucifer::geometry::Point;
    /// use lucifer::geometry::sphere::Sphere;
    ///
    /// let center = Point::new(0.0, 0.0, 0.0);
    /// let radius = 1.0;
    /// let sphere = Sphere::new(center, radius);
    ///
    /// assert_eq!(sphere.center, center);
    /// assert_eq!(sphere.radius, radius);
    /// ```
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let alpha = project(self.center - ray.origin, ray.direction);
        let r = ray.origin + ray.direction * alpha - self.center;
        let beta = self.radius.powi(2) - r.magnitude2();

        if beta < 0.0 {
            return None;
        }

        let gamma = (beta / ray.direction.magnitude2()).sqrt();
        let inside = gamma >= alpha;
        let lambda = if inside { alpha + gamma } else { alpha - gamma };

        if lambda <= 0.0 {
            return None;
        }

        let position = ray.origin + lambda * ray.direction;
        let mut normal = (position - self.center) / self.radius;

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
