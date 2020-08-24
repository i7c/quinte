#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::c_string_to_owned;
use std::ffi::CString;
use std::os::raw;
use std::ptr;

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

pub type NotmuchResult<T> = Result<T, NotmuchError>;

#[derive(Debug)]
pub struct NotmuchDb {
    db_ptr: *mut notmuch_database_t,
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
            Ok(NotmuchDb { db_ptr })
        }
    }

    pub fn search(&self, search_string: &str) -> NotmuchResult<MessageSearchResult> {
        let search_cstr = CString::new(search_string)?;

        unsafe {
            let query = notmuch_query_create(self.db_ptr, search_cstr.as_ptr());
            let mut messages: *mut notmuch_messages_t = ptr::null_mut();

            let status = notmuch_query_search_messages(query, &mut messages);

            if status != _notmuch_status_NOTMUCH_STATUS_SUCCESS {
                // if we never got so far as to create a MessageSearchResult, we destroy the query
                // manually, because it won't be destroyed by droping a MessageSearchResult
                notmuch_query_destroy(query);
                Err(NotmuchError::SearchMessagesFailed)
            } else {
                Ok(MessageSearchResult {
                    query,
                    messages_c_iter: messages,
                })
            }
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

pub struct MessageSearchResult {
    query: *mut notmuch_query_t,
    messages_c_iter: *mut notmuch_messages_t,
}

impl Drop for MessageSearchResult {
    fn drop(&mut self) {
        if !self.query.is_null() {
            unsafe {
                notmuch_query_destroy(self.query);
            }
            self.query = ptr::null_mut();
        }
    }
}

impl Iterator for MessageSearchResult {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        if self.query.is_null() || self.messages_c_iter.is_null() {
            return None;
        }
        unsafe {
            if notmuch_messages_valid(self.messages_c_iter) != 0 {
                let message = notmuch_messages_get(self.messages_c_iter);

                let content_type = get_header(message, "Content-Type");
                let date = notmuch_message_get_date(message);
                let from = get_header(message, "From")
                    .expect("notmuch did not return a from header for this message");
                let path = c_string_to_owned(notmuch_message_get_filename(message))
                    .expect("notmuch did not return a path for this message");
                let subject = get_header(message, "Subject");
                let to = get_header(message, "To");

                notmuch_messages_move_to_next(self.messages_c_iter);
                Some(Message {
                    content_type,
                    date,
                    from,
                    path,
                    subject,
                    to,
                })
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub content_type: Option<String>,
    pub date: i64,
    pub from: String,
    pub path: String,
    pub subject: Option<String>,
    pub to: Option<String>,
}

/// Returns the value of header from msg.
///
/// `msg` must be a valid pointer to a notmuch_message_t. We won't check it!
fn get_header(msg: *mut notmuch_message_t, header: &str) -> Option<String> {
    unsafe {
        c_string_to_owned(notmuch_message_get_header(
            msg,
            CString::new(header).expect("CString::new failed").as_ptr(),
        ))
    }
}
