extern crate gif;
extern crate x11;
extern crate x11cap;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

use std::fs::File;
use std::ffi;

mod capturer;
mod draw;
mod utils;

struct Captures<'a> {
    seq: i64,
    Frame: Option<gif::Frame<'a>>,
    Image: Vec<u8>,
}

fn main() {
    // setup logger
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();


    // Detect WS and load appropriate crates
    // detect window system
    let ws = utils::detect_ws();
    match ws {
        Ok(v) => {
            if v == utils::ws::X11 {
                info!("X11 WS detected");
            } else {

                info!("Wayland Detected");
            }
        }
        Err(e) => {
            error!("Could not identify window system");
        }
    }
    let s = ffi::CString::new(":0").unwrap();
    draw::draw_transparent_window(s.as_ptr());
}




fn start_capturing() {

    let c = x11cap::CaptureSource::Region {
        x: 200,
        y: 200,
        width: 1000,
        height: 800,
    };

    let mut d = x11cap::Capturer::new(c).unwrap();

    // let mut file = File::create("img.gif").unwrap();
    let color_map = &[0xFF, 0xFF, 0xFA];

    let geometry = d.get_geometry();

    //    let mut filef = gif::Encoder::new(
    //     file,
    //    geometry.width as u16,
    //   geometry.height as u16,
    //  color_map,
    // ).expect("Error in creating gif encoder");

    //    let (tx, rx): (Sender<Captures>, Receiver<Captures>) = mpsc::channel();

    let mut f = d.capture_frame().unwrap();
    let img = convert(f);
    //let mut c = Captures {
    //   seq: i,
    // Image: img,
    // Frame: None,
    //};

    //let mut frame = gif::Frame::from_rgb(
    //  geometry.width as u16,
    //geometry.height as u16,
    // c.Image.as_slice(),
    // );

    // frame.delay = 10;
    // c.Image = Vec::new();
    // c.Frame = Some(frame);

    // });

    //        println!("Capturing frame {}", i);

    //        filef.write_frame(&i.Frame.unwrap());
}
fn convert(u: x11cap::Image) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    for i in u.as_slice() {
        v.push(i.r);
        v.push(i.g);
        v.push(i.b);
    }
    v
}
