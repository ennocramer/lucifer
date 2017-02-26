use cgmath::{InnerSpace, Matrix, Matrix4, SquareMatrix, Transform};

use geometry::*;
use lighting::*;

/// An object in the scene, given by a `Geometry` with a specific
/// `Material` and positioned using a transformation defined by a
/// `Matrix4<f32>`.
pub struct Object<'a> {
    pub geometry: Box<Geometry + 'a>,
    pub material: Box<Material + 'a>,
    pub transform: Matrix4<f32>,
    pub inv_transform: Matrix4<f32>,
}

impl<'a> Object<'a> {
    pub fn new<G, S>(geometry: G, material: S, transform: Matrix4<f32>) -> Self
    where
        G: Geometry + 'a,
        S: Material + 'a,
    {
        Object {
            geometry: Box::new(geometry),
            material: Box::new(material),
            transform,
            inv_transform: transform.invert().unwrap(),
        }
    }

    fn transform_ray(&self, ray: &Ray) -> Ray {
        ray.clone().transform(&self.inv_transform)
    }

    fn transform_intersection(&self, ray: &Ray, intersection: &Intersection) -> Intersection {
        let inv_trans = self.inv_transform.transpose();
        let position = self.transform.transform_point(intersection.position);
        let normal =
            Transform::<Point>::transform_vector(&inv_trans, intersection.normal).normalize();
        let lambda = (position - ray.origin).magnitude();
        let inside = intersection.inside;
        Intersection {
            position,
            normal,
            lambda,
            inside,
        }
    }
}

pub struct ShadedIntersection {
    pub intersection: Intersection,
    pub bsdf: Bsdf,
}

pub struct Scene<'a> {
    objects: Vec<Object<'a>>,
    background: Radiance,
}

impl<'a> Scene<'a> {
    pub fn new(background: Radiance) -> Scene<'a> {
        Scene {
            objects: Vec::new(),
            background,
        }
    }

    pub fn background(&self) -> Radiance {
        self.background
    }

    pub fn add(&mut self, object: Object<'a>) {
        self.objects.push(object)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<ShadedIntersection> {
        let mut nearest: Option<(Intersection, &Object)> = None;

        for obj in &self.objects {
            if let Some(int) = obj.geometry.intersect(&obj.transform_ray(ray)) {
                nearest = match nearest {
                    None => Some((int, obj)),
                    Some((i, o)) => {
                        if int.lambda < i.lambda {
                            Some((int, obj))
                        } else {
                            Some((i, o))
                        }
                    }
                }
            }
        }

        match nearest {
            None => None,
            Some((intersection, object)) => {
                let bsdf = object.material.shade(&intersection);
                Some(ShadedIntersection {
                    intersection: object.transform_intersection(ray, &intersection),
                    bsdf,
                })
            }
        }
    }

    pub fn occlude(&self, ray: &Ray) -> bool {
        for obj in &self.objects {
            if obj.geometry.occlude(&obj.transform_ray(ray)) {
                return true;
            }
        }

        false
    }
}
