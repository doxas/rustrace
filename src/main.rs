
extern crate image;

struct Point {
    x: f64,
    y: f64,
    z: f64
}

struct Ray {
    origin: Point,
    position: Point,
    direction: Point
}

fn length3d(x: f64, y: f64, z :f64) -> f64 {
    let l :f64 = x * x + y * y + z * z;
    l.sqrt()
}

fn normalize3d(p: Point) -> Point {
    let l = length3d(p.x, p.y, p.z);
    Point {x: p.x / l, y: p.y / l, z: p.z / l}
}

fn sphere(p: &Point, size: f64) -> f64 {
    length3d(p.x, p.y, p.z) - size
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
            origin:    Point {x: 0.0, y: 0.0, z: 5.0},
            position:  Point {x: 0.0, y: 0.0, z: 5.0},
            direction: Point {x: 0.0, y: 0.0, z: -1.0}
        };
        let mut dest = vec![0, 0, 0, 255];

        let px :f64 = (x as f64 * 2.0 - width as f64) / width as f64;
        let py :f64 = (y as f64 * 2.0 - height as f64) / height as f64;

        ray.direction = normalize3d(Point {x: px, y: py, z: -1.0});

        for i in 0..count {
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

    img.save("test.png").unwrap();
}


