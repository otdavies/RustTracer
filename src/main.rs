use std::fmt::Write;
use std::fs::File;
mod ray;
mod vec3;
use ray::Ray;
use vec3::Vec3;

const IMAGE_WIDTH: usize = 512;

fn ray_color(r: &Ray) -> Vec3 {
    // Red sphere
    let center = Vec3::new(0.0, 0.0, -1.0);
    let hit = hit_sphere(&center, 0.5, r);
    if hit > 0.0 {
        let n: Vec3 = (r.at(hit) - center).normalized();
        return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0).scalar(0.5);
    }

    // Sky color
    let unit_dir = r.direction.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0).scalar(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scalar(t)
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}

fn main() {
    // Image properties
    let aspect_ratio = 16.0 / 9.0;
    let image_width = IMAGE_WIDTH;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Create the camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal.scalar(0.5) - vertical.scalar(0.5) - Vec3::new(0.0, 0.0, focal_length);

    // Generate render
    let mut image = render_image(
        image_width,
        image_height,
        &origin,
        &horizontal,
        &vertical,
        &lower_left_corner,
    );
    flip_image_vertically(&mut image, image_width, image_height);
    render_to_file("out.ppm", image, image_width, image_height);
}

fn render_image(
    width: usize,
    height: usize,
    origin: &Vec3,
    horizontal: &Vec3,
    vertical: &Vec3,
    ll_corner: &Vec3,
) -> Vec<u8> {
    let mut image: Vec<u8> = vec![0; height * width * 3];
    for x in 0..width {
        for y in 0..height {
            let i = to_index(x, y);
            let u = x as f64 / (width - 1) as f64;
            let v = y as f64 / (height - 1) as f64;
            let r = Ray::new(
                *origin,
                *ll_corner + horizontal.scalar(u) + vertical.scalar(v) - *origin,
            );
            let color = ray_color(&r);
            image[i] = (color.x * 255.) as u8;
            image[i + 1] = (color.y * 255.) as u8;
            image[i + 2] = (color.z * 255.) as u8;
        }
    }
    image
}

fn flip_image_vertically(image: &mut [u8], width: usize, height: usize) {
    for y in 0..height / 2 {
        let top = y * width * 3;
        let bottom = (height - y - 1) * width * 3;
        for x in 0..width * 3 {
            image.swap(top + x, bottom + x);
        }
    }
}

fn to_index(x: usize, y: usize) -> usize {
    (y * IMAGE_WIDTH + x) * 3
}

fn render_to_file(path: &str, image: Vec<u8>, width: usize, height: usize) {
    let mut image_file = File::create(path).unwrap();
    let header: &str = &format!("P3\n{} {}\n{}\n", width, height, 255);
    let mut body: String = String::new();

    for height in 0..height {
        for width in 0..width {
            let i = to_index(width, height);
            let red = image[i];
            let green = image[i + 1];
            let blue = image[i + 2];
            writeln!(&mut body, "{} {} {}", red, green, blue);
        }
    }

    std::io::Write::write_all(&mut image_file, header.as_bytes()).unwrap();
    std::io::Write::write_all(&mut image_file, body.as_bytes()).unwrap();
}
