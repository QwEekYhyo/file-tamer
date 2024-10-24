use std::{fs, path::{Path, PathBuf}};
use notify::{event::CreateKind, Event, EventKind};

use crate::file_utils;

pub fn handle_event<P: AsRef<Path>>(event: Event, path: &P) -> () {
    match event.kind {
        EventKind::Create(create_kind) => {
            if let CreateKind::File = create_kind {
                println!("New file created at {:?}", event.paths);
                // I am guessing there can only be one path in Event::paths if creating a file
                let file_path = &event.paths[0];
                let extension = match file_path.extension() {
                    Some(ext) => ext.to_str().unwrap(),
                    None => "none",
                };

                if let Some(destination_dir) = file_utils::EXTENSION_TO_DIRECTORY
                    .lock()
                    .unwrap()
                    .get(extension)
                {
                    let mut path_buffer = PathBuf::new();
                    path_buffer.push(path);
                    path_buffer.push(destination_dir);
                    if fs::create_dir_all(&path_buffer).is_ok() {
                        path_buffer.push(file_path.file_name().unwrap());
                        if let Err(e) = fs::rename(&event.paths[0], &path_buffer) {
                            eprintln!("Failed to move file: {}", e);
                        }
                    }
                }
            }
        },
        _ => {
            println!("Received unhandled event, type {:?}", event.kind);
        }
    }
}
