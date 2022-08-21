# trane-earmaster

This repository converts the exercises in EarMaster 7 to
[Trane](https://github.com/trane-project/trane) courses. This is an example of how existing
educational materials can easily be augmented with Trane. Using these courses require that you have
a working copy of EarMaster 7.

## Building

Building is not needed for most users. All the course materials are kept under the `courses/`
directory and can be used immediately.

These course materials are built by running the binary under the `src/` directory. If you want to
rebuild the courses, use the Makefile provided along by executing `make build_courses`. This command
requires that a stable version of the Rust toolchain is installed.
