
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
    distance: f64,
    origin: Point3D,
    position: Point3D,
    normal: Point3D,
    direction: Point3D,
    color: Color
}

struct Geometry {
    geometry: String,
    position: Point3D,
    normal: Point3D,
    radius: f64,
    color: Color
}

fn generate_ray(x: u32, y: u32, w: u32, h: u32, origin: Point3D) -> Ray {
    let aspect: f64 = w as f64 / h as f64;
    let s: f64 = (x as f64 * 2.0 - w as f64) / w as f64 * aspect;
    let t: f64 = (y as f64 * 2.0 - h as f64) / h as f64 * -1.0;
    let mut ray: Point3D = Point3D::new(s, t, -1.0);
    ray.normalize();

    Ray {
        hit: false,
        distance: 1.0 / 0.0,
        origin: origin,
        position: origin,
        normal: Point3D::new(0.0, 0.0, 0.0),
        direction: ray,
        color: Color {r: 0.0, g: 0.0, b: 0.0}
    }
}

fn intersect_plane(ray: &mut Ray, plane: Geometry) {
    let d: f64 = -plane.position.dot(plane.normal);
    let v: f64 = ray.direction.dot(plane.normal);
    let t: f64 = -(ray.origin.dot(plane.normal) + d) / v;
    if t > 0.0 {
        if t < ray.distance {
            ray.hit = true;
            ray.distance = t;
            ray.position = ray.origin + ray.direction * t;
            ray.normal = plane.normal;
            let mut m: f64 = ray.position.x % 2.0;
            let mut n: f64 = ray.position.z % 2.0;
            if m < 0.0 {m += 2.0;}
            if n < 0.0 {n += 2.0;}
            let mut c: f64 = 1.0;
            if (m > 1.0 && n > 1.0) || (m < 1.0 && n < 1.0) {c *= 0.5;}
            ray.color.r = plane.color.r * c;
            ray.color.g = plane.color.g * c;
            ray.color.b = plane.color.b * c;
        }
    }
}

fn intersect_sphere(ray: &mut Ray, sphere: Geometry) {
    let a: Point3D = ray.origin - sphere.position;
    let b: f64 = a.dot(ray.direction);
    let c: f64 = a.dot(a) - (sphere.radius * sphere.radius);
    let d: f64 = b * b - c;
    if d > 0.0 {
        let t = -b - d.sqrt();
        if t > 0.0 {
            if t < ray.distance {
                ray.hit = true;
                ray.distance = t;
                ray.position = ray.origin + ray.direction * t;
                let mut n = ray.position - sphere.position;
                n.normalize();
                ray.normal = n;
                ray.color.r = sphere.color.r;
                ray.color.g = sphere.color.g;
                ray.color.b = sphere.color.b;
            }
        }
    }
}

fn trace(ray: &mut Ray) {
    // plane
    let p: Geometry = Geometry {
        geometry: "plane".to_string(),
        position: Point3D::new(0.0, -1.0, 0.0),
        normal: Point3D::new(0.0, 1.0, 0.0),
        radius: 0.0,
        color: Color {r: 0.5, g: 0.5, b: 0.55}
    };

    // sphere
    let s: Geometry = Geometry {
        geometry: "sphere".to_string(),
        position: Point3D::new(0.0, 0.0, 0.0),
        normal: Point3D::new(0.0, 0.0, 0.0),
        radius: 1.0,
        color: Color {r: 1.0, g: 0.5, b: 0.1}
    };
    let t: Geometry = Geometry {
        geometry: "sphere".to_string(),
        position: Point3D::new(2.0, -0.25, 0.0),
        normal: Point3D::new(0.0, 0.0, 0.0),
        radius: 0.75,
        color: Color {r: 0.1, g: 0.5, b: 1.0}
    };

    // intersects for geometry in vec
    let obj = vec![p, s, t];
    for o in obj {
        if o.geometry == "plane" {
            intersect_plane(ray, o);
        } else if o.geometry == "sphere" {
            intersect_sphere(ray, o);
        }
    }

    // hit check
    if ray.hit == true {
        let mut light: Point3D = Point3D::new(1.0, 1.0, 0.5);
        light.normalize();
        let mut diff: f64 = ray.normal.dot(light);
        diff = diff.min(1.0).max(0.1);
        ray.color.r *= diff;
        ray.color.g *= diff;
        ray.color.b *= diff;
    }
}

fn main() {
    // constant
    let width: u32  = 512;
    let height: u32 = 512;

    // image
    let mut img = image::ImageBuffer::new(width, height);

    // write
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // ray
        let mut ray: Ray = generate_ray(x, y, width, height, Point3D {x: 0.0, y: 0.0, z: 5.0});

        // trace and intersects
        trace(&mut ray);

        // write color
        let r: u8 = (ray.color.r * 255.0) as u8;
        let g: u8 = (ray.color.g * 255.0) as u8;
        let b: u8 = (ray.color.b * 255.0) as u8;
        let a: u8 = 255;

        // put color
        *pixel = image::Rgba([r, g, b, a]);
    }
    img.save("./out/test.png").unwrap();
}


