extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("notify.txt", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
               println!("{:?} {:?} ({:?})", op, path, cookie)
           },
           Ok(event) => println!("broken event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}