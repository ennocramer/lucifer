extern crate cgmath;
extern crate clap;
extern crate image;

use cgmath::num_traits::clamp;
use cgmath::{vec3, Vector3};
use clap::{App, Arg};
use image::{Rgb, RgbImage};
use std::path::Path;

fn to_pixel(color: Vector3<f32>) -> Rgb<u8> {
    Rgb(color
        .map(|c| (clamp(c, 0.0, 1.0) * 255.0).round() as u8)
        .into())
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

    let img = RgbImage::from_fn(256, 256, |x, y| {
        to_pixel(vec3((x as f32) / 255.0, (y as f32) / 255.0, 0.5))
    });
    img.save(&Path::new(output))
        .expect("Could not save to file");
}
