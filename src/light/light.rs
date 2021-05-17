use cgmath::Vector3;
use crate::color;

pub struct Light {
    pub direction: Vector3<f64>,
    pub color: color::Color,
    pub intensity: f32
}
