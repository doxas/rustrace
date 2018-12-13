
extern crate image;

mod point;

use point::point3d::Point3D;

struct Color {
    r: f64,
    g: f64,
    b: f64
}

struct Ray {
    hit: bool,
    position: Point3D,
    normal: Point3D,
    direction: Point3D,
    color: Color
}

struct Sphere {
    position: Point3D,
    radius: f64,
    color: Color
}

fn generate_ray(x: u32, y: u32, w: u32, h: u32) -> Ray {
    let aspect: f64 = w as f64 / h as f64;
    let s: f64 = (x as f64 * 2.0 - w as f64) / w as f64 * aspect;
    let t: f64 = (y as f64 * 2.0 - h as f64) / h as f64 * -1.0;
    let mut ray: Point3D = Point3D::new(s, t, -1.0);
    ray.normalize();

    Ray {
        hit: false,
        position: Point3D::new(0.0, 0.0, 5.0),
        normal: Point3D::new(0.0, 0.0, 0.0),
        direction: ray,
        color: Color {r: 0.0, g: 0.0, b: 0.0}
    }
}

fn intersect_sphere(ray: Ray, sphere: Sphere) -> Ray {
    let a: Point3D = ray.position - sphere.position;
    let b: f64 = a.dot(ray.direction);
    let c: f64 = a.dot(a) - (sphere.radius * sphere.radius);
    let d: f64 = b * b - c;
    let mut r: Ray = Ray {
        hit: false,
        position: Point3D::new(0.0, 0.0, 0.0),
        normal: Point3D::new(0.0, 0.0, 0.0),
        direction: Point3D::new(0.0, 0.0, 0.0),
        color: Color {r: 0.0, g: 0.0, b: 0.0}
    };
    if d > 0.0 {
        let t = -b - d.sqrt();
        if t > 0.0 {
            r.hit = true;
            r.position = ray.position + ray.direction * t;
            let mut n = r.position - sphere.position;
            n.normalize();
            r.normal = n;
            r.color = sphere.color;
        }
    }
    r
}

fn trace(ray: Ray) -> Color {
    // sphere
    let s: Sphere = Sphere {
        position: Point3D::new(0.0, 0.0, 0.0),
        radius: 1.0,
        color: Color {r: 1.0, g: 0.5, b: 0.1}
    };

    let mut r: Ray = intersect_sphere(ray, s);

    if r.hit == true {
        let mut light: Point3D = Point3D::new(1.0, 1.0, 0.5);
        light.normalize();
        let mut diff: f64 = r.normal.dot(light);
        diff = diff.min(1.0);
        diff = diff.max(0.1);
        r.color.r *= diff;
        r.color.g *= diff;
        r.color.b *= diff;
    }

    r.color
}

fn main() {
    let width: u32 = 512;
    let height: u32 = 512;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // ray
        let ray: Ray = generate_ray(x, y, width, height);

        // trace and intersects
        let color: Color = trace(ray);

        // write color
        let r: u8 = (color.r * 255.0) as u8;
        let g: u8 = (color.g * 255.0) as u8;
        let b: u8 = (color.b * 255.0) as u8;
        let a: u8 = 255;

        // return color
        *pixel = image::Rgba([r, g, b, a]);
    }

    img.save("./out/test.png").unwrap();
}


