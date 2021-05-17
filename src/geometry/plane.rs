use crate::geometry::point::Point;
use cgmath::{Vector3, InnerSpace};
use crate::color::Color;
use crate::geometry::traits::Intersectable;
use crate::ray::Ray;
use crate::geometry::element::Element;

pub struct Plane {
    pub p0: Point,
    pub normal: Vector3<f64>,
    pub color: Color
}

impl Plane {
    pub fn new() -> Element {
        Element::Plane(Plane {
            p0: Point {
                x: 0.0,
                y: -5.0,
                z: 0.0
            },
            normal: Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0
            },
            color: Color::RGB(116, 141, 181)
        }
        )
    }

    pub fn surface_normal(&self, _: &Point) -> Vector3<f64> {
        -self.normal
    }
}

// // assuming vectors are all normalized
// float denom = dotProduct(n, l);
// if (denom > 1e-6) {
// Vec3f p0l0 = p0 - l0;
// t = dotProduct(p0l0, n) / denom;
// return (t >= 0);
// }
//
// return false;

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.p0.subtract(&ray.origin);
            let distance = v.dot(*normal) / denom;
            if distance >= 0.0 {
                return Some(distance)
            }
        }
        None
    }
}
