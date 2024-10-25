use std::path::Path;
use notify::{event::{CreateKind, ModifyKind, RenameMode}, Event, EventKind};

use crate::file_utils;

pub fn handle_event<P: AsRef<Path>>(event: Event, path: &P) -> () {
    match event.kind {
        // Maybe check if file is already in correct dir?
        EventKind::Create(create_kind) => {
            if let CreateKind::File = create_kind {
                println!("New file created at {:?}", event.paths[0]);
                file_utils::move_to_correct_dir(&event.paths[0], path);
            }
        },
        EventKind::Modify(modify_kind) => {
            if let ModifyKind::Name(RenameMode::To) = modify_kind {
                println!("File moved at {:?}", event.paths[0]);
                file_utils::move_to_correct_dir(&event.paths[0], path);
            }
        },
        _ => {
            println!("Received unhandled event, type {:?}", event.kind);
        }
    }
}
