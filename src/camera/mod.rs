//! Camera types and view mapping

use geometry::Ray;

pub mod affine;
pub mod tonemap;

pub use self::affine::AffineTransformCamera;
pub use self::tonemap::Tonemap;

/// A pair of `u32` representing the resolution of an image.
#[derive(Clone, Copy, Debug)]
pub struct Resolution {
    /// The horizontal resolution.
    pub width: u32,
    /// The vertical resolutions
    pub height: u32,
}

impl Resolution {
    /// Creates a new `Resolution`.
    ///
    /// # Examples
    /// ```
    /// use lucifer::camera::Resolution;
    ///
    /// let res = Resolution::new(1024, 768);
    ///
    /// assert_eq!(res.width, 1024);
    /// assert_eq!(res.height, 768);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        Resolution { width, height }
    }
}

/// A pixel-position within an image buffer.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    /// The horizontal position.
    pub x: u32,
    /// The vertical position.
    pub y: u32,
}

impl Target {
    /// Creates a new `Target`.
    ///
    /// # Examples
    /// ```
    /// use lucifer::camera::Target;
    ///
    /// let t = Target::new(0, 1);
    ///
    /// assert_eq!(t.x, 0);
    /// assert_eq!(t.y, 1);
    /// ```
    pub fn new(x: u32, y: u32) -> Self {
        Target { x, y }
    }

    /// Map the pixel coordinate to the unit square, such that the
    /// image covers the range `[-1, +1]`, with `+x` to the right and
    /// `+y` to the top.  Each pixel index is transformed to the
    /// floating point coordinate at the center of the pixel.
    ///
    /// # Examples
    /// ```
    /// use lucifer::camera::{Resolution, Target};
    ///
    /// let res = Resolution::new(2, 2);
    /// let t = Target::new(1, 0);
    ///
    /// assert_eq!(t.normalized(&res), (0.5, 0.5))
    /// ```
    pub fn normalized(self, resolution: Resolution) -> (f32, f32) {
        let step_x = 2.0 / (resolution.width as f32);
        let step_y = 2.0 / (resolution.height as f32);
        let fx = (self.x as f32) * step_x;
        let fy = (self.y as f32) * step_y;

        (fx - 1.0 + 0.5 * step_x, 1.0 - fy - 0.5 * step_y)
    }
}

/// A Trait describing a camera.
pub trait Camera {
    /// Construct a `Ray` to compute the light reaching a given
    /// `Target` in a render buffer of a given `Resolution`.
    fn primary(&self, resolution: Resolution, target: Target) -> Ray;
}
