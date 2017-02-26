use geometry::Intersection;
use lighting::{Albedo, Bsdf, Distribution, Effect, Material};

/// An ideal diffusely reflective material.
#[derive(Clone, Debug)]
pub struct Lambert {
    pub albedo: Albedo,
}

impl Lambert {
    /// Creates a new `Lambert` material.
    pub fn new(albedo: Albedo) -> Self {
        Lambert { albedo }
    }
}

impl Material for Lambert {
    fn shade(&self, _: &Intersection) -> Bsdf {
        let mut bsdf = Bsdf::new();

        bsdf.effects
            .push(Effect::DiffuseReflection(self.albedo, Distribution::Cosine));

        bsdf
    }
}
