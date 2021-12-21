extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate image;

use clap::{Arg, App};
use std::fs::{File, OpenOptions};
use raytracer::scene::*;
use image::ImageFormat;
use raytracer::point::Point;

fn main() {
    // use clap to access Command Line args for image output location
    let app = App::new("raytracer")
        .version("0.1")
        .author("bheisler <redattack34@gmail.com>")
        .about("Basic Raytracer")
        .arg(Arg::with_name("image")
            .help("Sets the output image file")
            .required(true)
            .index(2))
        .arg(Arg::with_name("scene")
            .help("Sets the scene json file")
            .required(true)
            .index(1));
    let matches = app.get_matches(); // get the args
  

    //TODO: file reading not working, ask Ari
    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    // Alternative approach: hard-code in a Scene rather than reading from a file
    // not ideal but will do for now until above code is fixed.
    let s = Scene {
        width: 1920,
        height: 1900,
        fov: 90.0,
        geometry: vec![ Geometry { 
            element: Element::Sphere(Sphere { 
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0
                },
                radius: 1.0, }),
            color: Color {
                    red: 0.2,
                    green: 1.0,
                    blue: 0.2
                },
        }],
    };

    // unwrap image path from Args
    let image_path = matches.value_of("image").unwrap();
    print!("{:?}", scene_file);
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    // use render fn to render our Scene
    let image = raytracer::render(&scene);

    // generate file and save to file with PNG format
    let mut image_file =
        OpenOptions::new().write(true).truncate(true).create(true).open(image_path).unwrap();
    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}