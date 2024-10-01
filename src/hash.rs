extern crate num_cpus;

use futures::stream::{self, StreamExt};
use tokio::io::{self};

pub async fn compute_sha1sum(path: &str) -> io::Result<String> {
    Ok(format!("{:?}", path))
}

pub async fn hash_files(files: Vec<String>) -> Vec<io::Result<String>> {
    let hash_lines = stream::iter(files)
        .map(|path| async move {
            match compute_sha1sum(&path).await {
                Ok(sha1sum) => Ok(sha1sum),
                Err(e) => Err(e),
            }
        })
        .buffer_unordered(num_cpus::get())
        .collect::<Vec<_>>()
        .await;

    hash_lines
}
