use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_files(dir: &str) -> Option<Vec<PathBuf>> {
    let mut entries = vec![];
    let walker = WalkDir::new(&dir).into_iter();

    for entry in walker {
        let entry = entry.ok()?;

        if entry.file_type().is_file() {
            let path = entry.path();
            entries.push(path.to_path_buf());
        }
    }

    return Some(entries);
}
