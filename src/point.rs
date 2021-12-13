use std::ops::{Add, Sub};
use serde::{Deserialize, Serialize};
use crate::vector::Vector3;

// use derive to automatically implement common methods for our structs

// Point: a point with an (x, y, z) on our scene, represents a pixel in this case
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// implementation for Point, zero() returns a Point at (0, 0, 0) 
impl Point {
    pub fn zero() -> Point {
        Point::from_one(0.0)
    }

    pub fn from_one(v: f64) -> Point {
        Point {x: v, y: v, z: v}
    }
}


// Various implementations for addition and subtraction 
// functionality for Points vs Points and Points vs Vectors

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, other: Vector3) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Point> for Vector3 {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        other + self
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Point> for Vector3 {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        other - self
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}