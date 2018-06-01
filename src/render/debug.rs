use cgmath::num_traits::clamp;

use camera::{Camera, Resolution, Target};
use geometry::{Intersection, Vector};
use lighting::Radiance;
use render::Renderer;
use scene::Scene;

#[derive(Clone, Debug, Default)]
pub struct DebugRenderer {}

impl DebugRenderer {
    pub fn new() -> DebugRenderer {
        DebugRenderer {}
    }

    fn visualize(&mut self, intersection: &Intersection) -> Radiance {
        let brightness = clamp(1.0 - intersection.lambda / 9.0, 0.0, 1.0);
        let color = 0.5 * intersection.normal + Vector::new(0.5, 0.5, 0.5);
        Radiance::from(color * brightness)
    }
}

impl Renderer for DebugRenderer {
    fn render(
        &mut self,
        scene: &Scene,
        camera: &Camera,
        resolution: Resolution,
        target: Target,
    ) -> Radiance {
        let ray = camera.primary(resolution, target);
        match scene.intersect(&ray) {
            None => scene.background(),
            Some(i) => self.visualize(&i.intersection),
        }
    }
}
