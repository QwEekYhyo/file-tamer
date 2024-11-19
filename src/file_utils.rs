use std::{collections::HashMap, fs::{self, OpenOptions}, path::{self, Path, PathBuf}, sync::Mutex, thread::sleep, time::Duration};
use once_cell::sync::Lazy;
use walkdir::WalkDir;

static EXTENSION_TO_DIRECTORY: Lazy<Mutex<HashMap<&str, &str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("pdf", "pdf documents");
    map.insert("cpp", "C++ source code");
    map.insert("hpp", "C++ source code");
    map.insert("c", "C source code");
    map.insert("h", "C source code");
    map.insert("rs", "Rust source code");
    map.insert("txt", "Text files");
    map.insert("doc", "Word documents");
    map.insert("docx", "Word documents");
    map.insert("odt", "Word documents");
    map.insert("xls", "Spreadsheets");
    map.insert("xlsx", "Spreadsheets");
    map.insert("ods", "Spreadsheets");
    map.insert("ppt", "Presentations");
    map.insert("pptx", "Presentations");
    map.insert("odp", "Presentations");
    map.insert("md", "Markdown files");
    map.insert("jpg", "Images");
    map.insert("jpeg", "Images");
    map.insert("png", "Images");
    map.insert("gif", "Images");
    map.insert("bmp", "Images");
    map.insert("tiff", "Images");
    map.insert("tif", "Images");
    map.insert("webp", "Images");
    map.insert("svg", "Images/Vector images");
    map.insert("mp3", "Audio files");
    map.insert("aac", "Audio files");
    map.insert("ogg", "Audio files");
    map.insert("wav", "Audio files");
    map.insert("flac", "Audio files");
    map.insert("aiff", "Audio files");
    map.insert("mp4", "Videos");
    map.insert("wmv", "Videos");
    map.insert("mov", "Videos");
    map.insert("avi", "Videos");
    map.insert("mkv", "Videos");
    map.insert("zip", "Archives");
    map.insert("rar", "Archives");
    map.insert("7z", "Archives");
    map.insert("tar", "Archives");
    map.insert("gz", "Archives");
    map.insert("bz2", "Archives");
    map.insert("xz", "Archives");
    map.insert("ttf", "Fonts");
    map.insert("otf", "Fonts");
    map.insert("woff", "Fonts");
    map.insert("woff2", "Fonts");
    map.insert("torrent", "Torrents");

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

fn wait_for_file_release(path: &PathBuf, max_wait: Duration) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < max_wait {
        match OpenOptions::new().write(true).open(path) {
            Ok(_) => return true, // File is available
            Err(_) => sleep(Duration::from_millis(10)) // Wait and retry
        }
    }
    return false; // File was not released within the time limit
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
                },
                Err(e) => {
                    println!("[Error] Could not resolve paths: {:?}", e);
                    return;
                }
            }

            ensure_new_filename(&mut path_buffer);

            wait_for_file_release(file_path, Duration::from_secs(5));
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
