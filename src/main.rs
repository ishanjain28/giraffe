extern crate x11cap;
extern crate image;

use image::{ImageBuffer, GenericImage};

fn main() {
    let c = x11cap::CaptureSource::Monitor(0);

    let mut d = x11cap::Capturer::new(c).unwrap();

    let f = d.capture_frame().unwrap();

    let f = f.as_slice();


    let mut img: image::RgbaImage = ImageBuffer::new(1920, 1080);


    for i in 0..img.height() {
        for j in 0..img.width() {

            let p = f[(i * img.width() + j) as usize];

            let q = image::Rgba { data: [p.r, p.g, p.b, 255] };

            img.put_pixel(j, i, q);
        }
    }

    img.save("t.png");;
}
