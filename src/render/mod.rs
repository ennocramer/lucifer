use camera::{Camera, Resolution, Target};
use lighting::Radiance;
use scene::Scene;

pub mod debug;
pub mod path;
pub mod ray;

pub use self::debug::DebugRenderer;
pub use self::path::PathTracer;
pub use self::ray::RayTracer;

pub trait Renderer {
    fn render(
        &mut self,
        scene: &Scene,
        camera: &Camera,
        resolution: Resolution,
        target: Target,
    ) -> Radiance;
}
