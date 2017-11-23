extern crate x11;

use x11::xlib;
use std::os::raw;
use std::ffi::CString;
use std::mem;

pub struct Draw {
    display: *mut xlib::Display,
    screen: i32,
}


impl Draw {
    fn new(s: *const raw::c_char) -> Draw {

        let dpy = open_display(s);

        let screen = unsafe { xlib::XDefaultScreen(dpy) };
        let window = unsafe { xlib::XDefaultRootWindow(dpy) };
        Draw {
            display: dpy,
            screen,
        }
    }
    fn get_visual_info(&self) -> *mut xlib::XVisualInfo {

        let mut nxvisuals;

        let mut visual_list: xlib::XVisualInfo;

        unsafe {
            visual_list = mem::uninitialized();
            nxvisuals = 0;

            visual_list.screen = self.screen;

            xlib::XGetVisualInfo(
                self.display,
                xlib::VisualScreenMask,
                &mut visual_list,
                &mut nxvisuals,
            );
        }

        &mut visual_list
    }

    fn match_visual_info(&self) -> *mut xlib::XVisualInfo {

        let mut vinfo: xlib::XVisualInfo;
        unsafe {
            vinfo = mem::uninitialized();
            if xlib::XMatchVisualInfo(
                self.display,
                self.screen,
                32,
                xlib::TrueColor,
                &mut vinfo,
            ) != 0
            {
                &mut vinfo
            } else {
                &mut vinfo
            }
        }
    }
    fn xsync(&self) -> i32 {
        unsafe { xlib::XSync(self.display, xlib::True) }
    }
    fn get_default_root_window(&self) -> xlib::Window {
        unsafe { xlib::XDefaultRootWindow(self.display) }
    }
    fn create_color_map(&self, v: *mut xlib::XVisualInfo) -> u64 {
        unsafe {
            xlib::XCreateColormap(
                self.display,
                self.get_default_root_window(),
                (*v).visual,
                xlib::AllocNone,
            )
        }
    }

    fn create_window(&self, v: *mut xlib::XVisualInfo) -> u64 {

        let mut attrs: xlib::XSetWindowAttributes;

        unsafe {

            attrs = mem::uninitialized();
            attrs.colormap = self.create_color_map(v);
            attrs.background_pixel = 0;
            attrs.border_pixel = 0;
            attrs.event_mask = xlib::ExposureMask | xlib::KeyPressMask;

            xlib::XCreateWindow(
                self.display,
                self.get_default_root_window(),
                0,
                0,
                1920,
                1080,
                0,
                (*v).depth,
                xlib::InputOutput as u32,
                (*v).visual,
                // TODO: Using CWBackPixmap causes BadPixmap error, Find out reason
                xlib::CWColormap | xlib::CWEventMask | xlib::CWBorderPixel,
                &mut attrs,
            )

        }
    }

    fn map_window(&self, window: u64) -> i32 {

        unsafe { xlib::XMapWindow(self.display, window) }

    }

    fn create_gc(&self, win: xlib::Window) -> *mut xlib::_XGC {
        let mut xgcval: xlib::XGCValues;

        unsafe {
            xgcval = mem::uninitialized();

            xlib::XCreateGC(self.display, win, 0, &mut xgcval)
        }
    }

    fn change_gc(&self, gc: *mut xlib::_XGC) -> i32 {

        let mut xgcval: xlib::XGCValues;

        unsafe {
            xgcval = mem::uninitialized();

            let mut xcol: xlib::XColor;
            xcol.red = 153 * 256; // 16 bit colors
            xcol.green = 116 * 256;
            xcol.blue = 65 * 256;
        
            xlib::XAllocColor(self.display, 

            xlib::XChangeGC(self.display, gc, xlib::GCForeground as u64, &mut xgcval)

        }

    }
}

pub fn draw_transparent_window(mon_id: *const raw::c_char) {

        let dpy = open_display(mon_id);

        let screen = unsafe { xlib::XDefaultScreen(dpy) };
        let window = unsafe { xlib::XDefaultRootWindow(dpy) };


     let mut vinfo: xlib::XVisualInfo;
        unsafe {
            vinfo = mem::uninitialized();
            if xlib::XMatchVisualInfo(
                self.display,
                self.screen,
                32,
                xlib::TrueColor,
                &mut vinfo,
            ) ;
}    
    let w = d.create_window(vinfo);

    d.create_gc(w);

    d.map_window(w);

}

fn open_display(id: *const raw::c_char) -> *mut xlib::Display {
    let dpy;
    unsafe {
        dpy = xlib::XOpenDisplay(id);
    }
    dpy
}
