use crate::geometry::sphere::Sphere;

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Sphere
}

impl <'a> Intersection<'a> {
    pub fn new(distance: f64, object: &Sphere) -> Intersection {
        Intersection{distance, object}
    }
}

