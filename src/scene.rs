use crate::geometry::sphere::Sphere;
use crate::ray::Ray;
use crate::geometry::intersection::Intersection;
use crate::geometry::traits::Intersectable;
use crate::geometry::element::Element;
use crate::light::light::Light;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub light: Light,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
