use std::f32::consts::PI;

use cgmath::{dot, vec3, InnerSpace, Point3, Vector3};
use rand::Rng;

use camera::{Camera, Resolution, Target};
use geometry::Ray;
use lighting::{Albedo, Effect, Radiance};
use montecarlo::{Estimator, Sample};
use render::Renderer;
use scene::Scene;

#[derive(Clone, Copy, Debug)]
pub struct PathTracer<R: Rng> {
    pub rng: R,
    pub contribution_limit: f32,
    pub depth_limit: u8,
    pub samples: u32,
}

fn secondary(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
    Ray::new(origin + direction * 0.0001, direction)
}

fn make_tangent(normal: Vector3<f32>) -> Vector3<f32> {
    if normal[0].abs() > normal[1].abs() {
        vec3(normal[2], 0.0, -normal[0]).normalize()
    } else {
        vec3(0.0, normal[2], -normal[1]).normalize()
    }
}

fn align_with(normal: Vector3<f32>, vector: Vector3<f32>) -> Vector3<f32> {
    let tangent = make_tangent(normal);
    let bitangent = normal.cross(tangent);

    vector[0] * tangent + vector[1] * bitangent + vector[2] * normal
}

impl<R: Rng> PathTracer<R> {
    pub fn new(rng: R, contribution_limit: f32, depth_limit: u8, samples: u32) -> PathTracer<R> {
        PathTracer {
            rng,
            contribution_limit,
            depth_limit,
            samples,
        }
    }

    fn trace(
        &mut self,
        scene: &Scene,
        ray: &Ray,
        contribution: Albedo,
        depth: u8,
    ) -> Sample<Radiance> {
        if depth >= self.depth_limit || contribution.luma_factor() < self.contribution_limit {
            return Sample::from(Radiance::none());
        }

        match scene.intersect(ray) {
            None => Sample::from(scene.background()),
            Some(i) => {
                let intersection = &i.intersection;
                let bsdf = &i.bsdf;

                let cos_t_view = -dot(ray.direction, intersection.normal);

                let mut sample = Sample::from(Radiance::none());

                for effect in &bsdf.effects {
                    match *effect {
                        Effect::Emission(emission, dist) => {
                            sample += Sample::from(emission * dist.eval(cos_t_view));
                        }

                        Effect::DiffuseReflection(albedo, dist) => {
                            let (v, prob) = dist.sample(&mut self.rng);
                            let cos_t_in = v[2];
                            let factor = cos_t_in * albedo * dist.eval(cos_t_view);

                            let incidence = align_with(intersection.normal, v);
                            let incoming = self.trace(
                                scene,
                                &secondary(intersection.position, incidence),
                                contribution * factor,
                                depth + 1,
                            );

                            sample += incoming * Sample::new(factor, prob * 2.0 * PI);
                        }

                        Effect::SpecularReflection(albedo, dist) => {
                            let proj_ray =
                                intersection.normal * dot(intersection.normal, ray.direction);
                            let reflected_ray = (ray.direction - proj_ray * 2.0).normalize();

                            let (v, prob) = dist.sample(&mut self.rng);
                            let cos_t_in = v[2];
                            let factor = cos_t_in * albedo * dist.eval(cos_t_view);

                            let incidence = align_with(reflected_ray, v);
                            let incoming = self.trace(
                                scene,
                                &secondary(intersection.position, incidence),
                                contribution * factor,
                                depth + 1,
                            );

                            sample += incoming * Sample::new(factor, prob * 2.0 * PI);
                        }

                        Effect::DiffuseRefraction(albedo, _, dist) => {
                            let (v, prob) = dist.sample(&mut self.rng);
                            let cos_t_in = v[2];
                            let factor = cos_t_in * albedo * dist.eval(cos_t_view);

                            let incidence = align_with(-intersection.normal, v);
                            let incoming = self.trace(
                                scene,
                                &secondary(intersection.position, incidence),
                                contribution * factor,
                                depth + 1,
                            );

                            sample += incoming * Sample::new(factor, prob * 2.0 * PI);
                        }

                        Effect::SpecularRefraction(_, _, _) => assert!(false),
                    }
                }

                sample
            }
        }
    }
}

impl<R: Rng> Renderer for PathTracer<R> {
    fn render(
        &mut self,
        scene: &Scene,
        camera: &Camera,
        resolution: Resolution,
        target: Target,
    ) -> Radiance {
        let mut estimate = Estimator::new();

        for _ in 0..self.samples {
            estimate.add(self.trace(
                scene,
                &camera.primary(resolution, target),
                Albedo::white(),
                0,
            ))
        }

        estimate.value()
    }
}
