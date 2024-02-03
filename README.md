

```markdown
# Rusty Journal

**A Command-Line To-Do App Written in Rust**

Rusty Journal is a lightweight command-line to-do application written in Rust. It offers a simple and efficient way to manage your tasks right from the terminal.

## Usage
```sh
# Add a task
cargo run -- -j test-journal.json add "buy milk"

# List all tasks
cargo run -- -j test-journal.json list

# Complete a task
cargo run -- -j test-journal.json done 2
```

## Features
- **Add Tasks:** Quickly add new tasks with ease.
- **Complete Tasks:** Mark tasks as done to track your progress.
- **List Tasks:** View all tasks with their current status.

## Custom Journal File
You can specify a custom journal file using the `-j` or `--journal-file` option.

## Getting Started
1. Clone the repository.
2. Build the project using `cargo build`.
3. Run the application with `cargo run`.

## Example Workflow
```sh
# Add tasks
cargo run -- -j test-journal.json add "buy milk"
cargo run -- -j test-journal.json add "take the dog for a walk"
cargo run -- -j test-journal.json add "water the plants"

# List tasks
cargo run -- -j test-journal.json list

# Complete a task
cargo run -- -j test-journal.json done 2

# List updated tasks
cargo run -- -j test-journal.json list
`
Feel free to contribute, report issues, or suggest enhancements. Happy journaling!
`

