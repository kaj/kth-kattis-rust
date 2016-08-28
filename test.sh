#! /bin/bash
# Execute this script from each of the subdirectories.

set -e
mkdir -p bin
rustc src/main.rs --crate-type bin -C opt-level=3 -o bin/submission

ulimit -t 3
ulimit -d 16000
ulimit -s 1024
ulimit -m 4096
ulimit -v 100000
#ulimit -a

./bin/submission < in > out && diff correct out
