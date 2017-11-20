extern crate x11cap;
extern crate gif;


use std::fs::File;
use std::{thread, time};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod capturer;

struct Captures<'a> {
    seq: i64,
    Frame: Option<gif::Frame<'a>>,
    Image: Vec<u8>,
}

fn main() {
    let c = x11cap::CaptureSource::Region {
        x: 200,
        y: 200,
        width: 1000,
        height: 800,
    };

    let mut d = x11cap::Capturer::new(c).unwrap();

    let mut file = File::create("img.gif").unwrap();
    let color_map = &[0xFF, 0xFF, 0xFA];

    let geometry = d.get_geometry();

    let mut filef = gif::Encoder::new(
        file,
        geometry.width as u16,
        geometry.height as u16,
        color_map,
    ).expect("Error in creating gif encoder");

    //    let (tx, rx): (Sender<Captures>, Receiver<Captures>) = mpsc::channel();
    let mut caps = Vec::new();

    let mut i = 1;
    loop {
        let mut f = d.capture_frame().unwrap();
        let img = convert(f);
        let mut c = Captures {
            seq: i,
            Image: img,
            Frame: None,
        };

        thread::spawn(move || {
            let mut frame = gif::Frame::from_rgb(
                geometry.width as u16,
                geometry.height as u16,
                c.Image.as_slice(),
            );

            frame.delay = 10;
            c.Image = Vec::new();
            c.Frame = Some(frame);


            caps.insert(c.seq as usize, c);
        });

        if i > 300 {
            break;
        }
        thread::sleep(time::Duration::from_millis(40));
        println!("Capturing frame {}", i);
        i += 1;
    }

    for i in caps {
        filef.write_frame(&i.Frame.unwrap());
    }
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
