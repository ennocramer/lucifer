use std;

use cgmath::prelude::*;

use geometry::{Geometry, Intersection, Point, Ray, Vector};

/// An axis-aligned cube.
#[derive(Clone, Debug)]
pub struct Cube {
    /// The cube's center point.
    pub center: Point,
    /// The cube's "radius". The cube extends from `center -
    /// radius` to `center + radius.
    pub radius: Vector,
}

impl Cube {
    /// Creates a `Cube` of size `dimensions`, centered on `center`.
    ///
    /// # Examples
    /// ```
    /// use lucifer::geometry::{Point, Vector};
    /// use lucifer::geometry::cube::Cube;
    ///
    /// let center = Point::new(0.0, 0.0, 0.0);
    /// let dimensions = Vector::new(2.0, 2.0, 2.0);
    /// let cube = Cube::new(center, dimensions);
    ///
    /// assert_eq!(cube.center, center);
    /// assert_eq!(cube.radius, dimensions / 2.0);
    /// ```
    pub fn new(center: Point, dimensions: Vector) -> Cube {
        let radius = dimensions / 2.0;
        Cube { center, radius }
    }
}

impl Geometry for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let vmin: [f32; 3] = (self.center - self.radius - ray.origin)
            .div_element_wise(ray.direction)
            .into();

        let vmax: [f32; 3] = (self.center + self.radius - ray.origin)
            .div_element_wise(ray.direction)
            .into();

        let lin = vmin
            .iter()
            .zip(vmax.iter())
            .map(|(a, b)| if a < b { (a, -1.0) } else { (b, 1.0) })
            .enumerate()
            .fold((std::f32::NEG_INFINITY, 0.0, 0), |acc, (d, x)| {
                if *x.0 > acc.0 {
                    (*x.0, x.1, d)
                } else {
                    acc
                }
            });

        let lout = vmin
            .iter()
            .zip(vmax.iter())
            .map(|(a, b)| if a > b { (a, -1.0) } else { (b, 1.0) })
            .enumerate()
            .fold((std::f32::INFINITY, 0.0, 0), |acc, (d, x)| {
                if *x.0 < acc.0 {
                    (*x.0, x.1, d)
                } else {
                    acc
                }
            });

        if lout.0 < lin.0 {
            return None;
        }

        let inside = lin.0 <= 0.0;
        let (lambda, n, dim) = if inside { lout } else { lin };

        if lambda <= 0.0 {
            return None;
        }

        let position = ray.origin + lambda * ray.direction;
        let mut normal = Vector::zero();
        normal[dim] = n;

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
