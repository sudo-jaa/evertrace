use crate::ray;

pub trait Intersectable {
    fn intersect(&self, ray: &ray::Ray) -> Option<f64>;
}
