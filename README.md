# Compute checksum

- We can calculate the checksum for a given path
- For each file, we generate a SHA-1 hash
- Lastly, we combine and hash all of those

Both processes for collecting files and generating hashes are executed in parallel.

## Usage

```
./target/debug/compute_checksum --path ./target
./target/debug/compute_checksum --path ./target --exclude '*.e2e-spec.ts' --exclude '*.spec.ts'
```

## Development

```
cargo run
cargo build
cargo build --release
```

## Testing

Let’s compare it with the equivalent in Bash by cloning `git@github.com:rust-lang/rust.git` to use as input:

```
1m 9.18s >> ./compute_checksum.sh ../rust/
   2.67s >> ./compute_checksum --path ../rust/ --exclude "node_modules/* (2.67s)
```
