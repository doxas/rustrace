
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

struct Plane {
    position: Point3D,
    normal: Point3D,
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

fn intersect_plane(ray: &Ray, plane: Plane) -> Ray {
    let d: f64 = -plane.position.dot(plane.normal);
    let v: f64 = ray.direction.dot(plane.normal);
    let t: f64 = -(ray.position.dot(plane.normal) + d) / v;
    let mut r: Ray = Ray {
        hit: false,
        position: Point3D::new(0.0, 0.0, 0.0),
        normal: Point3D::new(0.0, 0.0, 0.0),
        direction: Point3D::new(0.0, 0.0, 0.0),
        color: Color {r: 0.0, g: 0.0, b: 0.0}
    };
    if t > 0.0 {
        r.hit = true;
        r.position = ray.position + (ray.direction * t);
        r.normal = plane.normal;
        let mut m: f64 = r.position.x % 2.0;
        let mut n: f64 = r.position.z % 2.0;
        if m < 0.0 {m += 2.0;}
        if n < 0.0 {n += 2.0;}
        let mut c: f64 = 1.0;
        if (m > 1.0 && n > 1.0) || (m < 1.0 && n < 1.0) {c *= 0.5;}
        r.color.r = plane.color.r * c;
        r.color.g = plane.color.g * c;
        r.color.b = plane.color.b * c;
    }
    r
}

fn intersect_sphere(ray: &Ray, sphere: Sphere) -> Ray {
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
    // plane
    let p: Plane = Plane {
        position: Point3D::new(0.0, -1.0, 0.0),
        normal: Point3D::new(0.0, 1.0, 0.0),
        color: Color {r: 0.7, g: 0.7, b: 0.7}
    };

    // sphere
    let s: Sphere = Sphere {
        position: Point3D::new(0.0, 0.0, 0.0),
        radius: 1.0,
        color: Color {r: 1.0, g: 0.5, b: 0.1}
    };

    // intersects
    let rp: Ray = intersect_plane(&ray, p);
    let rs: Ray = intersect_sphere(&ray, s);
    let mut r: Ray = Ray {
        hit: false,
        position: Point3D::new(0.0, 0.0, 0.0),
        normal: Point3D::new(0.0, 0.0, 0.0),
        direction: Point3D::new(0.0, 0.0, 0.0),
        color: Color {r: 0.0, g: 0.0, b: 0.0}
    };

    // hit check
    if rp.hit == true && rs.hit == true {
        let lenp: Point3D = rp.position - ray.position;
        let lens: Point3D = rs.position - ray.position;
        r = if lenp.length() < lens.length() { rp } else { rs };
    } else if rp.hit == true {
        r = rp;
    } else if rs.hit == true {
        r = rs;
    }
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
    // constant
    let width: u32  = 512;
    let height: u32 = 512;

    // image
    let mut img = image::ImageBuffer::new(width, height);

    // write
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

        // put color
        *pixel = image::Rgba([r, g, b, a]);
    }
    img.save("./out/test.png").unwrap();
}


