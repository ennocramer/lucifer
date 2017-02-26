use cgmath::prelude::*;
use cgmath::Matrix4;

use camera::{Camera, Resolution, Target};
use geometry::{Point, Ray, Vector};

/// A camera model defined by an affine transformation matrix.
#[derive(Clone, Debug)]
pub struct AffineTransformCamera {
    /// The transformation matrix.
    pub transform: Matrix4<f32>,
}

impl AffineTransformCamera {
    /// Creates a new `AffineTransformCamera` with a given
    /// transformation matrix.
    pub fn new(transform: Matrix4<f32>) -> Self {
        AffineTransformCamera { transform }
    }
}

impl Camera for AffineTransformCamera {
    fn primary(&self, resolution: Resolution, target: Target) -> Ray {
        let (fx, fy) = target.normalized(resolution);
        let o = Point::new(fx, fy, -1.0);
        let t = o + Vector::new(0.0, 0.0, 2.0);

        let origin = self.transform.transform_point(o);
        let target = self.transform.transform_point(t);
        let direction = target - origin;

        Ray::new(origin, direction)
    }
}
