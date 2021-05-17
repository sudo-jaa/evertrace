use cgmath::{InnerSpace, Vector3};

use crate::{color, ray};
use crate::geometry::point;
use crate::geometry::traits::Intersectable;
use crate::geometry::element::Element;
use crate::color::Color;
use crate::geometry::point::Point;

pub struct Sphere {
    pub center: point::Point,
    pub radius: f64,
    pub color: color::Color
}

struct Position {
    x: f64,
    y: f64,
    z: f64
}

impl Sphere {
    pub fn new(color: Color, radius: f64, x: f64, y: f64, z: f64) -> Element {
        Element::Sphere(Sphere{
            center: Point {
                x,
                y,
                z
            },
            radius,
            color
        })
    }

    pub fn surface_normal(&self, hit_point: &Point) -> Vector3<f64> {
        hit_point.subtract(&self.center).normalize()
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &ray::Ray) -> Option<f64> {
        let l: Vector3<f64> = self.center.subtract(&ray.origin);
        let adj2 = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj2 * adj2);

        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            return None;
        }

        let thickness = (radius2 - d2).sqrt();
        let t0 = adj2 - thickness;
        let t1 = adj2 + thickness;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 {t0} else {t1};
        Some(distance)
    }
}
