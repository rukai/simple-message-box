#![windows_subsystem = "windows"] // disable the terminal in windows, like a real gui app

extern crate simple_message_box;

fn main() {
    simple_message_box::create_message_box("Hello, World!", "Hi");
}
