pub mod notmuch;
pub mod server;

use std::{ffi::CStr, os::raw};

#[derive(Debug)]
pub enum Error {
    WebSocketError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

fn c_string_to_owned(ptr: *const raw::c_char) -> Option<String> {
    unsafe {
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }
}
