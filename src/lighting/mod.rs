//! Materials and quantities of light and color

use std::f32::consts::PI;
use std::ops::{Mul, MulAssign};

use cgmath::{dot, vec3, ElementWise, One, Vector3, Zero};
use rand::Rng;
use smallvec::SmallVec;

use geometry::Intersection;

pub mod blackbody;
pub mod lambert;
pub mod phong;

pub use self::blackbody::Blackbody;
pub use self::lambert::Lambert;
pub use self::phong::Phong;

/// The radiant intensity of a ray of light.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    From,
    Into,
    Index,
    IndexMut,
    Add,
    AddAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Radiance(Vector3<f32>);

impl Radiance {
    /// Creates a new `Radiance` with the given intensities for red,
    /// blue, and green.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Radiance(vec3(r, g, b))
    }

    /// Creates a new `Radiance` representing absolute darkness.
    #[inline]
    pub fn none() -> Self {
        Self::gray(0.0)
    }

    /// Creates a new `Radiance` with the given red intensity.
    #[inline]
    pub fn red(f: f32) -> Self {
        Self::new(f, 0.0, 0.0)
    }

    /// Creates a new `Radiance` with the given green intensity.
    #[inline]
    pub fn green(f: f32) -> Self {
        Self::new(0.0, f, 0.0)
    }

    /// Creates a new `Radiance` with the given blue intensity.
    #[inline]
    pub fn blue(f: f32) -> Self {
        Self::new(0.0, 0.0, f)
    }

    /// Creates a new `Radiance` with equal red, green, and blue
    /// intensity.
    #[inline]
    pub fn gray(f: f32) -> Self {
        Self::new(f, f, f)
    }

    /// Computes the lightness according to NTSC.
    #[inline]
    pub fn luma(self) -> f32 {
        dot(self.0, vec3(0.21, 0.72, 0.07))
    }
}

impl Default for Radiance {
    fn default() -> Radiance {
        Radiance::none()
    }
}

impl Zero for Radiance {
    fn zero() -> Radiance {
        Radiance::none()
    }

    fn is_zero(&self) -> bool {
        *self == Radiance::none()
    }
}

impl Mul<Radiance> for f32 {
    type Output = Radiance;
    fn mul(self, rhs: Radiance) -> Radiance {
        Radiance(rhs.0 * self)
    }
}

/// The light absorption of a surface.
#[derive(
    Clone, Copy, Debug, PartialEq, From, Into, Index, IndexMut, Mul, MulAssign, Div, DivAssign,
)]
pub struct Albedo(Vector3<f32>);

impl Albedo {
    /// Creates a new `Albedo` with the given absorption factors for
    /// red, green, and blue.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Albedo(vec3(r, g, b))
    }

    /// Creates a new `Albedo` for a surface absorbing all light.
    #[inline]
    pub fn black() -> Self {
        Self::gray(0.0)
    }

    /// Creates a new `Albedo` for a surface absorbing no light.
    #[inline]
    pub fn white() -> Self {
        Self::gray(1.0)
    }

    /// Creates a new `Albedo` for a surface that absorbs all non-red
    /// light.
    #[inline]
    pub fn red(f: f32) -> Self {
        Self::new(f, 0.0, 0.0)
    }

    /// Creates a new `Albedo` for a surface that absorbs all
    /// non-green light.
    #[inline]
    pub fn green(f: f32) -> Self {
        Self::new(0.0, f, 0.0)
    }

    /// Creates a new `Albedo` for a surface that absorbs all non-blue
    /// light.
    #[inline]
    pub fn blue(f: f32) -> Self {
        Self::new(0.0, 0.0, f)
    }

    /// Creates a new `Albedo` for a surface that absorbs all colors
    /// of light equally.
    #[inline]
    pub fn gray(f: f32) -> Self {
        Self::new(f, f, f)
    }

    /// Computes the influence on lightness according to NTSC.
    #[inline]
    pub fn luma_factor(self) -> f32 {
        dot(self.0, vec3(0.21, 0.72, 0.07))
    }
}

impl Default for Albedo {
    fn default() -> Albedo {
        Albedo::white()
    }
}

impl One for Albedo {
    fn one() -> Albedo {
        Albedo::white()
    }
}

impl Mul<Albedo> for Albedo {
    type Output = Albedo;
    fn mul(self, rhs: Albedo) -> Albedo {
        Albedo(self.0.mul_element_wise(rhs.0))
    }
}

impl MulAssign<Albedo> for Albedo {
    fn mul_assign(&mut self, rhs: Albedo) {
        self.0.mul_assign_element_wise(rhs.0)
    }
}

impl Mul<Albedo> for f32 {
    type Output = Albedo;
    fn mul(self, rhs: Albedo) -> Albedo {
        Albedo(rhs.0 * self)
    }
}

impl Mul<Radiance> for Albedo {
    type Output = Radiance;
    fn mul(self, rhs: Radiance) -> Radiance {
        Radiance(self.0.mul_element_wise(rhs.0))
    }
}

impl Mul<Albedo> for Radiance {
    type Output = Radiance;
    fn mul(self, rhs: Albedo) -> Radiance {
        Radiance(self.0.mul_element_wise(rhs.0))
    }
}

impl MulAssign<Albedo> for Radiance {
    fn mul_assign(&mut self, rhs: Albedo) {
        self.0.mul_assign_element_wise(rhs.0);
    }
}

/// The refractive index.
#[derive(Clone, Copy, Debug)]
pub struct Ior(f32);

/// The distribution for light emitted, reflected, or refracted by a
/// surface.
///
/// The `Distribution` is given for a hemisphere around an axis, such
/// as the surface normal, andassumed to be isotropic around the axis.
/// It is computed from the cosine of the angle the hemisphere's
/// center axis, usually called `theta`.
#[derive(Clone, Copy, Debug)]
pub enum Distribution {
    /// All light is emitted, reflected, or refacted exactly along the
    /// axis, without any scattering.  This is the `Distribution` for a perfect
    /// mirror surface.
    Dirac,
    /// Light is emitted, reflected, or refracted uniformly over the
    /// hemisphere around the axis.
    Uniform,
    /// Light is emitted, reflected, or refracted in a cosine
    /// wheighted hemisphere around the axis.  This is the `Distribution` for a
    /// perfectly rough surface, resulting in uniform brightness from
    /// all angles.
    Cosine,
    /// Light is emitted, reflected, or refracted in an exponential
    /// drop-off based on the cosine of the angle to the axis.  This
    /// is the `Distribution` for shiny surfaces, with a higher exponent
    /// resulting in less scattering.  An exponent of `0` is equal to
    /// `Uniform`, an exponent of `1` is equal to `Cosine`, and an
    /// exponent of `infinity` is equal to `Dirac`.
    CosineExp(f32),
}

impl Distribution {
    pub fn eval(self, cos_t: f32) -> f32 {
        assert!(cos_t >= -1.0 && cos_t <= 1.0);

        if cos_t < 0.0 {
            return 0.0;
        }

        match self {
            Distribution::Dirac => if cos_t >= 1.0 {
                1.0
            } else {
                0.0
            },
            Distribution::Uniform => 1.0 / cos_t,
            Distribution::Cosine => 1.0,
            Distribution::CosineExp(e) => cos_t.powf(e - 1.0),
        }
    }

    /// Randomly sample a vector in the hemisphere around `+z`.
    ///
    /// Returns a unit-vector, randomly sampled from the hemisphere
    /// around `+z` according to the given `Distribution` and the
    /// value of the probability density function (pdf) for the returned
    /// sample.
    pub fn sample<R: Rng>(self, rng: &mut R) -> (Vector3<f32>, f32) {
        match self {
            Distribution::Dirac => (vec3(0.0, 0.0, 1.0), 0.5 / PI),
            Distribution::Uniform => {
                let x: f32 = rng.gen();
                let y: f32 = rng.gen();

                let phi = x * 2.0 * PI;
                let cos_theta = 1.0 - y;
                let r = (1.0 - cos_theta * cos_theta).sqrt();

                (vec3(r * phi.cos(), r * phi.sin(), cos_theta), 0.5 / PI)
            }
            Distribution::Cosine => {
                let x: f32 = rng.gen();
                let y: f32 = rng.gen();

                let phi = x * 2.0 * PI;
                let cos_theta = (1.0 - y).sqrt();
                let r = (1.0 - cos_theta * cos_theta).sqrt();

                (
                    vec3(r * phi.cos(), r * phi.sin(), cos_theta),
                    cos_theta / PI,
                )
            }
            Distribution::CosineExp(e) => {
                let x: f32 = rng.gen();
                let y: f32 = rng.gen();

                let phi = x * 2.0 * PI;
                let cos_theta = (1.0 - y).powf(1.0 / (e + 1.0));
                let r = (1.0 - cos_theta * cos_theta).sqrt();

                (
                    vec3(r * phi.cos(), r * phi.cos(), cos_theta),
                    (e + 1.0) * cos_theta.powf(e) / PI,
                )
            }
        }
    }
}

/// A component effect of the appearance of a surface.
#[derive(Clone, Copy, Debug)]
pub enum Effect {
    /// Light emission independent of incoming light.
    Emission(Radiance, Distribution),
    /// (Diffuse) reflection centered on surface normal.
    DiffuseReflection(Albedo, Distribution),
    /// (Specular) reflection centered on mirrored incidence vector.
    SpecularReflection(Albedo, Distribution),
    /// (Diffuse) refraction centered on inverse surface normal.
    DiffuseRefraction(Albedo, Ior, Distribution),
    /// (Specular) refracion centered on refracted incidence vector.
    SpecularRefraction(Albedo, Ior, Distribution),
}

/// The appearence of a point on a surface, described as a set of
/// `Effect`s.
#[derive(Clone, Debug)]
pub struct Bsdf {
    pub effects: SmallVec<[Effect; 5]>,
}

impl Bsdf {
    /// Creates a new and empty `Bsdf`.
    pub fn new() -> Self {
        Bsdf {
            effects: SmallVec::new(),
        }
    }
}

impl Default for Bsdf {
    fn default() -> Bsdf {
        Bsdf::new()
    }
}

/// Trait describing materials.
pub trait Material {
    fn shade(&self, intersection: &Intersection) -> Bsdf;
}
