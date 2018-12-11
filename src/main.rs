
extern crate image;

fn main() {
    let width = 8;
    let height = 8;

    // construct new imagebuffer
    let mut img = image::ImageBuffer::new(width, height);

    // iterate over all pixels in the image.
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = x * (255 / width);
        let g = y * (255 / height);
        let b = 255;
        let a = 255;
        *pixel = image::Rgba([r as u8, g as u8, b as u8, a as u8]);
    }

    img.save("test.png").unwrap();
}

