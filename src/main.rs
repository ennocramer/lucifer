extern crate cgmath;
extern crate clap;
extern crate image;

extern crate lucifer;

use cgmath::num_traits::clamp;
use cgmath::prelude::*;
use cgmath::{Matrix4, Ortho, Vector3};
use clap::{App, Arg};
use image::{Rgb, RgbImage};
use std::path::Path;

use lucifer::geometry::*;

fn to_pixel(color: Vector3<f32>) -> Rgb<u8> {
    Rgb(color
        .map(|c| (clamp(c, 0.0, 1.0) * 255.0).round() as u8)
        .into())
}

fn render<T: Geometry>(scene: &T, ray: &Ray) -> Vector3<f32> {
    match scene.intersect(ray) {
        None => Vector::new(0.0, 0.0, 0.0),
        Some(i) => {
            let brightness = clamp(1.0 - i.lambda, 0.0, 1.0);
            let color = 0.5 * i.normal + Vector::new(0.5, 0.5, 0.5);
            color * brightness
        }
    }
}

fn primary(x: u32, y: u32, vp: &Matrix4<f32>) -> Ray {
    let fx = (x as f32) / 255.0;
    let fy = (y as f32) / 255.0;
    let origin = Point::new(fx * 2.0 - 1.0, fy * 2.0 - 1.0, -1.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    ray.transform(vp)
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

    let scene = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

    let img = RgbImage::from_fn(256, 256, |x, y| {
        to_pixel(render(&scene, &primary(x, 255 - y, &vp)))
    });
    img.save(&Path::new(output))
        .expect("Could not save to file");
}
