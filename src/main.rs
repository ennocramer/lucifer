extern crate cgmath;
extern crate clap;
extern crate image;
extern crate rand;

extern crate lucifer;

use cgmath::num_traits::clamp;
use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, PerspectiveFov, Rad, Vector3};
use clap::{App, Arg};
use image::{Rgb, RgbImage};
use rand::{SeedableRng, XorShiftRng};
use std::path::Path;

use lucifer::camera::*;
use lucifer::geometry::*;
use lucifer::lighting::*;
use lucifer::render::*;
use lucifer::scene::*;

fn to_rgb(color: Vector3<f32>) -> Rgb<u8> {
    Rgb(color
        .map(|c| (clamp(c, 0.0, 1.0) * 255.0).round() as u8)
        .into())
}
fn to_pixel(radiance: Radiance, exposure: f32, tonemap: &Tonemap) -> Rgb<u8> {
    let v: Vector = radiance.into();
    to_rgb(v.map(|c| tonemap.apply(c * exposure)))
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

    let proj = Matrix4::from(PerspectiveFov {
        fovy: Rad::from(Deg(40.0)),
        aspect: 1.0,
        near: 1.0,
        far: 100.0,
    });

    let view = Matrix4::look_at(
        Point::new(0.0, 0.0, 6.8),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let vp = proj.concat(&view).invert().unwrap();

    let camera = AffineTransformCamera::new(vp);

    let white = Lambert::new(Albedo::new(0.725, 0.71, 0.68));
    let red = Lambert::new(Albedo::new(0.63, 0.065, 0.05));
    let green = Lambert::new(Albedo::new(0.14, 0.45, 0.091));
    let glow = Blackbody::new(Radiance::new(17.0, 12.0, 4.0));

    let mut scene = Scene::new(Radiance::none());
    scene.add(Object::new(
        Cube::new(Point::new(0.0, 2.0, 0.0), Vector::new(4.0, 0.02, 4.0)),
        white.clone(),
        Matrix4::identity(),
    ));
    scene.add(Object::new(
        Cube::new(Point::new(0.0, -2.0, 0.0), Vector::new(4.0, 0.02, 4.0)),
        white.clone(),
        Matrix4::identity(),
    ));
    scene.add(Object::new(
        Cube::new(Point::new(0.0, 0.0, -2.0), Vector::new(4.0, 4.0, 0.02)),
        white.clone(),
        Matrix4::identity(),
    ));
    scene.add(Object::new(
        Cube::new(Point::new(-2.0, 0.0, 0.0), Vector::new(0.02, 4.0, 4.0)),
        red.clone(),
        Matrix4::identity(),
    ));
    scene.add(Object::new(
        Cube::new(Point::new(2.0, 0.0, 0.0), Vector::new(0.02, 4.0, 4.0)),
        green.clone(),
        Matrix4::identity(),
    ));
    scene.add(Object::new(
        Cube::new(Point::new(0.0, 0.0, 0.0), Vector::new(1.2, 2.4, 1.2)),
        white.clone(),
        Matrix4::from_translation(Vector::new(-0.7, -0.8, -0.5))
            .concat(&Matrix4::from_angle_y(Deg(-160.0))),
    ));
    scene.add(Object::new(
        Cube::new(Point::new(0.0, 0.0, 0.0), Vector::new(1.2, 1.2, 1.2)),
        white.clone(),
        Matrix4::from_translation(Vector::new(0.7, -1.4, 0.4))
            .concat(&Matrix4::from_angle_y(Deg(160.0))),
    ));
    scene.add(Object::new(
        Cube::new(Point::new(-0.05, 1.98, 0.03), Vector::new(0.94, 0.02, 0.76)),
        glow.clone(),
        Matrix4::identity(),
    ));

    let mut renderer = PathTracer::new(XorShiftRng::from_seed([0; 16]), 0.01, 8, 512);

    let res = Resolution::new(256, 256);
    let mut img = RgbImage::new(res.width, res.height);
    for y in 0..res.height {
        for x in 0..res.width {
            let radiance = renderer.render(&scene, &camera, res, Target::new(x, y));
            img.put_pixel(x, y, to_pixel(radiance, 1.0, &Tonemap::Filmic));
        }
    }
    img.save(&Path::new(output))
        .expect("Could not save to file");
}
