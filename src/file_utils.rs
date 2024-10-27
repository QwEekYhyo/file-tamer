use std::{collections::HashMap, fs, path::{Path, PathBuf}, sync::Mutex};
use once_cell::sync::Lazy;

static EXTENSION_TO_DIRECTORY: Lazy<Mutex<HashMap<&str, &str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("pdf", "pdf documents");
    map.insert("cpp", "C++ source code");
    map.insert("hpp", "C++ source code");
    map.insert("rs", "Rust source code");
    map.insert("txt", "Text files");

    return Mutex::new(map);
});

pub fn move_to_correct_dir<P: AsRef<Path>>(file_path: &PathBuf, organized_path: &P) -> () {
    let extension = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "none",
    };

    if let Some(destination_dir) = EXTENSION_TO_DIRECTORY
        .lock()
        .unwrap()
        .get(extension)
    {
        let mut path_buffer = PathBuf::new();
        path_buffer.push(organized_path);
        path_buffer.push(destination_dir);
        if fs::create_dir_all(&path_buffer).is_ok() {
            path_buffer.push(file_path.file_name().unwrap());
            if file_path == &path_buffer {
                println!("[File] {} is already in correct directory", path_buffer.display());
                return;
            }
            if let Err(e) = fs::rename(file_path, &path_buffer) {
                eprintln!("Failed to move file: {}", e);
            } else {
                println!("[File] Successfully moved filed to {}", path_buffer.display());
            }
        }
    }
}
