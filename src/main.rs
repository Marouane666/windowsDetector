extern crate winapi;
extern crate chrono;

use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};
use std::ptr::null_mut;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use std::time::{Instant, Duration};

fn main() {
    let mut last_title = String::new();
    let mut log_entries = String::new();
    let mut start = Instant::now();

    loop {
        // Check if one minute has passed
        if start.elapsed() >= Duration::from_secs(60) {
            // Log to file if there are entries to log
            if !log_entries.is_empty() {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("window_log.txt")
                    .expect("Cannot open file");

                file.write_all(log_entries.as_bytes()).expect("Cannot write to file");
                log_entries.clear(); // Clear the log entries for the next minute
            }
            start = Instant::now(); // Reset the timer
        }

        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_null() {
            continue;
        }

        let mut title: [u16; 256] = [0; 256];
        let len = unsafe {
            GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32)
        };

        if len > 0 {
            let title_os = OsString::from_wide(&title[..len as usize]);
            let current_title = title_os.to_string_lossy().into_owned();

            // Only proceed if the window title has changed
            if current_title != last_title {
                last_title = current_title.clone();

                // Get the current time
                let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let log_entry = format!("{} - {}\n", time, current_title);
                
                // Print the window title and time to the console
                println!("{}", log_entry.trim_end());

                // Add the current log entry for file logging
                log_entries.push_str(&log_entry);
            }
        }
    }
}
