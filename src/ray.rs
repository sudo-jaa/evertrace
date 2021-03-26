use cgmath::{InnerSpace};
use crate::geometry::point::Point;
use crate::scene::Scene;

pub struct Ray {
    pub origin: Point,
    pub direction: cgmath::Vector3<f64>,
}

// pub struct Camera {
//     aspect_ratio: f64,
//     fov_adjustment: f64,
// }
//
// impl Camera {
//    pub fn create_prime(&self, _: u32, _: u32, _: &Scene) {
//        println!("Not implemented!")
//    }
// }

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);

        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);

        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: cgmath::Vector3{
                x: sensor_x,
                y: sensor_y,
                z: -1.0
            }.normalize()
        }
    }
}
