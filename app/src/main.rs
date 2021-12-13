extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate raytracer;
extern crate image;

use clap::{Arg, App};
use std::fs::{File, OpenOptions};
use raytracer::scene::*;
use image::ImageFormat;
use raytracer::point::Point;

fn main() {
    let app = App::new("raytracer")
        .version("0.1")
        .author("bheisler <redattack34@gmail.com>")
        .about("Basic Raytracer")
        .arg(Arg::with_name("image")
            .help("Sets the output image file")
            .required(true)
            .index(1));
    let matches = app.get_matches();
    println!("helloo");
    //let scene_path = matches.value_of("scene").unwrap();
    //let scene_file = File::open(scene_path).expect("File not found");
    let s = Scene {
        width: 1920,
        height: 1900,
        fov: 90.0,
        sphere: Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0
                },
                radius: 1.0,
                color: Color {
                    red: 0.2,
                    green: 1.0,
                    blue: 0.2
                },
        },
    };

    let image_path = matches.value_of("image").unwrap();

    //let scene: Scene = serde_json::from_reader(scene_file).unwrap();


    let image = raytracer::render(&s);

    let mut image_file =
        OpenOptions::new().write(true).truncate(true).create(true).open(image_path).unwrap();
    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}