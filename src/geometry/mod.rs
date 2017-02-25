//! Spatial geometry and ray-intersection math

use std::f32::INFINITY;

use cgmath;
use cgmath::{InnerSpace, Transform};

pub mod cube;
pub mod disc;
pub mod plane;
pub mod sphere;

pub use self::cube::Cube;
pub use self::disc::Disc;
pub use self::plane::Plane;
pub use self::sphere::Sphere;

/// A direction or distance in space.
pub type Vector = cgmath::Vector3<f32>;

/// A position in space.
pub type Point = cgmath::Point3<f32>;

/// A photon's (potential) path.
#[derive(Clone, Debug)]
pub struct Ray {
    /// The photon's origin.
    pub origin: Point,
    /// The photon's direction (normalized).
    pub direction: Vector,
    /// The maximum length of the path (often INFINITY).
    pub length: f32,
}

impl Ray {
    /// Creates a new `Ray` of infinite length from an `origin` and
    /// `direction`.  `direction` will be normalized.
    ///
    /// # Examples
    /// ```
    /// use lucifer::geometry::{Point, Ray, Vector};
    ///
    /// let origin = Point::new(0.0, 0.0, 0.0);
    /// let direction = Vector::new(1.0, 0.0, 0.0);
    /// let ray = Ray::new(origin, direction);
    ///
    /// assert_eq!(ray.origin, origin);
    /// assert_eq!(ray.direction, direction);
    /// assert_eq!(ray.length, std::f32::INFINITY);
    /// ```
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
            length: INFINITY,
        }
    }

    /// Creates a new `Ray` of finite length from an `origin` and an `target`.
    ///
    /// # Examples
    /// ```
    /// use lucifer::geometry::{Point, Ray};
    ///
    /// let origin = Point::new(0.0, 0.0, 0.0);
    /// let target = Point::new(1.0, 0.0, 0.0);
    /// let ray = Ray::from_endpoints(origin, target);
    ///
    /// assert_eq!(ray.origin, origin);
    /// assert_eq!(ray.direction, target - origin);
    /// assert_eq!(ray.length, 1.0);
    /// ```
    pub fn from_endpoints(origin: Point, target: Point) -> Ray {
        let distance = target - origin;
        let length = distance.magnitude();
        let direction = distance / length;

        Ray {
            origin,
            direction,
            length,
        }
    }

    /// Transform a `Ray` using any `Transform`.
    ///
    /// # Examples
    /// ```
    /// use lucifer::cgmath::{Matrix3, Transform};
    /// use lucifer::geometry::{Point, Ray, Vector};
    ///
    /// let origin = Point::new(0.0, 0.0, 0.0);
    /// let direction = Vector::new(1.0, 0.0, 0.0);
    /// let ray = Ray::new(origin, direction);
    /// let ray = ray.transform(&<Matrix as Transform<Point>>::one());
    ///
    /// assert_eq!(ray.origin, origin);
    /// assert_eq!(ray.direction, direction);
    /// assert_eq!(ray.length, std::f32::INFINITY);
    /// ```
    pub fn transform<T>(self, transform: &T) -> Ray
    where
        T: Transform<Point>,
    {
        let origin = transform.transform_point(self.origin);
        let direction = transform.transform_vector(self.direction);
        let scale = direction.magnitude();

        Ray {
            origin,
            direction: direction / scale,
            length: self.length * scale,
        }
    }
}

/// A description of a point of interaction between a photon and an
/// object.
#[derive(Clone, Debug)]
pub struct Intersection {
    /// The position in space.
    pub position: Point,
    /// The objects surface normal at the intersection point.
    pub normal: Vector,
    /// The distance from the ray's origin (always positive); that is
    /// `position = ray.origin + ray.direction * lambda`.
    pub lambda: f32,
    /// A boolean indicating whether the ray hit the inside of the
    /// object.
    pub inside: bool,
}

/// Trait describing shapes.
///
/// # Laws
///
/// `g.occlude(r) <-> g.intersect(r).is_some()`
pub trait Geometry {
    /// Compute the first (nearest to `ray`'s origin) intersection
    /// between `self` and `ray`.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;

    /// An optimized intersection test that only checks whether any
    /// intersection between `self` and `ray` exists and does not need
    /// to determine the nearest one. Can be used for faster shadow
    /// occlusion tests.
    ///
    /// The default implementation simply calls `self.intersect(ray).is_some()`.
    fn occlude(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}
