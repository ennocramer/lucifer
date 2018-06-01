use std::f32::consts::FRAC_1_PI;

use cgmath::{InnerSpace, Point3};

use camera::{Camera, Resolution, Target};
use geometry::{Intersection, Ray};
use lighting::{Bsdf, Effect, Radiance};
use render::Renderer;
use scene::Scene;

#[derive(Clone, Debug)]
pub struct Light {
    pub position: Point3<f32>,
    pub emission: Radiance,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct RayTracer {
    light: Light,
}

impl RayTracer {
    pub fn new(light: Light) -> RayTracer {
        RayTracer { light }
    }

    fn phong(&mut self, ray: &Ray, intersection: &Intersection, bsdf: &Bsdf) -> Radiance {
        let light_to_intersection = self.light.position - intersection.position;
        let incidence = light_to_intersection.normalize();
        let coverage =
            (self.light.radius / light_to_intersection.magnitude()).atan() * 0.5 * FRAC_1_PI;

        let proj_ray = intersection.normal * intersection.normal.dot(ray.direction);
        let reflected_ray = (ray.direction - proj_ray * 2.0).normalize();

        let cos_t_normal = incidence.dot(intersection.normal);
        let cos_t_ray = incidence.dot(reflected_ray);

        let mut radiance = Radiance::none();

        for effect in &bsdf.effects {
            match *effect {
                Effect::Emission(emission, pdf) => radiance += emission * pdf.eval(cos_t_normal),

                Effect::DiffuseReflection(albedo, pdf) => {
                    if cos_t_normal > 0.0 {
                        radiance += cos_t_normal
                            * coverage
                            * self.light.emission
                            * albedo
                            * pdf.eval(cos_t_normal)
                    }
                }

                Effect::SpecularReflection(albedo, pdf) => {
                    if cos_t_ray > 0.0 {
                        radiance += cos_t_normal
                            * coverage
                            * self.light.emission
                            * albedo
                            * pdf.eval(cos_t_ray)
                    }
                }

                Effect::DiffuseRefraction(_, _, _) | Effect::SpecularRefraction(_, _, _) => {}
            }
        }

        radiance
    }
}

impl Renderer for RayTracer {
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
            Some(i) => self.phong(&ray, &i.intersection, &i.bsdf),
        }
    }
}
