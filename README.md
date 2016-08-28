# kth-kattis-rust
Checklist code to show that the rust programming language is viable
for [the Kattis code checker](https://open.kattis.com/).

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

# How submission are tested

Each subdirectory of this directory is a problem with one submission.
The files in each such directory are:
* `src` is a directory that contains the actual submitted file(s).
  Often just a single `main.rs`, but there can be multiple files.
* `in` and `correct` specifies the problem.

To compile and run the submission for a problem, cd to that problem
directory and run:

    ../test.sh

After running, the directory will also contain:
* `bin/submission` is the compiled submission.
* `out` contains the output from running the submitted program.

The script `test.sh` in this directory builds and tests a submission.
It compiles the submission, sets a few limits and then runs the
submission, comparing the output to the correct output.

Note that source files other than main.rs does not have to mentioned
on the command line (or in Cargo.toml if cargo is used).

The return code will be zero for a correct solution or non-zero
for incorrect solutions (compiler errors or incorrect output).
Any compiler errors / warnings or difference in output will be on
standard output.

## Cargo

The above mentioned `test.sh` works with raw rustc.
Thus [the stadard library](https://doc.rust-lang.org/stable/std/) but
no other librararies can be used by submissions.

To make selected libraries (e.g. [regex](https://crates.io/crates/regex))
availiable for use in submissions, a test script like `test-with-cargo.sh`
can be used.
