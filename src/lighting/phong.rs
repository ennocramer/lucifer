use geometry::Intersection;
use lighting::{Albedo, Bsdf, Distribution, Effect, Material, Radiance};

/// A combination of emission, diffuse, and specular reflection.
#[derive(Clone, Debug)]
pub struct Phong {
    pub emission: Radiance,
    pub diffuse: Albedo,
    pub specular: Albedo,
    pub shininess: f32,
}

impl Phong {
    /// Creates a new `Phong` material with no emission, diffuse, or
    /// specular reflection.
    pub fn new() -> Self {
        Phong {
            emission: Radiance::none(),
            diffuse: Albedo::black(),
            specular: Albedo::black(),
            shininess: 0.0,
        }
    }

    /// Sets the emission component.
    pub fn glow(self, color: Radiance) -> Self {
        let mut mat = self;
        mat.emission = color;
        mat
    }

    /// Sets the diffuse reflection color.
    pub fn color(self, color: Albedo) -> Self {
        let mut mat = self;
        mat.diffuse = color;
        mat
    }

    /// Sets the specular reflection color and exponent.
    pub fn highlight(self, color: Albedo, shininess: f32) -> Self {
        let mut mat = self;
        mat.specular = color;
        mat.shininess = shininess;
        mat
    }
}

impl Default for Phong {
    fn default() -> Self {
        Self::new()
    }
}

impl Material for Phong {
    fn shade(&self, _: &Intersection) -> Bsdf {
        let mut bsdf = Bsdf::new();

        if self.emission != Radiance::none() {
            bsdf.effects
                .push(Effect::Emission(self.emission, Distribution::Cosine));
        }

        if self.diffuse != Albedo::black() {
            bsdf.effects.push(Effect::DiffuseReflection(
                self.diffuse,
                Distribution::Cosine,
            ));
        }

        if self.specular != Albedo::black() {
            bsdf.effects.push(Effect::SpecularReflection(
                self.specular,
                Distribution::CosineExp(self.shininess),
            ));
        }

        bsdf
    }
}
