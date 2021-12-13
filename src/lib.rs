
extern crate serde_derive;
extern crate image;
extern crate serde;

pub mod scene;
pub mod point;
pub mod vector;
mod rendering;

use scene::Scene;
use image::{DynamicImage, GenericImage, Rgba, Pixel};
use rendering::Ray;
use rendering::Intersectable;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                println!("{}", 1);
                image.put_pixel(x, y, scene.sphere.color.to_rgba())
                
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

// #[test]
// fn test_can_render_scene() {
//     let scene = Scene {
//         width: 800,
//         height: 600,
//         fov: 90.0,
//         sphere: scene::Sphere { 
//             center: Point {
//                 x: 0.0, 
//                 y: 0.0, 
//                 z: -5.0}, 
//             radius: 1.0, 
//             color: scene::Color { red: 0.4, green: 1.0, blue: 0.4 }, },
//     };
//     let img: DynamicImage = render(&scene);
//     let w = img.width();
//     assert_eq!(scene.width, img.width());
//     assert_eq!(scene.height, img.height());
// }