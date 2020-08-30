#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod message;

use super::{Error, Result};
use crate::c_string_to_owned;
use std::ffi::CString;
use std::os::raw;
use std::{ptr, sync::Mutex};

#[derive(Debug)]
pub enum NotmuchError {
    FfiCString(std::ffi::NulError),
    DbFailedToOpen(String),
    SearchMessagesFailed,
}

impl From<std::ffi::NulError> for NotmuchError {
    fn from(error: std::ffi::NulError) -> Self {
        NotmuchError::FfiCString(error)
    }
}

pub type NotmuchResult<T> = std::result::Result<T, NotmuchError>;

#[derive(Debug)]
pub struct NotmuchDb {
    db_ptr: *mut notmuch_database_t,
    mutex: Mutex<()>,
}

unsafe impl Send for NotmuchDb {}
unsafe impl Sync for NotmuchDb {}

impl Drop for NotmuchDb {
    fn drop(&mut self) {
        if !self.db_ptr.is_null() {
            unsafe {
                notmuch_database_destroy(self.db_ptr);
            }
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
            Err(NotmuchError::DbFailedToOpen(
                c_string_to_owned(msg).unwrap_or_else(|| "No error message".to_owned()),
            ))
        } else {
            Ok(NotmuchDb {
                db_ptr,
                mutex: Mutex::new(()),
            })
        }
    }

    pub fn search(&self, search_string: &str) -> Result<MessageSearchResult> {
        let _guard = self.mutex.lock();
        let search_cstr = CString::new(search_string)?;

        unsafe {
            let query = notmuch_query_create(self.db_ptr, search_cstr.as_ptr());
            let mut messages: *mut notmuch_messages_t = ptr::null_mut();

            let status = notmuch_query_search_messages(query, &mut messages);

            if status != _notmuch_status_NOTMUCH_STATUS_SUCCESS {
                // if we never got so far as to create a MessageSearchResult, we destroy the query
                // manually, because it won't be destroyed by droping a MessageSearchResult
                notmuch_query_destroy(query);
                Err(Error::NotmuchSearch)
            } else {
                Ok(MessageSearchResult {
                    query,
                    messages_c_iter: messages,
                })
            }
        }
    }
}

pub struct MessageSearchResult {
    query: *mut notmuch_query_t,
    messages_c_iter: *mut notmuch_messages_t,
}
