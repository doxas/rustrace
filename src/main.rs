
extern crate image;

mod point;

use point::point3d::Point3D;

struct Ray {
    origin: Point3D,
    position: Point3D,
    direction: Point3D
}

fn sphere(p: &Point3D, size: f64) -> f64 {
    p.length() - size
}

fn main() {
    let count = 8;
    let width = 512;
    let height = 512;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {

        // initial
        let mut distance :f64 = 0.0;
        let mut ray :Ray = Ray {
            origin:    Point3D::new(0.0, 0.0, 5.0),
            position:  Point3D::new(0.0, 0.0, 5.0),
            direction: Point3D::new(0.0, 0.0, -1.0)
        };
        let mut dest = vec![0, 0, 0, 255];

        let px :f64 = (x as f64 * 2.0 - width as f64) / width as f64;
        let py :f64 = (y as f64 * 2.0 - height as f64) / height as f64;

        ray.direction = Point3D::new(px, py, -1.0);

        for _ in 0..count {
            distance = sphere(&ray.position, 1.0);
            ray.position.x += ray.direction.x * distance;
            ray.position.y += ray.direction.y * distance;
            ray.position.z += ray.direction.z * distance;
        }

        if distance < 0.001 {
            dest[0] = 255;
            dest[1] = 255;
            dest[2] = 255;
        }

        *pixel = image::Rgba([dest[0] as u8, dest[1] as u8, dest[2] as u8, dest[3] as u8]);
    }

    img.save("./out/test.png").unwrap();
}


