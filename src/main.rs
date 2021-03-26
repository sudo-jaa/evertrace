use std::path::Path;

use image::{DynamicImage, GenericImage, Pixel, Rgba};
use crate::geometry::traits::Intersectable;
use crate::geometry::sphere::Sphere;
use crate::geometry::point::Point;

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

            if scene.sphere.intersect(&ray).is_some() {
                let col: Rgba<u8> = match &scene.sphere.color {
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
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: geometry::sphere::Sphere {
            center: geometry::point::Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: color::Color::GREEN
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}


fn main() {
    println!("raytrace!");
    let scene = scene::Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: color::Color::RGB(213, 105, 76)
        },
    };

    let img: DynamicImage = render(&scene);
    let saved = img.save( Path::new("./bar.png"));
    println!("done raytracing!! {:?}", saved);
}
