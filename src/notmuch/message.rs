use super::*;
use serde::{Deserialize, Serialize};

unsafe impl Send for MessageSearchResult {}

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

#[derive(Debug, Serialize, Deserialize)]
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
