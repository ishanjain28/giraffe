extern crate x11cap;
extern crate gif;

use std::sync::{mpsc, Arc, Mutex};
use std::fs::File;
use std::thread;

use record;

#[derive(Debug)]
struct ThreadPool {
    workers: Vec<Worker>,
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    fn new(u: usize, rx: mpsc::Receiver<record::Image>) -> ThreadPool {
        let mut workers = Vec::with_capacity(u);
        let receiver = Arc::new(Mutex::new(rx));

        for id in 0..u {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers }
    }
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<record::Image>>>) -> Worker {


        let t = thread::spawn(move || loop {
            let rx = rx.lock().unwrap();

            match rx.recv() {
                Ok(v) => {

                    println!("{} {:?}", id, v);

                    thread::spawn(move || { Worker::work(v); });

                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        });

        Worker { id: id, thread: t }
    }

    fn work(v: record::Image) {
        let bgr8_slice = v.Image;

        let mut frame = gif::Frame::from_rgb(1000, 1000, convert(bgr8_slice).as_slice());

    }
}

pub fn process(rx: mpsc::Receiver<record::Image>) {

    let mut f = File::create("img.gif").unwrap();

    let color_map = &[0xFF, 0xFF, 0xFF];

    let mut filef = gif::Encoder::new(f, 1000, 1000, color_map).expect("Failed to create encoder");

    let pool = ThreadPool::new(8, rx);

    //    for v in rx {
    //  filef.write_frame(&frame);

    // }/
}

fn convert(u: x11cap::Image) -> Vec<u8> {
    let s = u.as_slice();

    let mut v: Vec<u8> = Vec::with_capacity(s.len());

    for i in s {
        v.push(i.r);
        v.push(i.g);
        v.push(i.b);
    }
    v
}
