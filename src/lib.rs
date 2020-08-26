pub mod notmuch;
pub mod server;

use std::{ffi::CStr, os::raw};

#[derive(Debug)]
pub enum Error {
    CString,
    FrameParse(String),
    Internal(&'static str),
    NotmuchSearch,
    UnknownPayload,
    WebSocket(String),
}

impl From<std::ffi::NulError> for Error {
    fn from(_error: std::ffi::NulError) -> Self {
        Error::CString
    }
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
