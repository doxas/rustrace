
extern crate image;

mod point;

use point::point3d::Point3D;

struct Ray {
    position: Point3D,
    direction: Point3D
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

fn sphere(p: Point3D, size: f64) -> f64 {
    p.length() - size
}

fn trans(p: Point3D, modulo: f64) -> Point3D {
    let half: f64 = modulo / 2.0;
    let neg_x: f64 = p.x.signum();
    let neg_y: f64 = p.y.signum();
    let neg_z: f64 = p.z.signum();
    Point3D {
        x: (p.x % modulo) * neg_x - half,
        y: (p.y % modulo) * neg_y - half,
        z: (p.z % modulo) * neg_z - half
    }
}

fn map(p: Point3D) -> f64 {
    sphere(trans(p, 4.0), 1.0)
}

fn normal(p: Point3D) -> Point3D {
    let eps = 0.001;
    let mut n: Point3D = Point3D::new(
        map(p + Point3D::new(eps, 0.0, 0.0)) - map(p - Point3D::new(eps, 0.0, 0.0)),
        map(p + Point3D::new(0.0, eps, 0.0)) - map(p - Point3D::new(0.0, eps, 0.0)),
        map(p + Point3D::new(0.0, 0.0, eps)) - map(p - Point3D::new(0.0, 0.0, eps))
    );
    n.normalize();
    n
}

fn march(x: u32, y: u32, w: u32, h: u32, count: u32) -> Color {
    // normalize coordinate
    let px: f64 = (x as f64 * 2.0 - w as f64) / w as f64;
    let py: f64 = -(y as f64 * 2.0 - h as f64) / h as f64;

    // initialize variable
    let mut dest: Color = Color {r: 0, g: 0, b: 0, a: 0};
    let mut distance: f64 = 0.0;
    let mut ray: Ray = Ray {
        position:  Point3D::new(0.0, 0.0, 5.0),
        direction: Point3D::new(px, py, -1.0)
    };
    ray.direction.normalize();

    // ray marching
    for _ in 0..count {
        distance = map(ray.position);
        ray.position.x += ray.direction.x * distance;
        ray.position.y += ray.direction.y * distance;
        ray.position.z += ray.direction.z * distance;
    }

    // hit check
    if distance < 0.001 {
        let n: Point3D = normal(ray.position);
        let mut l: Point3D = Point3D::new(1.0, 1.0, 0.5);
        l.normalize();
        // diffuse
        let diff: f64 = l.dot(n).max(0.1);

        dest.r = (255.0 * diff) as u8;
        dest.g = (255.0 * diff) as u8;
        dest.b = (255.0 * diff) as u8;
    }
    dest.a = 255;
    dest
}

fn main() {
    let count: u32 = 32;
    let width: u32 = 512;
    let height: u32 = 512;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // write color
        let dest: Color = march(x, y, width, height, count);

        // return color
        *pixel = image::Rgba([dest.r, dest.g, dest.b, dest.a]);
    }

    img.save("./out/test.png").unwrap();
}


