use cgmath::Vector3;

pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn subtract(&self, point: &Point) -> Vector3<f64> {
        Vector3{x: self.x - point.x, y: self.y - point.y, z: self.z - point.z}
    }

    pub fn add(&self, point: &Point) -> Vector3<f64> {
        Vector3{x: self.x + point.x, y: self.y + point.y, z: self.z + point.z}
    }
}
