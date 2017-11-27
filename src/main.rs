extern crate gif;
extern crate x11;
extern crate x11cap;
extern crate rayon;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

use std::fs::File;
use std::sync::{Arc, RwLock, mpsc};
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

    //    draw::draw_transparent_window(s.as_ptr());

    // Channel that transports shot images from a thread to image processing threads
    let (tx, rx): (mpsc::Sender<record::Image>, mpsc::Receiver<record::Image>) = mpsc::channel();

    let mut is_rec = Arc::new(RwLock::new(true));

    let mut is_rec_clone = is_rec.clone();

    thread::spawn(move || {

        thread::sleep(time::Duration::from_millis(20000));
        //    selectora):draw_selection_window(&giraffe);

        let mut d = is_rec.write().unwrap();

        *d = false;

    });

    // The thread which captures selected area and pushes images to a processing queue
    thread::spawn(move || {

        let capture_region = x11cap::CaptureSource::Region {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };

        let mut d = match x11cap::Capturer::new(capture_region) {
            Ok(v) => v,
            Err(e) => {
                error!("{:?}", e);
                return false;
            }
        };
        let mut is_recording = is_rec_clone.read().unwrap();
        let mut is_rec = *is_recording;
        drop(is_recording);

        let mut count = 0;
        while is_rec {
            is_recording = is_rec_clone.read().unwrap();

            is_rec = *is_recording;

            drop(is_recording);

            let mut q = match d.capture_frame() {
                Ok(v) => v,
                Err(e) => {
                    return false;
                }
            };
            let dimensions = q.get_dimensions();

            let q = record::Image {
                id: count,
                Image: q,
                width: dimensions.0 as u16,
                height: dimensions.1 as u16,
            };

            println!("Recording Frame {} ", count);

            //            println!("IS_RECORDING? {}", is_rec);

            tx.send(q);
            count += 1;

            thread::sleep(time::Duration::from_millis(125));

        }

        println!("STOPPED RECORDING");

        return false;
    });

    let mut fs = match File::create("img.gif") {
        Ok(v) => v,
        Err(e) => {

            println!("{}", e);
            return;
        }
    };

    println!("Starting Processor");

    // start processor
    processor::process(rx, &mut fs);
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
