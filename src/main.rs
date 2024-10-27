mod event_watcher;
mod event_handler;
mod file_utils;

use std::{fs, process::exit};

fn main() {
    let watched_path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    let destination_path = std::env::args()
        .nth(2)
        .expect("Argument 2 needs to be a path");
    println!("Watching {}", watched_path);
    println!("Organizing to {}", destination_path);

    if let Err(e) = fs::create_dir_all(&destination_path) {
        println!("[Error] Could not create directory {}", destination_path);
        println!("error: {:?}", e);
        exit(1);
    }

    futures::executor::block_on(async {
        if let Err(e) = event_watcher::async_watch(watched_path, destination_path).await {
            println!("error: {:?}", e); // TODO: better error display for users
        }
    });
}
