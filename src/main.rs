mod event_watcher;
mod event_handler;
mod file_utils;

use std::{env, fs, process::exit};

fn main() {
    let mut args = env::args().skip(1); // Skip the executable name

    let command = args.next().expect("First argument should be a command");
    match command.as_str() {
        "watch" => {
            let watched_path = args.next()
                .expect("Argument 2 needs to be a path");
            let destination_path = args.next()
                .expect("Argument 3 needs to be a path");

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
        _ => {
            println!("Unknown command: {}", command);
            println!("Usage: executable watch <watched_path> <destination_path>");
            exit(1);
        }
    }
}
