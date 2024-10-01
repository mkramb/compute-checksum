use walkdir::WalkDir;

pub fn collect_files(dir: &str) -> Vec<String> {
    let entries = WalkDir::new(&dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .collect::<Vec<String>>();

    entries
 }
