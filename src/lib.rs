pub mod notmuch;
pub mod server;

use std::{ffi::CStr, os::raw};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid CString")]
    CString,
    #[error("Failed to parse frame: {0:?}")]
    FrameParse(String),
    #[error("Internal error ({0:?})")]
    Internal(&'static str),
    #[error("Failed to search")]
    NotmuchSearch,
    #[error("Unknown Payload")]
    UnknownPayload,
    #[error("Websocket Error: {0:?}")]
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
