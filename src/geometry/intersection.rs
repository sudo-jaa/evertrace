use crate::geometry::sphere::Sphere;
use crate::geometry::element::Element;

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Element
}

impl <'a> Intersection<'a> {
    pub fn new(distance: f64, object: &Element) -> Intersection {
        Intersection{distance, object}
    }
}

