
use crate::point::Point;
use std::ops::{Add, Mul};
use image::{ Pixel, Rgba};
use crate::vector::{Vector3};
use serde::{Deserialize, Serialize};
use crate::rendering::{Ray};
const GAMMA: f32 = 2.2;



// gamma encoding functions for better bit usage
fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

// Color struct to store rgb color values
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}


impl Color {
    // squish rgb values between (0.0, 1.0)
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    // convert rgb type to Rgba using gamma encoding
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        )
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode((rgba.data[0] as f32) / 255.0),
            green: gamma_decode((rgba.data[1] as f32) / 255.0),
            blue: gamma_decode((rgba.data[2] as f32) / 255.0),
        }
    }
}

// useful color manipulation functions, multiplication and addition
impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

// Sphere struct for Scene, with a center Point, radius and color
#[derive(Deserialize, Serialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
}

// stores element types that can be drawn in scene
#[derive(Deserialize, Serialize, Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Element::Sphere(sphere) => {
                let l: Vector3 = sphere.center - ray.origin;  // the hypotenuse of our right triangle 
                let adj = l.dot(&ray.direction);           // our ray is the adj side
                let d2 = l.dot(&l) - (adj * adj);
        
                let radius2 = sphere.radius * sphere.radius;
                if d2 > radius2 {
                    return None;
                }
                let thc = (radius2 - d2).sqrt();
                let t0 = adj - thc;
                let t1 = adj + thc;

                if t0 < 0.0 && t1 < 0.0 {
                    return None;
                }

                let distance = if t0 < t1 { t0 } else { t1 };
                Some(distance)
            }
            Element::Plane(plane) => {
                let normal = plane.normal;
                let denom = normal.dot(&ray.direction);
                if denom > 1e-6 {
                    let v = plane.origin - ray.origin;
                    let distance = v.dot(&normal) / denom;
                    if distance >= 0.0 {
                        return Some(distance);
                    }
                }
                None
            }
        }
    }
}
pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(d: f64, o: &'b Element) -> Intersection<'b>{
         Intersection {distance: d, object: o,}
    }
}
// Intersectable trait for checking if something intersects a Ray
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Geometry {
    pub element: Element,
    pub color: Color,
}


// our basic scene consists of width, height, fov, and elments to draw
#[derive(Deserialize, Serialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub geometry: Vec<Geometry>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.geometry
            .iter()
            .filter_map(|e| e.element.intersect(ray).map(|d| Intersection::new(d, &e.element)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}