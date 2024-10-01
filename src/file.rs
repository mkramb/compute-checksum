use glob::Pattern;
use jwalk::WalkDir;

pub fn collect_files(dir: &str, exclude: &Vec<String>) -> Vec<String> {
    let exclude_patterns: Vec<Pattern> = exclude
        .into_iter()
        .filter_map(|p| Pattern::new(&p).ok()) // Convert the exclude strings into glob patterns
        .collect();

    let entries = WalkDir::new(&dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .filter(|e| !e.is_empty() && !exclude_patterns.iter().any(|pattern| pattern.matches(e)))
        .collect::<Vec<String>>();

    entries
}
