//! Sierpinski triangle generator

extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;

struct Point {
    x: u32,
    y: u32,
}

const WIDTH: u32 = 500;
const HEIGHT: u32 = 400;
const ITERATIONS: u32 = 200_000;

fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([240u8])
        } else {
            image::Luma([20u8])
        }
    });

    // Define our 3 triangle points
    let pts: [Point; 3] = [
        Point {x: WIDTH / 2, y: 0},
        Point {x: 0, y: HEIGHT},
        Point {x: WIDTH, y: HEIGHT},
    ];

    let mut p = Point {x: 0, y: 0};
    let pixel = img[(0, 0)];

    let mut num: usize;
    for _ in 0..ITERATIONS {
        num = rand::thread_rng().gen_range(0, 3);
        p.x = (p.x + pts[num].x) / 2;
        p.y = (p.y + pts[num].y) / 2;
        img.put_pixel(p.x, p.y, pixel);
    }

    let ref mut fout = File::create("triangle.png").unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);
}

