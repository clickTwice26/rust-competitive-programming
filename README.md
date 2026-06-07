# Rust Competitive Programming

A clean, efficient project structure for competitive programming in Rust.

## Project Structure

This project uses Cargo Workspaces to organize problems by platform:

```
rust-competitive-programming/
├── Cargo.toml          # Root workspace config
├── .gitignore
├── templates/
│   └── template.rs     # Fast I/O template
├── codeforces/         # Codeforces problems
│   ├── Cargo.toml
│   └── src/bin/        # Each file is a standalone binary
│       └── 123a_problem_name.rs
├── leetcode/           # LeetCode problems
└── cses/               # CSES problems
```

## Usage

### Running a Problem

To run a specific problem:

```bash
cargo run --bin 123a_problem_name -p codeforces
```

To run with full optimizations (recommended for testing against constraints):

```bash
cargo run --release --bin 123a_problem_name -p codeforces < input.txt
```

### Adding a New Problem

1. Copy the template from `templates/template.rs` to the appropriate platform's `src/bin` directory
2. Name your file following the platform's convention (e.g., `codeforces/src/bin/123a_problem.rs`)
3. Write your solution in the `solve()` function
4. Run it!

## Template Features

- **Fast I/O**: Locks stdin/stdout globally and uses buffered writing
- **Token Scanner**: A convenient macro for quickly parsing input tokens
- **Clean separation**: Logic in `solve()`, I/O in `main()`
