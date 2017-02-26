use geometry::Intersection;
use lighting::{Bsdf, Distribution, Effect, Material, Radiance};

/// A pure emitter of light.
#[derive(Clone, Debug)]
pub struct Blackbody {
    pub radiance: Radiance,
}

impl Blackbody {
    /// Creates a new `Blackbody` material.
    pub fn new(radiance: Radiance) -> Self {
        Blackbody { radiance }
    }
}

impl Material for Blackbody {
    fn shade(&self, _: &Intersection) -> Bsdf {
        let mut bsdf = Bsdf::new();

        bsdf.effects
            .push(Effect::Emission(self.radiance, Distribution::Cosine));

        bsdf
    }
}
