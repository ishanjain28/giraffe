extern crate x11;
extern crate log;
extern crate fern;
extern crate chrono;

use x11::{xlib, glx, xcursor};
use std::os::raw;
use std::ffi::CString;
use std::{mem, ptr};


#[derive(Debug)]
pub struct Selector {
    display: *mut xlib::Display,
    screens: i32,
}

#[derive(Debug)]
struct Pointer {
    root_x: i32,
    root_y: i32,
    win_x: i32,
    win_y: i32,
    mask: u32,
    root_win: xlib::Window,
    child_win: xlib::Window,
}

impl Selector {
    fn new(dpy_id: *const raw::c_char) -> Selector {

        let dpy = open_display(dpy_id);

        let screen_count = unsafe { xlib::XScreenCount(dpy) };

        let mut s = Selector {
            display: dpy,
            screens: screen_count,
        };

        s
    }

    fn get_root_window(&self) -> xlib::Window {
        unsafe { xlib::XDefaultRootWindow(self.display) }
    }

    fn query_pointer(&self) -> Option<Pointer> {

        let mut win: xlib::Window;
        let mut p: Pointer;

        unsafe {
            win = mem::uninitialized();
            p = mem::uninitialized();

            let found = xlib::XQueryPointer(
                self.display,
                self.get_root_window(),
                &mut p.root_win,
                &mut p.child_win,
                &mut p.root_x,
                &mut p.root_y,
                &mut p.win_x,
                &mut p.win_y,
                &mut p.mask,
            );

            if found == xlib::False {
                return None;
            } else {
                return Some(p);
            }
        }
    }

    fn grab_pointer(&self) -> bool {
        unsafe {


            // TODO: Figure out a better mouse pointer for selection window
            let cursor = xcursor::XcursorLibraryLoadCursor(
                self.display,
                CString::new("sb_v_precision_select").unwrap().as_ptr(),
            );

            let grabbed = xlib::XGrabPointer(
                self.display,
                self.get_root_window(),
                xlib::False,
                (xlib::ButtonPressMask | xlib::ButtonReleaseMask |
                     xlib::Button1MotionMask) as u32,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
                0,
                cursor,
                0,
            );

            if grabbed == xlib::GrabSuccess {
                true
            } else {
                false
            }

        }
    }

    fn release_pointer(&self) {}

    fn create_gc(&self) -> *mut xlib::_XGC {

        let mut xgcval: xlib::XGCValues;
        xgcval.background = 0x00FF00;
        xgcval.foreground = 0xFFFFFF;

        unsafe {
            xgcval = mem::uninitialized();

            xlib::XCreateGC(self.display, self.get_root_window(), 0, &mut xgcval)
        }
    }

    fn draw_rectangle(&self, x: i32, y: i32, w: u32, h: u32) {

        let mut gc = self.create_gc();

        let mut xcol: xlib::XColor;
        let mut cmap: xlib::Colormap;
        let mut vinfo: xlib::XVisualInfo;

        unsafe {
            xcol = mem::uninitialized();
            cmap = mem::uninitialized();
            vinfo = mem::uninitialized();

            xcol.red = 200 * 256; // 16 bit colors
            xcol.green = 100 * 256;
            xcol.blue = 50 * 256;

            xlib::XMatchVisualInfo(
                self.display,
                xlib::XDefaultScreen(self.display),
                32,
                xlib::TrueColor,
                &mut vinfo,
            );

            cmap = xlib::XCreateColormap(
                self.display,
                self.get_root_window(),
                vinfo.visual,
                xlib::AllocNone,
            );

            xlib::XFillRectangle(self.display, self.get_root_window(), gc, x, y, w, h);
        }
    }
}


pub fn draw_selection_window(mon_id: *const raw::c_char) {
    let mut s = Selector::new(mon_id);

    info!("This display has {} screens", s.screens);

    let grabbed = s.grab_pointer();
    if !grabbed {
        error!("Error in grabbing pointer");
        return;
    }
    let mut evt: xlib::XEvent;

    // Listen for mouse/keyboard events
    // I only need to listen to mouse button press/release and keyrelease event on keyboard

    unsafe {
        evt = mem::uninitialized();

        while true {
            xlib::XNextEvent(s.display, &mut evt);

            let mut draw_rectangle = false;
            let mut x = 0;
            let mut y = 0;

            match evt.button.type_ {

                xlib::ButtonPress => {
                    if evt.button.button == 1 {
                        draw_rectangle = true;

                        x = evt.button.x_root;
                        y = evt.button.y_root;

                    }
                }
                xlib::MotionNotify => {

                    // Left Mouse Button
                    s.draw_rectangle(x, y, evt.button.x_root as u32, evt.button.y_root as u32);

                    println!("{:?}", evt);

                }
                //                xlib::ButtonRelease => {

                    // Left Mouse Button
  //                  if evt.button.button == 1 {
    //                    println!("Left Mouse Button Released");
      //              }

        //        }
                _ => {
                    //println!("{:?}", evt);
                }
            }


        }
    }

    while true {
        match s.query_pointer() {
            Some(v) => {

                println!("{:?}", v);
            }
            None => {
                error!("Mouse Pointer not found");

            }
        }
    }
}

pub fn draw_transparent_window(mon_id: *const raw::c_char) {

    println!("Drawing transparent window");

    let display = open_display(mon_id);

    let screen = unsafe { xlib::XDefaultScreen(display) };
    let window = unsafe { xlib::XDefaultRootWindow(display) };

    let mut vinfo: xlib::XVisualInfo;
    let mut win: xlib::Window;
    let mut gc: *mut xlib::_XGC;
    let mut attr: xlib::XSetWindowAttributes;
    let mut xgcval: xlib::XGCValues;
    let mut xcol: xlib::XColor;

    unsafe {
        vinfo = mem::uninitialized();
        win = mem::uninitialized();
        gc = mem::uninitialized();
        attr = mem::uninitialized();
        xgcval = mem::uninitialized();
        xcol = mem::uninitialized();

        xlib::XMatchVisualInfo(display, screen, 32, xlib::TrueColor, &mut vinfo);

        attr.colormap = xlib::XCreateColormap(display, window, vinfo.visual, xlib::AllocNone);
        attr.event_mask = xlib::ExposureMask | xlib::KeyPressMask;
        attr.background_pixmap = 0;
        attr.border_pixel = 0;
        win = xlib::XCreateWindow(
            display,
            window,
            0,
            0,
            1920,
            1080,
            0,
            vinfo.depth,
            xlib::InputOutput as u32,
            vinfo.visual,
            xlib::CWColormap | xlib::CWEventMask | xlib::CWBackPixmap | xlib::CWBorderPixel,
            &mut attr,
        );


        gc = xlib::XCreateGC(display, win, 0, &mut xgcval);
        xcol.red = 153 * 256; // 16 bit colors
        xcol.green = 116 * 256;
        xcol.blue = 65 * 256;

        xlib::XAllocColor(display, attr.colormap, &mut xcol);
        xgcval.foreground = xcol.pixel;

        xlib::XChangeGC(display, gc, xlib::GCForeground as u64, &mut xgcval);

        let mut glxContext =
            glx::glXCreateContext(display, &mut vinfo, ptr::null_mut(), xlib::True);

        if glxContext == ptr::null_mut() {
            println!("glxContext in null");
            return;
        }

        glx::glXMakeCurrent(display, win, glxContext);

        xlib::XMapWindow(display, win);

        let userwanttoclose = false;

        while !userwanttoclose {
            let mut redraw = 0;

            while xlib::XPending(display) > 0 {
                let mut xevt: xlib::XEvent;
                xevt = mem::uninitialized();
                xlib::XNextEvent(display, &mut xevt);

                redraw = 1;
                break;
            }
            if redraw == 1 {


                glx::glXSwapBuffers(display, win);
                glx::glXWaitGL();

                xlib::XDrawString(
                    display,
                    win,
                    gc,
                    10,
                    20,
                    CString::new("Hello").unwrap().as_ptr(),
                    5,
                );
            }
        }
    }
}

fn open_display(id: *const raw::c_char) -> *mut xlib::Display {
    let dpy;
    unsafe {
        dpy = xlib::XOpenDisplay(ptr::null());
    }
    dpy
}
