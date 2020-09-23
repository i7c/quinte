#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod message;

use super::{Error, Result};
use crate::c_string_to_owned;
use message::Message;
use std::ffi::CString;
use std::fs;
use std::os::raw;
use std::{ptr, sync::Mutex};

#[derive(Debug, thiserror::Error)]
pub enum NotmuchError {
    #[error("Invalid CString")]
    FfiCString(std::ffi::NulError),
    #[error("{0}")]
    DbFailedToOpen(String),
}

impl From<std::ffi::NulError> for NotmuchError {
    fn from(error: std::ffi::NulError) -> Self {
        NotmuchError::FfiCString(error)
    }
}

pub type NotmuchResult<T> = std::result::Result<T, NotmuchError>;

#[derive(Debug)]
pub struct NotmuchDb {
    db_ptr: Mutex<NotmuchDbPointer>,
}

#[derive(Debug)]
struct NotmuchDbPointer(*mut notmuch_database_t);

unsafe impl Send for NotmuchDbPointer {}

impl Drop for NotmuchDb {
    fn drop(&mut self) {
        // Unwrap panics when the mutex was poisoned, which means another thread panicked while it held the mutex.
        // If this drop is called while panicking, a second panic will abort the program - but this seems fine.
        let db_ptr = self.db_ptr.lock().expect("Poisoned Mutex").0;
        if !db_ptr.is_null() {
            unsafe {
                notmuch_database_destroy(db_ptr);
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
                db_ptr: Mutex::new(NotmuchDbPointer(db_ptr)),
            })
        }
    }

    pub fn search(&self, search_string: &str) -> Result<MessageSearchResult> {
        let db_ptr = self.db_ptr.lock().expect("Poisoned Mutex").0;
        let search_cstr = CString::new(search_string)?;

        unsafe {
            let query = notmuch_query_create(db_ptr, search_cstr.as_ptr());
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

    pub fn find_message(&self, message_id: &str) -> Result<Message> {
        let db_ptr = self.db_ptr.lock().expect("Poisoned Mutex").0;
        let message_id_cstr = CString::new(message_id)?;

        unsafe {
            let mut message_ptr: *mut notmuch_message_t = ptr::null_mut();
            let status =
                notmuch_database_find_message(db_ptr, message_id_cstr.as_ptr(), &mut message_ptr);
            if status != _notmuch_status_NOTMUCH_STATUS_SUCCESS || message_ptr.is_null() {
                Err(Error::MessageIdNotFound(message_id.to_owned()))
            } else {
                let message = Message::from_notmuch(message_ptr);
                notmuch_message_destroy(message_ptr);
                return Ok(message);
            }
        }
    }

    pub fn load_mail(&self, message_id: &str) -> Result<String> {
        let message = self.find_message(message_id)?;
        fs::read_to_string(&message.path).map_err(|_| Error::MailLoad(message.path))
    }
}

pub struct MessageSearchResult {
    query: *mut notmuch_query_t,
    messages_c_iter: *mut notmuch_messages_t,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notmuch_db_is_sync() {
        fn is_sync<T: Sync>() {}
        is_sync::<NotmuchDb>();
    }

    #[test]
    fn notmuch_db_is_send() {
        fn is_send<T: Send>() {}
        is_send::<NotmuchDb>();
    }
}
