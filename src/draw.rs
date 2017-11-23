extern crate x11;
extern crate log;
extern crate fern;
extern crate gl;

use x11::{xlib, glx};
use std::os::raw;
use std::ffi::CString;
use std::{mem, ptr};

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

    };
}

fn open_display(id: *const raw::c_char) -> *mut xlib::Display {
    let dpy;
    unsafe {
        dpy = xlib::XOpenDisplay(id);
    }
    dpy
}
