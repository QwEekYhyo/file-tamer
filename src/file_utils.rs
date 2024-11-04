use std::{collections::HashMap, fs, path::{self, Path, PathBuf}, sync::Mutex};
use once_cell::sync::Lazy;
use walkdir::WalkDir;

static EXTENSION_TO_DIRECTORY: Lazy<Mutex<HashMap<&str, &str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("pdf", "pdf documents");
    map.insert("cpp", "C++ source code");
    map.insert("hpp", "C++ source code");
    map.insert("rs", "Rust source code");
    map.insert("txt", "Text files");

    return Mutex::new(map);
});

fn are_paths_equal(path1: &PathBuf, path2: &PathBuf) -> std::io::Result<bool> {
    let absolute_path1 = path::absolute(path1)?;
    let absolute_path2 = path::absolute(path2)?;
    return Ok(absolute_path1 == absolute_path2);
}

fn ensure_new_filename(file: &mut PathBuf) -> () {
    let mut counter = 1;

    let old_name = file.file_stem()
        .unwrap_or_default()
        .to_os_string();

    while file.try_exists().unwrap_or(false) {
        let mut new_name = old_name.clone();
        new_name.push(format!(" ({})", counter));

        if let Some(ext) = file.extension() {
            new_name.push(".");
            new_name.push(ext);
        }

        *file = file.with_file_name(new_name);
        counter += 1;
    }
}

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

            match are_paths_equal(file_path, &path_buffer) {
                Ok(true) => {
                    println!("[File] {} is already in correct directory", path_buffer.display());
                    return;
                },
                Ok(false) => {
                    // Paths are not the same, we can proceed to move the file
                }
                Err(e) => {
                    println!("[Error] Could not resolve paths: {:?}", e);
                    return;
                }
            }

            ensure_new_filename(&mut path_buffer);

            if let Err(e) = fs::rename(file_path, &path_buffer) {
                eprintln!("Failed to move file: {}", e);
            } else {
                println!("[File] Successfully moved filed to {}", path_buffer.display());
            }
        }
    }
}

pub fn organize_directory<P: AsRef<Path>>(directory: &P, destination_dir: &P) -> () {
    WalkDir::new(directory).into_iter()
        .filter_map(|e| e.ok())
        .for_each(|entry| move_to_correct_dir(&PathBuf::from(entry.path()), destination_dir));
}
