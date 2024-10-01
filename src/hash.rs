
use rayon::prelude::*;

pub fn compute_sha1sum(path: &String) -> Option<String> {
    Some(format!("{:?}", path))
}

pub fn hash_files(files: Vec<String>) -> Vec<String> {
    let hash_lines = files
        .par_iter()
        .map(|path| {
            match compute_sha1sum(path) {
                Some(sha1sum) => sha1sum,
                None => String::from(""),
            }
        })
        .collect();

    hash_lines
}
