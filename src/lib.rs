#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod message_box_impl;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod message_box_impl;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
mod message_box_impl {
    pub(crate) fn create_message_box_impl(_: &str, _: &str) {}
}

/// Create a message box with the given message and title.
pub fn create_message_box(message: &str, title: &str) {
    message_box_impl::create_message_box_impl(message, title);
}
