extern crate x11;
extern crate x11cap;

use std::sync::mpsc;
use std::{thread, time};
use std;

#[derive(Debug)]
pub struct Image {
    pub id: u64,
    pub Image: x11cap::Image,
    pub width: u16,
    pub height: u16,
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
    // useless right now
    //    let q = Image { id: 0, Image: q };

    println!("Recording frames");
    //  tx.send(q);

    thread::sleep(time::Duration::from_millis(10));



    return true;

}
