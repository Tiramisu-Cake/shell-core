 Minimal Unix Shell (Rust)

This repository contains a small Unix shell implementation written in Rust.

The project was built as part of the  
[Codecrafters “Build Your Own Shell” challenge](https://app.codecrafters.io/courses/shell/overview)  
and focuses on implementing the core mechanics of how a Unix shell executes commands.

---

## What this shell does

The shell supports the essential features needed to run Unix commands:

- parsing command lines into commands and arguments  
- running external programs using `fork` and `exec`  
- connecting commands with pipelines (`|`)  
- handling input and output redirection (`<`, `>`, `>>`)  
- basic quoting and environment variable expansion  

The goal of the project is not to replicate a full-featured shell like Bash, but to understand and implement how command execution works at the process and file-descriptor level.

---

## How it works (high level)

At a high level, the shell operates in two steps:

1. **Parsing input**  
   A command line is read and converted into a structured representation describing commands, pipelines, and redirections.

2. **Executing commands**  
   The shell creates processes, sets up pipes and file descriptor redirections, and executes programs using standard Unix system calls.

Parsing and execution are kept separate to keep the code easier to follow and reason about.

---

## Testing

Correctness is validated using the official Codecrafters test suite.

The tests treat the shell as a black box:
- commands are executed,
- output and exit codes are captured,
- behavior is compared against expected shell semantics.

This ensures that the implementation behaves correctly from the user’s point of view, without enforcing a specific internal architecture.

---

## Out of scope

This project focuses on core command execution and process management.  
The following features are outside the scope of the implementation:

- job control (background processes, `fg` / `bg`, signal forwarding)  
- interactive terminal features (line editing, history, prompt customization)  
- advanced shell scripting features  
- full POSIX or Bash compatibility  

These areas are largely orthogonal to the execution model explored in this project.

---

## Motivation

Writing a shell is a practical way to explore how Unix process management works in practice:
process creation, pipes, file descriptors, and error handling.

This project was primarily an exploration of those low-level mechanisms, implemented in Rust with an emphasis on clarity and correctness.

---

## Running the shell

The shell can be built and run locally using Cargo:

```bash
cargo run

