#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use std::io::Write;
use std::thread;
use minifb::{Key, MouseMode, Window, WindowOptions};
use tungstenite::{self, Message, Result, Error};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    const CHECKBOX_TEXT: &str = "Tell us about the crash so we can fix it";
    const DETAILS_BUTTON_TEXT: &str = "Details...";
    const QUIT_BUTTON_TEXT: &str = "Quit";
    const RESTART_BUTTON_TEXT: &str = "Restart";
    const CRASH_REPORT_MESSAGE: &str = "We're Sorry. A crash has occurred. We value your feedback.";
    const SUBMIT_MESSAGE: &str = "Your crash report will be submitted before you quit or restart.";

    const WINDOW_TITLE: &str = "Rust Crash Reporter";
    const WINDOW_WIDTH: usize = 400;
    const WINDOW_HEIGHT: usize = 300;

    let mut error_message = "Sample crash error message goes here...".to_string();
    let mut window = Window::new(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to open window");

    // Create UI elements
    let mut checkbox_id = 0;
    let mut details_button_id = 0;
    let mut quit_button_id = 0;
    let mut restart_button_id = 0;
    let mut text_box_id = 0;
    {
        // let mut y = 20;
        // window.add_text(10, y, CRASH_REPORT_MESSAGE, 16).expect("Unable to add text");
        // y += 30;
        // window
        //     .add_text(10, y, &error_message, 14)
        //     .expect("Unable to add text");
        // y += 30;
        // checkbox_id = window
        //     .add_checkbox(10, y, CHECKBOX_TEXT, true)
        //     .expect("Unable to add checkbox");
        // y += 25;
        // details_button_id = window
        //     .add_button(10, y, DETAILS_BUTTON_TEXT)
        //     .expect("Unable to add details button");
        // y += 25;
        // text_box_id = window.add_textbox(10, y, 380, 100).expect("Unable to add textbox");
        // y += 110;
        // window.add_text(10, y, SUBMIT_MESSAGE, 12).expect("Unable to add text");
        // y += 25;
        // quit_button_id = window
        //     .add_button(10, y, QUIT_BUTTON_TEXT)
        //     .expect("Unable to add quit button");
        // y += 25;
        // restart_button_id = window
        //     .add_button(180, y, RESTART_BUTTON_TEXT)
        //     .expect("Unable to add restart button");
    }

    let mut running = true;
    while running {
        // let event = window.poll_event();
        // match event {
        //     minifb::Event::Quit => {
        //         running = false;
        //     }
        //     minifb::Event::Key(Key::Escape, _, _) => {
        //         running = false;
        //     }
        //     minifb::Event::Checkbox(id, checked) => {
        //         if id == checkbox_id {
        //             println!("Checkbox: {}", checked);
        //         }
        //     }
        //     minifb::Event::Button(id, _) => {
        //         if id == details_button_id {
        //             println!("Pressed Details...");
        //             // Later: show details dialog
        //         }
        //         if id == quit_button_id {
        //             println!("Pressed Quit");
        //             running = false;
        //         }
        //         if id == restart_button_id {
        //             println!("Pressed Restart");
        //             running = false;
        //         }
        //     }
        //     _ => {}
        // }
        // window.update();
    }

    Ok(())
}

// Test case to drive the window
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crash_reporter_window() {
        // let mut output = std::io::sink::Stdout::new();
        std::thread::spawn(move || {
            if let Err(e) = main() {
                eprintln!("Error: {}", e);
            }
        });

        // Simulate user interactions
        thread::sleep(std::time::Duration::from_millis(500));
        // unsafe {
        //     minifb::push_event(minifb::Event::Checkbox(0, true));
        //     minifb::push_event(minifb::Event::Button(1, minifb::MouseMode::Pressed));
        //     thread::sleep(std::time::Duration::from_millis(200));
        //     minifb::push_event(minifb::Event::Button(2, minifb::MouseMode::Pressed));
        // }
    }
}
