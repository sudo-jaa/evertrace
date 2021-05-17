use std::path::Path;

use image::{DynamicImage, GenericImage, Pixel, Rgba, GenericImageView};
use crate::geometry::traits::Intersectable;
use crate::geometry::sphere::Sphere;
use crate::geometry::point::Point;
use crate::geometry::element::Element;
use crate::geometry::plane::Plane;
use crate::light::light::Light;
use cgmath::{Vector3, InnerSpace};

mod light;
mod color;
mod ray;
mod scene;
mod geometry;

pub fn render(scene: &scene::Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = ray::Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            if intersection.is_some() {
                let hit_point = ray.origin.add(ray.direction * intersection.unwrap().distance);
                let surface_normal = intersection.unwrap().object.surface_normal(&hit_point);
                let direction_to_light = -scene.light.direction;
                let light_power = (surface_normal.dot(direction_to_light) as f32) * scene.light.intensity;
                let col: Rgba<u8> = match intersection.unwrap().object.color() {
                    color::Color::RED => Rgba{0: [255, 0, 0, 255]},
                    color::Color::BLUE => Rgba{0: [0, 0, 255, 1]},
                    color::Color::GREEN => Rgba{0: [0, 255, 0, 1]},
                    color::Color::RGB(r, g, b) => Rgba{0: [*r, *g, *b, 1]}
                };
                image.put_pixel(x, y, col.to_rgba());
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

#[test]
fn test_can_render_scene() {
    let scene = scene::Scene {
        light: Light{direction: Vector3::new(0.0, 0.0, 0.0), color: color::Color::RGB(255, 255, 255), intensity: 12.0},
        width: 800,
        height: 600,
        fov: 90.0,
        elements: vec![Element::Sphere(Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: color::Color::GREEN
        })],
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

fn main() {
    println!("raytrace!");
    let scene = scene::Scene {
        light: Light{direction: Vector3::new(0.0, 0.0, 0.0), color: color::Color::RGB(255, 255, 255), intensity: 12.0},
        width: 1800,
        height: 1200,
        fov: 90.0,
        elements: vec![Sphere::new(color::Color::random(), 1.0, 0.0, 0.0, -5.0),
                       Sphere::new(color::Color::random(), 0.75, 0.5, 0.0, -4.0),
                       Sphere::new(color::Color::random(), 1.2, 1.0, 1.0, -4.25),
                       Sphere::new(color::Color::random(), 11.0, -3.0, 0.0, -20.0),
        Plane::new()]
    };

    let img: DynamicImage = render(&scene);
    let saved = img.save( Path::new("./bar.png"));
    println!("done raytracing!! {:?}", saved);
}
