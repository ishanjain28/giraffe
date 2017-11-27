extern crate x11cap;
extern crate gif;

use std::sync::{mpsc, Arc, Mutex};
use std::fs::File;
use std::thread;
use std::boxed::Box;

use record;

#[derive(Debug)]
struct ThreadPool {
    workers: Vec<Worker>,
}

#[derive(Debug)]
struct Worker {
    busy: bool,
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    fn new(
        u: usize,
        rx: mpsc::Receiver<record::Image>,
        res_tx: mpsc::Sender<gif::Frame<'static>>,
    ) -> ThreadPool {
        let mut workers = Vec::with_capacity(u);
        let receiver = Arc::new(Mutex::new(rx));
        let res_tx = Arc::new(Mutex::new(res_tx));

        for id in 0..u {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&res_tx)));
        }
        ThreadPool { workers }
    }

    fn add_workers(
        &mut self,
        u: usize,
        rx: mpsc::Receiver<record::Image>,
        res_tx: mpsc::Sender<gif::Frame<'static>>,
    ) {

        let receiver = Arc::new(Mutex::new(rx));
        let res_tx = Arc::new(Mutex::new(res_tx));

        for id in 0..u {
            self.workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&res_tx),
            ));
        }
    }
}

impl Worker {
    fn new<'a>(
        id: usize,
        rx: Arc<Mutex<mpsc::Receiver<record::Image>>>,
        results: Arc<Mutex<mpsc::Sender<gif::Frame<'static>>>>,
    ) -> Worker {

        let results_clone = mpsc::Sender::clone(&results.lock().unwrap());

        let t = thread::spawn(move || loop {

            let rx = rx.lock().unwrap();

            match rx.recv() {
                Ok(v) => {

                    // Free lock so other threads can use it
                    drop(rx);
                    println!("{} {:?}", id, v);

                    Worker::work(v, results_clone);
                }
                Err(e) => {
                    break;
                }
            }
        });

        Worker {
            id: id,
            thread: t,
            busy: false,
        }
    }

    fn work(v: record::Image, res_tx: mpsc::Sender<gif::Frame>) {

        let bgr8_slice = v.Image.as_slice();

        let mut frame = gif::Frame::from_rgb(v.width, v.height, convert(bgr8_slice).as_slice());

        res_tx.send(frame);

    }
}

pub fn process(rx: mpsc::Receiver<record::Image>, file: &mut File) {

    let mut f = File::create("img.gif").unwrap();

    let color_map = &[0xFF, 0xFF, 0xFF];

    let mut filef = gif::Encoder::new(f, 1000, 1000, color_map).expect("Failed to create encoder");


    let (res_tx, res_rx): (mpsc::Sender<gif::Frame<'static>>,
                           mpsc::Receiver<gif::Frame<'static>>) = mpsc::channel();

    let pool = ThreadPool::new(8, rx, res_tx);

    //    for v in rx {
    //  filef.write_frame(&frame);

    // }/
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
