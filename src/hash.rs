use std::fs::File;
use std::io::{BufReader, Read, Result};

use rayon::prelude::*;
use sha1::{Digest, Sha1};

pub fn compute_file_hash(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    compute_hash(reader)
}

pub fn compute_hash<R: Read>(mut reader: BufReader<R>) -> Result<String> {
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn hash_files(files: Vec<String>) -> Vec<String> {
    let mut hash_lines: Vec<String> = files
        .par_iter()
        .filter_map(|path| match compute_file_hash(path) {
            Ok(sha1sum) => Some(sha1sum),
            Err(e) => {
                eprintln!("Problem parsing file: {e}");
                None
            }
        })
        .collect();

    hash_lines.sort();
    hash_lines
}
