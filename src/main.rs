
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

fn generate_ray(x: u32, y: u32, w: u32, h: u32) -> Ray {
    let aspect: f64 = w as f64 / h as f64;
    let s: f64 = (x as f64 * 2.0 - w as f64) / w as f64 * aspect;
    let t: f64 = (y as f64 * 2.0 - h as f64) / h as f64 * -1.0;
    let mut ray: Point3D = Point3D::new(s, t, -1.0);
    ray.normalize();

    Ray {
        position: Point3D::new(0.0, 0.0, 5.0),
        direction: ray
    }
}

fn main() {
    let width: u32 = 512;
    let height: u32 = 512;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // ray
        let ray: Ray = generate_ray(x, y, width, height);

        // write color
        let dest: Color = Color {
            r: (ray.direction.x.abs() * 255.0) as u8,
            g: (ray.direction.y.abs() * 255.0) as u8,
            b: (ray.direction.z.abs() * 255.0) as u8,
            a: 255
        };

        // return color
        *pixel = image::Rgba([dest.r, dest.g, dest.b, dest.a]);
    }

    img.save("./out/test.png").unwrap();
}


