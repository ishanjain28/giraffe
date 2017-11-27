extern crate gif;
extern crate x11;
extern crate x11cap;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

use std::fs::File;
use std::ffi;
use x11::xlib;
use std::os::raw;
use std::sync::{Arc, Mutex, mpsc};
use std::{thread, time};

mod selector;
mod utils;
mod record;
mod processor;

struct Captures<'a> {
    seq: i64,
    Frame: Option<gif::Frame<'a>>,
    Image: Vec<u8>,
}

pub struct Giraffe {
    pub display: *mut xlib::Display,
    pub screen_count: i32,
}

impl Giraffe {
    fn new(id: *const raw::c_char) -> Giraffe {

        let dpy;
        let s_count;

        unsafe {
            dpy = xlib::XOpenDisplay(id);
            s_count = xlib::XScreenCount(dpy);
        }

        Giraffe {
            display: dpy,
            screen_count: s_count,
        }
    }
    fn get_root_window(&self) -> xlib::Window {
        unsafe { xlib::XDefaultRootWindow(self.display) }
    }
}

impl Drop for Giraffe {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}

fn init_logger() {
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
}

fn main() {

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
    //    draw::draw_transparent_window(s.as_ptr());

    // Create a Giraffe
    let giraffe = Giraffe::new(s.as_ptr());

    let (tx, rx): (mpsc::Sender<record::Image>, mpsc::Receiver<record::Image>) = mpsc::channel();

    let mut recording = Arc::new(Mutex::new(true));

    let mut recording_clone = Arc::clone(&recording);

    // The thread which captures selected area at 45fps and pushes images to a processing queue
    thread::spawn(move || {

        let capture_region = x11cap::CaptureSource::Region {
            x: 500,
            y: 40,
            width: 1000,
            height: 1000,
        };

        let mut d = match x11cap::Capturer::new(capture_region) {
            Ok(v) => v,
            Err(e) => {
                error!("{:?}", e);
                return false;
            }
        };

        let is_recording = *(recording_clone.lock().unwrap());

        let mut count = 0;

        while is_recording {

            let mut q = match d.capture_frame() {
                Ok(v) => v,
                Err(e) => {
                    return false;
                }
            };

            let q = record::Image { Image: q };

            println!("Recording Frame {} ", count);

            tx.send(q);
            count += 1;

            thread::sleep_ms(200);

        }
        return false;
    });

    // start processor
    processor::process(rx);

    thread::sleep_ms(10000);

    let mut d = *(recording.lock().unwrap());
    //    selectora):draw_selection_window(&giraffe);
    d = false;

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
    //    let img = convert(f);
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
