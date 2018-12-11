
extern crate image;

struct Point {
    x: f64,
    y: f64,
    z: f64
}

struct Ray {
    origin: Point,
    direction: Point
}

fn length2d(x: f64, y: f64) -> f64 {
    let l :f64 = x * x + y * y;
    l.sqrt()
}

fn main() {
    let count = 8;
    let width = 512;
    let height = 512;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {

        // initial
        // let distance :f64 = 0.0;
        // let mut ray :Ray = Ray {
        //     origin: Point {x: 0.0, y: 0.0, z: -5.0},
        //     direction: Point {x: 0.0, y: 0.0, z: -5.0}
        // };
        let mut dest = vec![0, 0, 0, 255];

        let px :f64 = (x as f64 * 2.0 - width as f64) / width as f64;
        let py :f64 = (y as f64 * 2.0 - height as f64) / height as f64;
        let pl :f64 = length2d(px, py);
        let po :f64 = if pl == 0.0 { 1.0 } else { 0.1 / pl };
        po.min(1.0);
        po.max(0.0);
        dest[0] = (po * 255.0) as u8;
        dest[1] = (po * 255.0) as u8;
        dest[2] = (po * 255.0) as u8;

        *pixel = image::Rgba([dest[0] as u8, dest[1] as u8, dest[2] as u8, dest[3] as u8]);
    }

    img.save("test.png").unwrap();
}


