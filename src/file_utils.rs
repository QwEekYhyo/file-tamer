use std::{collections::HashMap, sync::Mutex};
use once_cell::sync::Lazy;

pub static EXTENSION_TO_DIRECTORY: Lazy<Mutex<HashMap<&str, &str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("pdf", "pdf documents");
    map.insert("cpp", "C++ source code");
    map.insert("hpp", "C++ source code");
    map.insert("rs", "Rust source code");
    map.insert("txt", "Text files");

    return Mutex::new(map);
});
