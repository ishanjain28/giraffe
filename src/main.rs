extern crate x11;

use std::ffi;
use std::os::raw;
use x11::xlib;
use std::fmt;
use std::mem;
use std::io::Read;


struct Display {
    width: u32,
    height: u32,
    refer: *mut xlib::_XDisplay,
}

fn main() {

    let s = match ffi::CString::new(":0") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);

            return;
        }
    };

    let d = Display::new(s);

    let attribs = d.fetch_window_attributes();

    println!("{:?}", attribs);

    let img = d.get_image();



    println!("");

    println!("{:?}", img);

    println!("image Captured");
}

impl Display {
    fn new(s: std::ffi::CString) -> Display {

        let x_img_n_ptr: *mut x11::xlib::XImage;

        let display = open_display(s.as_ptr());
        let h: u32;
        let w: u32;

        unsafe {
            w = xlib::XDisplayWidth(display, 0) as u32;
            h = xlib::XDisplayHeight(display, 0) as u32;
        }
        let d = Display {
            height: h,
            width: w,
            refer: display,
        };
        d
    }

    fn get_resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn fetch_window_attributes(&self) -> xlib::XWindowAttributes {
        let mut window_attribs;

        unsafe {
            window_attribs = mem::uninitialized();
            let window = xlib::XRootWindow(self.refer, 0);

            let status = xlib::XGetWindowAttributes(self.refer, window, &mut window_attribs);
            println!("Window Attributes Status: {}", status);
        };
        window_attribs
    }

    fn get_image(&self) -> xlib::XImage {

        let mut img;
        unsafe {
            img = *(xlib::XGetImage(
                self.refer,
                xlib::XDefaultRootWindow(self.refer),
                0,
                0,
                self.width,
                self.height,
                xlib::XAllPlanes(),
                xlib::ZPixmap,
            ));
        };
        img
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}x{}", self.refer, self.width, self.height)
    }
}

fn create_pixmap(d: xlib::Display) {}

fn open_display(d: *const raw::c_char) -> *mut x11::xlib::Display {
    let display: *mut x11::xlib::Display;
    unsafe {
        display = x11::xlib::XOpenDisplay(d);
    }

    display
}
