#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};
use std::os::raw;
use std::ptr;

#[derive(Debug)]
pub struct NotmuchErrorDetail {
    pub message: String,
    pub original_error: notmuch_status_t,
}

#[derive(Debug)]
pub enum NotmuchError {
    DbFailedToOpen(NotmuchErrorDetail),
}

pub type NotmuchResult<T> = std::result::Result<T, NotmuchError>;

#[derive(Debug)]
pub struct NotmuchDb {
    db_ptr: *mut notmuch_database_t,
}

impl NotmuchDb {
    pub fn open(path: &str) -> Result<NotmuchDb, NotmuchError> {
        let mut db_ptr: *mut notmuch_database_t = ptr::null_mut();
        let path = CString::new(path).expect("Failed to create CString");

        let result: notmuch_status_t;
        let message: Option<String>;
        unsafe {
            let mut msg: *mut raw::c_char = ptr::null_mut();
            result = notmuch_database_open_verbose(
                path.as_ptr(),
                notmuch_database_mode_t_NOTMUCH_DATABASE_MODE_READ_ONLY,
                &mut db_ptr,
                &mut msg,
            );
            if !msg.is_null() {
                message = Some(
                    CStr::from_ptr(msg)
                        .to_str()
                        .expect("libnotmuch error message was not valid utf8")
                        .to_owned(),
                );
                libc::free(msg as *mut libc::c_void);
            } else {
                message = None;
            }
        }
        if result != _notmuch_status_NOTMUCH_STATUS_SUCCESS {
            Err(NotmuchError::DbFailedToOpen(NotmuchErrorDetail {
                message: format!("Could not open database: {:?}", message),
                original_error: result,
            }))
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
