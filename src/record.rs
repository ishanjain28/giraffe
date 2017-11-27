extern crate x11cap;
extern crate gif;

use std;

#[derive(Debug)]
pub struct Image {
    pub id: u64,
    pub Image: x11cap::Image,
    pub width: u16,
    pub height: u16,
}

pub struct gifFrame<'a> {
    pub frame: gif::Frame<'a>,
    pub id: u64,
}

unsafe impl std::marker::Send for Image {}
