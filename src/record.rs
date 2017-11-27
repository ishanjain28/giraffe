extern crate x11;
extern crate x11cap;

use x11::xlib;
use std::sync::mpsc;
use std::thread;
use std;

#[derive(Debug)]
pub struct Image {
    pub Image: x11cap::Image,
}

unsafe impl std::marker::Send for Image {}

pub struct Recorder {
    recording: bool,
}


impl Recorder {
    pub fn new() -> Recorder {
        Recorder { recording: false }
    }


    pub fn start_recording(
        &mut self,
        tx: mpsc::Sender<Image>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) {

    }

    pub fn stop_recording(&mut self) {}
}
pub fn record(capture_region: x11cap::CaptureSource, tx: mpsc::Sender<Image>) -> bool {
    let mut d = match x11cap::Capturer::new(capture_region) {
        Ok(v) => v,
        Err(e) => {
            error!("{:?}", e);
            return false;
        }
    };

    //let color_map = &[0xFF, 0xFF, 0xFF];

    let mut q = match d.capture_frame() {
        Ok(v) => v, 
        Err(e) => {
            error!("{:?}", e);

            return false;
        }

    };
    let q = Image { Image: q };

    println!("Recording frames");
    tx.send(q);

    thread::sleep_ms(10);


    return true;

}
