# kth-kattis-rust
Checklist code to show that the rust programming language is viable for Kattis code checker at KTH.

##  Submissions that do the following:
1. Hello world
2. Hello world implemented in at least 2 files
3. stack-based MLE (Memory Limit Exceeded - Stack overflow)
4. heap-based MLE (Memory Limit Exceeded)
5. CE (Compile error)
6. Try to open and print out /etc/passwd
7. Any interesting RTEs that should be reported back specifically to the (RTE = Run Time Error) user as extrainfo.  Three examples are given, they should probably all be reported simply as "program panicked" to the user.
8. Some other RTE
9. TLE (Time limit exceeded)
10. AC solution to different (AC = Accepted)

Each submission is a directory, to compile and run a submission, cd to
that directory and run:

    cargo run

Before attempting to run 4-heap-overflow, it might be a good idea to
set a smallish memory limit.

## Kattis-style testing

The actual submission to a problem are the file(s) in each src
directory.  The Cargo.toml is the same for all problems, and could be
provided, togehter with input and correct output, by Kattis. If Kattis
then executes the command line:

    cargo build -q && ./target/debug/submission < in > out && diff correct out

Or, with optimization enabled:

    cargo build -q --release && ./target/release/submission < in > out && diff correct out

Or, without cargo with optimization:

    rustc src/main.rs --crate-type bin -C opt-level=3 && ./main < in > out && diff correct out

Note that source files other than main.rs does not have to mentioned
on the command line (or in Cargo.toml if cargo is used).

The return code will be zero for a correct solution or non-zero
incorrect solutions (compiler errors or incorrect output).  Any
compiler errors / warnings or difference in output will be on standard
output.

There is also a script, `test.sh`, that compiles, sets some limits,
and executes a submission.
It can be called from each subdirectory to get the expected results.
