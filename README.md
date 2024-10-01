# Compute checksum

- We can calculate the checksum for a given path
- For each file, we generate a SHA-1 hash
- Lastly, we combine and hash all of those

Both processes for collecting files and generating hashes are executed in parallel.

### Usage

Start with help:

```
./compute_checksum --help
Usage: compute_checksum [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>
  -e, --exclude <EXCLUDE>
  -h, --help               Print help
  -V, --version            Print version
```

Example:

```
./compute_checksum --path ./target
./compute_checksum --path ./target --exclude "*/build" --exclude "*/deps"
```

### Development

```
cargo run
cargo build
cargo build --release
```

### Testing

Let’s compare it with the equivalent in Bash by cloning `git@github.com:rust-lang/rust.git` to use as input:

```
1m 9.18s >> ./compute_checksum.sh ../rust/
   2.67s >> ./compute_checksum --path ../rust/ --exclude "*/node_modules" (2.67s)
```
