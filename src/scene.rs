use crate::geometry::sphere::Sphere;
use crate::ray::Ray;
use crate::geometry::intersection::Intersection;
use crate::geometry::traits::Intersectable;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.spheres
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
