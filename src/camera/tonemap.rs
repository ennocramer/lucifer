pub enum Tonemap {
    Linear,
    Gamma(f32),
    Reinhard(f32),
    Filmic,
}

impl Tonemap {
    pub fn apply(&self, c: f32) -> f32 {
        match *self {
            Tonemap::Linear => c,
            Tonemap::Gamma(g) => c.powf(1.0 / g),
            Tonemap::Reinhard(g) => (c / (1.0 + c)).powf(1.0 / g),
            Tonemap::Filmic => {
                let x = (c - 0.004).max(0.0);
                (x * (6.2 * x + 0.5)) / (x * (6.2 * x + 1.7) + 0.06)
            }
        }
    }
}
