#! /bin/bash

set -eo pipefail
dir=$1;

# Find all files in the specified directory, excluding 'node_modules' directory,
# generate a SHA-1 hash for each file, then:

# 1. Remove the directory prefix from each file path.
# 2. Sort the output to ensure consistent ordering.
# 3. Generate a final SHA-1 hash from the sorted list of file hashes.
find "$dir" -type f -not -path "$dir/node_modules/*" \
    -exec sha1sum {} \; | sed "s~$dir~~g" | LC_ALL=C sort -d | sha1sum | awk '{print $1}'