extern crate cgmath;
extern crate clap;
extern crate image;

extern crate lucifer;

use cgmath::num_traits::clamp;
use cgmath::prelude::*;
use cgmath::{dot, Matrix4, Ortho, Vector3};
use clap::{App, Arg};
use image::{Rgb, RgbImage};
use std::path::Path;

use lucifer::camera::*;
use lucifer::geometry::*;
use lucifer::lighting::*;

fn to_rgb(color: Vector3<f32>) -> Rgb<u8> {
    Rgb(color
        .map(|c| (clamp(c, 0.0, 1.0) * 255.0).round() as u8)
        .into())
}

fn to_pixel(radiance: Radiance) -> Rgb<u8> {
    to_rgb(radiance.into())
}

fn shade(intersection: &Intersection, bsdf: &Bsdf) -> Radiance {
    let light = Point::new(-1.0, 1.0, 1.0);
    let emission = Radiance::gray(1.0);
    let incidence = (light - intersection.position).normalize();
    let cos_t_normal = dot(incidence, intersection.normal);

    let mut radiance = Radiance::none();

    for effect in &bsdf.effects {
        match *effect {
            Effect::DiffuseReflection(albedo, pdf) => {
                if cos_t_normal > 0.0 {
                    radiance += cos_t_normal * emission * albedo * pdf.eval(cos_t_normal)
                }
            }
            Effect::Emission(_, _)
            | Effect::SpecularReflection(_, _)
            | Effect::DiffuseRefraction(_, _, _)
            | Effect::SpecularRefraction(_, _, _) => unimplemented!(),
        }
    }

    radiance
}

fn render<T: Geometry, M: Material>(scene: &T, material: &M, ray: &Ray) -> Radiance {
    match scene.intersect(ray) {
        None => Radiance::none(),
        Some(i) => shade(&i, &material.shade(&i)),
    }
}

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("output")
                .help("Output file name")
                .value_name("filename")
                .required(false)
                .default_value("lucifer.png"),
        );
    let matches = app.get_matches();

    let output = matches.value_of("output").unwrap();

    let proj = Matrix4::from(Ortho {
        left: -1.0,
        right: 1.0,
        bottom: -1.0,
        top: 1.0,
        near: 0.0,
        far: 2.0,
    });

    let view = Matrix4::look_at(
        Point::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let vp = proj.concat(&view).invert().unwrap();

    let camera = AffineTransformCamera::new(vp);

    let scene = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

    let material = Lambert::new(Albedo::red(0.7));

    let res = Resolution::new(256, 256);
    let img = RgbImage::from_fn(res.width, res.height, |x, y| {
        to_pixel(render(
            &scene,
            &material,
            &camera.primary(res, Target::new(x, y)),
        ))
    });
    img.save(&Path::new(output))
        .expect("Could not save to file");
}
