
extern crate image;

mod point;

use point::point3d::Point3D;

struct Ray {
    position: Point3D,
    direction: Point3D
}

fn sphere(p: Point3D, size: f64) -> f64 {
    p.length() - size
}

fn map(p: Point3D) -> f64 {
    sphere(p, 2.0)
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

fn main() {
    let count = 8;
    let width = 512;
    let height = 512;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // normalize coordinate
        let px: f64 = (x as f64 * 2.0 - width as f64) / width as f64;
        let py: f64 = -(y as f64 * 2.0 - height as f64) / height as f64;

        // initialize variable
        let mut dest = vec![0, 0, 0, 255];
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
            dest[0] = (n.x.max(0.0) * 255.0) as isize;
            dest[1] = (n.y.max(0.0) * 255.0) as isize;
            dest[2] = (n.z.max(0.0) * 255.0) as isize;
        }

        // write color
        *pixel = image::Rgba([dest[0] as u8, dest[1] as u8, dest[2] as u8, dest[3] as u8]);
    }

    img.save("./out/test.png").unwrap();
}


