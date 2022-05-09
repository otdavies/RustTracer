use std::fs::File;
use std::io::prelude::*;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    let mut image = render_image();
    flip_image_vertically(&mut image);
    render_to_file("out.ppm", image);
}

fn render_image() -> Vec<u8> {
    let mut image: Vec<u8> = vec![0; IMAGE_HEIGHT * IMAGE_WIDTH * 3];
    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let i = to_index(x, y);
            image[i] = ((x as f64 / IMAGE_WIDTH as f64) * 255.) as u8;
            image[i + 1] = ((y as f64 / IMAGE_HEIGHT as f64) * 255.) as u8;
            image[i + 2] = (0.25 * 255.) as u8;
        }
    }
    image
}

fn flip_image_vertically(image: &mut [u8]) {
    for y in 0..IMAGE_HEIGHT / 2 {
        let top = y * IMAGE_WIDTH * 3;
        let bottom = (IMAGE_HEIGHT - y - 1) * IMAGE_WIDTH * 3;
        for x in 0..IMAGE_WIDTH * 3 {
            image.swap(top + x, bottom + x);
        }
    }
}

#[allow(dead_code)]
fn flip_image_horizontally(image: &mut [u8]) {
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH / 2 {
            let left = y * IMAGE_WIDTH * 3 + x * 3;
            let right = y * IMAGE_WIDTH * 3 + (IMAGE_WIDTH - x - 1) * 3;
            image.swap(left, right);
        }
    }
}

fn to_index(x: usize, y: usize) -> usize {
    (y * IMAGE_WIDTH + x) * 3
}

fn render_to_file(path: &str, image: Vec<u8>) {
    let mut image_file = File::create(path).unwrap();
    let header: &str = &format!("P3\n{} {}\n{}\n", IMAGE_HEIGHT, IMAGE_WIDTH, 255);
    let mut body: String = String::new();

    for height in 0..IMAGE_WIDTH {
        for width in 0..IMAGE_HEIGHT {
            let i = to_index(width, height);
            let red = image[i];
            let green = image[i + 1];
            let blue = image[i + 2];
            body.push_str(&format!("{} {} {}\n", red, green, blue));
        }
    }

    image_file.write_all(header.as_bytes()).unwrap();
    image_file.write_all(body.as_bytes()).unwrap();
}
