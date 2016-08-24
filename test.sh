#! /bin/bash
# Execute this script from each of the subdirectories.

set -e
cargo build --release

ulimit -t 3
ulimit -d 16000
ulimit -s 1024
ulimit -m 4096
ulimit -v 100000
#ulimit -a

./target/release/submission < in > out && diff correct out
