use crate::geometry::sphere::Sphere;
use crate::geometry::plane::Plane;
use crate::geometry::traits::Intersectable;
use crate::ray::Ray;
use crate::color::Color;
use cgmath::Vector3;
use crate::geometry::point::Point;

pub enum Element {
    Sphere(Sphere),
    Plane(Plane)
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color
        }
    }
    pub fn surface_normal(&self, hit_point: &Point) -> Vector3<f64> {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point)
        }
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match &self {
            Element::Sphere(ref s) => {
                s.intersect(ray)
            }
            Element::Plane(ref p) => {
                p.intersect(ray)
            }
        }
    }
}
