extern crate x11;

use std::ffi;
use std::os::raw::c_char;
use x11::xlib;
use std::fmt;
use std::ptr;


struct Display {
    width: i32,
    height: i32,
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

    d.fetch_window_attributes();

    println!("{}", d);


    let img = d.get_image();

    unsafe {
        println!("{}", (*img).depth);
    }

    //if !display.is_null() {
    println!("image Captured");
    // } else {
    println!("No Image Captured");
    //  }
}

impl Display {
    fn new(s: std::ffi::CString) -> Display {

        let x_img_n_ptr: *mut x11::xlib::XImage;

        let display = open_display(s.as_ptr());
        let h: i32;
        let w: i32;

        unsafe {
            w = xlib::XDisplayWidth(display, 0);
            h = xlib::XDisplayHeight(display, 0);
        }
        let d = Display {
            height: h,
            width: w,
            refer: display,
        };
        d
    }

    fn get_resolution(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    fn fetch_window_attributes(&self) -> *mut xlib::XWindowAttributes {
        let mut window_attribs: *mut xlib::XWindowAttributes = ptr::null_mut();

        unsafe {

            let window = xlib::XRootWindow(self.refer, 0);

            let status = xlib::XGetWindowAttributes(self.refer, window, window_attribs);
            println!("Status: {}", status);
        };


        window_attribs
    }

    fn get_image(&self) -> *mut xlib::XImage {

        let img: *mut xlib::XImage;

        unsafe {
            img = xlib::XGetImage(
                self.refer,
                xlib::XDefaultRootWindow(self.refer),
                0,
                0,
                1920,
                1080,
                1,
                xlib::ZPixmap,
            );
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

fn open_display(d: *const c_char) -> *mut x11::xlib::Display {
    let display: *mut x11::xlib::Display;
    unsafe {
        display = x11::xlib::XOpenDisplay(d);
    }

    display
}
