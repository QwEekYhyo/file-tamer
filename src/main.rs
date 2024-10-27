mod event_handler;
mod event_watcher;
mod file_utils;
mod utils;

use std::{env, fs, process::exit};

type ArgsIter = std::iter::Skip<env::Args>;

fn prepare(mut args: ArgsIter) -> (String, String) {
    let watched_path = args.next()
        .expect("Argument 2 needs to be a path");
    let destination_path = args.next()
        .expect("Argument 3 needs to be a path");

    if let Err(e) = fs::create_dir_all(&destination_path) {
        println!("[Error] Could not create directory {}", destination_path);
        println!("error: {:?}", e);
        exit(1);
    }

    return (watched_path, destination_path);
}

fn main() {
    let mut args = env::args().skip(1); // Skip the executable name

    let command = args.next().expect("First argument should be a command");
    match command.as_str() {
        "watch" => {
            let (watched_path, destination_path) = prepare(args);

            println!("Watching {}", watched_path);
            println!("Organizing to {}", destination_path);

            futures::executor::block_on(async {
                if let Err(e) = event_watcher::async_watch(watched_path, destination_path).await {
                    println!("error: {:?}", e); // TODO: better error display for users
                }
            });
        },
        "organize" | "organise" => {
            let (dir, destination_path) = prepare(args);

            println!("This will move files currently in {}", dir);
            if utils::ask_confirmation("Do you really wish to continue?") {
                println!("Organizing {} to {}", dir, destination_path);
                file_utils::organize_directory(&dir, &destination_path);
            }
        },
        _ => {
            println!("Unknown command: {}", command);
            println!("Usage: executable watch <watched_dir> <destination_dir>");
            println!("Usage: executable organize <directory> <destination_dir>");
            exit(1);
        }
    }
}
