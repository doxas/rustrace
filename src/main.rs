
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

fn trace(x: u32, y: u32, w: u32, h: u32) -> Color {
    let aspect: f64 = w as f64 / h as f64;
    let s: f64 = (x as f64 * 2.0 - w as f64) / w as f64 * aspect;
    let t: f64 = (y as f64 * 2.0 - h as f64) / h as f64 * -1.0;
    let mut ray: Point3D = Point3D::new(s, t, -1.0);
    ray.normalize();
    Color {
        r: (ray.x.abs() * 255.0) as u8,
        g: (ray.y.abs() * 255.0) as u8,
        b: (ray.z.abs() * 255.0) as u8,
        a: 255
    }
}

fn main() {
    let width: u32 = 512;
    let height: u32 = 512;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // write color
        let dest: Color = trace(x, y, width, height);

        // return color
        *pixel = image::Rgba([dest.r, dest.g, dest.b, dest.a]);
    }
    img.save("./out/test.png").unwrap();
}


