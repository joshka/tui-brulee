#![allow(dead_code)]

use std::io;

pub mod image;
pub mod text;

pub fn to_io_error(err: taffy::TaffyError) -> io::Error {
    io::Error::other(err)
}
