use camera::{Camera, Resolution, Target};
use lighting::Radiance;
use scene::Scene;

pub mod debug;

pub use self::debug::DebugRenderer;

pub trait Renderer {
    fn render(
        &mut self,
        scene: &Scene,
        camera: &Camera,
        resolution: Resolution,
        target: Target,
    ) -> Radiance;
}
