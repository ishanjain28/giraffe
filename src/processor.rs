extern crate x11cap;
extern crate gif;
extern crate rayon;

use std::sync::{mpsc, Arc, Mutex};
use std::fs::File;
use std::thread;
use rayon::ThreadPool;

use record;

fn work(v: record::Image, tx: Arc<Mutex<mpsc::Sender<record::gifFrame>>>) {

    let bgr8_slice = v.Image.as_slice();

    let mut frame = gif::Frame::from_rgb(v.width, v.height, convert(bgr8_slice).as_slice());

    let tx = tx.lock().unwrap();


    tx.send(record::gifFrame {
        frame: frame,
        id: v.id,
    });

}

pub fn process(rx: mpsc::Receiver<record::Image>, f: &mut File) {

    let color_map = &[0xFF, 0xFF, 0xFF];

    let (res_tx, res_rx): (mpsc::Sender<record::gifFrame>, mpsc::Receiver<record::gifFrame>) =
        mpsc::channel();

    let receiver = Arc::new(Mutex::new(rx));
    let res_tx = Arc::new(Mutex::new(res_tx));

    let pool = ThreadPool::new(rayon::Configuration::new().num_threads(8)).unwrap();

    let mut filef = gif::Encoder::new(f, 1920, 1080, color_map).expect("Failed to create encoder");

    pool.scope(|s| {
        loop {
            let rx = receiver.lock().unwrap();

            match rx.recv() {
                Ok(v) => {

                    // Free lock so other threads can use it
                    drop(rx);
                    let res_tx = Arc::clone(&res_tx);

                    s.spawn(|_| work(v, res_tx));

                }
                Err(e) => {

                    println!("{:?}", e);

                    break;
                }
            }
        }
    });

    let mut results = Vec::new();

    loop {
        match res_rx.recv() {
            Ok(v) => {
                results.push(v);
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }

    for i in results {

        println!("Writing frame {} ", i.id);

        filef.write_frame(&i.frame);
    }
}

fn convert(s: &[x11cap::Bgr8]) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(s.len());

    for i in s {
        v.push(i.r);
        v.push(i.g);
        v.push(i.b);
    }
    v
}
