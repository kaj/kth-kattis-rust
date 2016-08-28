#! /bin/bash
# Execute this script from each of the subdirectories.

cat > Cargo.toml <<EOF
[package]
name = "submission"
version = "0.1.0"

[dependencies]
regex = "*"
EOF

set -e
cargo build --quiet --release

ulimit -t 3
ulimit -d 16000
ulimit -s 1024
ulimit -m 4096
ulimit -v 100000
#ulimit -a

./target/release/submission < in > out && diff correct out
