#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};
use std::os::raw;
use std::ptr;

#[derive(Debug)]
pub enum NotmuchError {
    FfiCString(std::ffi::NulError),
    DbFailedToOpen(String),
}

impl From<std::ffi::NulError> for NotmuchError {
    fn from(error: std::ffi::NulError) -> Self {
        NotmuchError::FfiCString(error)
    }
}

pub type NotmuchResult<T> = Result<T, NotmuchError>;

#[derive(Debug)]
pub struct NotmuchDb {
    db_ptr: *mut notmuch_database_t,
}

fn c_string_to_owned(ptr: *const raw::c_char) -> String {
    unsafe {
        if ptr.is_null() {
            "".to_owned()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

impl NotmuchDb {
    pub fn open(path: &str) -> NotmuchResult<NotmuchDb> {
        let path = CString::new(path)?;

        let result: notmuch_status_t;
        let mut db_ptr: *mut notmuch_database_t = ptr::null_mut();
        let mut msg: *mut raw::c_char = ptr::null_mut();
        unsafe {
            result = notmuch_database_open_verbose(
                path.as_ptr(),
                notmuch_database_mode_t_NOTMUCH_DATABASE_MODE_READ_ONLY,
                &mut db_ptr,
                &mut msg,
            );
        }
        if result != _notmuch_status_NOTMUCH_STATUS_SUCCESS {
            Err(NotmuchError::DbFailedToOpen(c_string_to_owned(msg)))
        } else {
            Ok(NotmuchDb { db_ptr })
        }
    }
}

impl Drop for NotmuchDb {
    fn drop(&mut self) {
        unsafe {
            if !self.db_ptr.is_null() {
                notmuch_database_destroy(self.db_ptr);
            }
        }
    }
}
