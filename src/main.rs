mod event_handler;
mod event_watcher;
mod file_utils;
mod utils;

use std::{env, fs, process::exit};

type ArgsIter = std::iter::Skip<env::Args>;

fn prepare(mut args: ArgsIter) -> (String, String) {
    let watched_path = match args.next() {
        Some(path) => path,
        None => {
            println!("file-tamer: missing source directory");
            println!("Try 'file-tamer help' for more information.");
            exit(1);
        }
    };
    let destination_path = match args.next() {
        Some(path) => path,
        None => {
            println!("file-tamer: missing destination directory");
            println!("Try 'file-tamer help' for more information.");
            exit(1);
        }
    };

    if let Err(e) = fs::create_dir_all(&destination_path) {
        println!("[Error] Could not create directory {}", destination_path);
        println!("error: {:?}", e);
        exit(1);
    }

    return (watched_path, destination_path);
}

fn display_help() -> () {
    println!("Usage: file-tamer COMMAND [ARGS]");
    println!();
    println!("Here is a list of possible commands:");
    println!();
    println!("  watch      WATCHED_DIRECTORY DESTINATION_DIRECTORY");
    println!("             watch a directory for new files and move them as they come");
    println!("  organize   SOURCE_DIRECTORY DESTINATION_DIRECTORY");
    println!("             organize a directory to another");
    println!("  help");
    println!("             display this help");
}

fn main() {
    let mut args = env::args().skip(1); // Skip the executable name

    let command = match args.next() {
        Some(cmd) => cmd,
        None => {
            println!("file-tamer: missing command");
            println!("Try 'file-tamer help' for more information.");
            exit(1);
        }
    };
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
        "help" | "--help" | "-h" => {
            display_help();
        },
        _ => {
            println!("file-tamer: unrecognized command '{}'", command);
            println!("Try 'file-tamer help' for more information.");
            exit(1);
        }
    }
}
