use crate::point::Point;
use crate::vector::Vector3;
use crate::scene::{Scene, Sphere, Color};

// the Ray Struct is our main rendering struct
// consists of an origin Point which is by convention at (0, 0, 0)
// (but will be adjusted to be (0, 0, -0.5) in calculations), and
// a direction Vector
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    // a Prime ray is defined as a ray coming from a Point/Pixel in our Scene, to 
    // the origin camera through our imaginary sensor. The sensor is by convention 
    // 2x2 units large and 1 unit away from the origin, which aids in making the math simpler.
    // create_prime returns a ray passing through the point at (x,y) 
    //      Note: all Ray direction vectors will have z = -1.0 since they go directly
    //            away from the camera sensor 
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        
        // due to our fov and aspect ratio adjustments, we assume that width > height
        // for our scene.
        assert!(scene.width > scene.height);
        
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();          // fov adjustment factor
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);    // aspect ratio adjustment factor

        // calculate the converted coordinates from our scene to the sensor coordinates
        // from (0..scene.width, 0..scene.height) -> (-1.0..1.0, -1.0..1.0)
        // the fov and aspect ratio adjustments are applied
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        // return our prime ray with origin (0,0,0) and the normalized direction vector 
        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                    x: sensor_x,
                    y: sensor_y,
                    z: -1.0,
                }
                .normalize(),
        }
    }
}

