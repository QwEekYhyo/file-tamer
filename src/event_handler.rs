use std::path::Path;
use notify::{event::{CreateKind, ModifyKind, RenameMode}, Event, EventKind};

use crate::file_utils;

pub fn handle_event<P: AsRef<Path>>(event: Event, path: &P) -> () {
    match event.kind {
        EventKind::Create(create_kind) => {
            // Not using a second match because I don't want too much indentation
            let new_file_path = &event.paths[0];
            if let CreateKind::File = create_kind {
                println!("[Event] New file created at {:?}", new_file_path);
                file_utils::move_to_correct_dir(new_file_path, path);
            } else if let CreateKind::Any = create_kind { // Only needed on Windows???
                println!("[Event] New file or folder created at {:?} (unknown type)", new_file_path);
                if new_file_path.is_file() {
                    file_utils::move_to_correct_dir(new_file_path, path);
                }
            }
        },
        EventKind::Modify(modify_kind) => {
            if let ModifyKind::Name(RenameMode::To) = modify_kind {
                println!("[Event] File moved at {:?}", event.paths[0]);
                file_utils::move_to_correct_dir(&event.paths[0], path);
            }
        },
        _ => {
            println!("[Event] Received unhandled event, type {:?}", event.kind);
        }
    }
}
