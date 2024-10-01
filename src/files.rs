use walkdir::WalkDir;
use std::ops::Not;

pub fn collect_files(dir: &str) -> Vec<String> {
    let entries = WalkDir::new(&dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .filter(|e| e.is_empty().not())
        .collect::<Vec<String>>();

    entries
 }
