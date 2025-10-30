# ✅ Todo CLI

A simple and elegant command-line todo list manager written in Rust.

Perfect for learning Rust fundamentals! 🦀

---

## 🎯 What You'll Learn

This project teaches you:

- ✅ **Structs & Enums**: Organizing data with custom types
- ✅ **Methods (impl blocks)**: Adding behavior to types
- ✅ **Error Handling**: Using `Result<T, E>` and the `?` operator
- ✅ **File I/O**: Reading and writing JSON files
- ✅ **Collections**: Working with `Vec<T>` (vectors)
- ✅ **Iterators**: Using `filter()`, `find()`, `map()`
- ✅ **Pattern Matching**: `match` and `if let`
- ✅ **CLI Parsing**: Building user-friendly command-line tools with Clap
- ✅ **Serialization**: Converting Rust types to/from JSON with Serde

---

## 🚀 Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/whispem/todo-cli
cd todo-cli
```

### 2. Build and run

```bash
# Build the project
cargo build --release

# Run with cargo
cargo run -- add "Learn Rust"
cargo run -- list

# Or use the binary directly
./target/release/todo add "My first task"
./target/release/todo list
```

### 3. Install globally (optional)

```bash
cargo install --path .
```

Now you can use `todo` from anywhere!

---

## 📖 Usage Guide

### ➕ Add a task

```bash
todo add "Write Rust documentation"
todo add "Learn about ownership and borrowing"
todo add "Build a web server"
```

**Output:**
```
✓ Task #1 added: Write Rust documentation
```

---

### 📋 List tasks

```bash
# List ALL tasks (todo + done)
todo list
```

**Output:**
```
All Tasks:

[1] ☐ Write Rust documentation
[2] ☐ Learn about ownership and borrowing
[3] ☑ Build a web server
```

```bash
# List only PENDING tasks
todo list --todo
```

**Output:**
```
Pending Tasks:

[1] ☐ Write Rust documentation
[2] ☐ Learn about ownership and borrowing
```

```bash
# List only COMPLETED tasks
todo list --done
```

**Output:**
```
Completed Tasks:

[3] ☑ Build a web server
```

---

### ✅ Mark a task as done

```bash
todo done 1
```

**Output:**
```
✓ Task #1 marked as done!
```

---

### ↩️ Undo (mark as todo again)

```bash
todo undone 1
```

**Output:**
```
✓ Task #1 marked as todo.
```

---

### 🗑️ Remove a task

```bash
todo remove 2
```

**Output:**
```
✓ Task #2 removed.
```

---

### 🧹 Clear all completed tasks

```bash
todo clear
```

**Output:**
```
✓ Cleared 5 completed task(s).
```

---

## 💾 How Data is Stored

Tasks are saved in **`tasks.json`** in the current directory.

### Example `tasks.json` file:

```json
{
  "tasks": [
    {
      "id": 1,
      "description": "Learn Rust ownership",
      "status": "Done"
    },
    {
      "id": 2,
      "description": "Build a CLI app",
      "status": "Todo"
    }
  ],
  "next_id": 3
}
```

You can:
- ✅ Edit this file manually if needed
- ✅ Share it with others
- ✅ Backup it to cloud storage
- ✅ Version control it with Git

---

## 🎯 Real-World Example Workflow

```bash
# Monday morning: plan your week
todo add "Review pull requests"
todo add "Write unit tests for auth module"
todo add "Update API documentation"
todo add "Fix bug #127"
todo add "Team meeting at 2pm"

# Check what's pending
todo list --todo

# As you work, mark tasks done
todo done 1
todo done 2

# End of day: see what you accomplished
todo list --done

# Friday: clean up completed tasks
todo clear
```

---

## 🛠️ Code Architecture

The entire app is in **one file** (`src/main.rs`) with 5 clear sections:

### 1️⃣ Imports
```rust
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
```

### 2️⃣ Data Types
```rust
enum TaskStatus { Todo, Done }

struct Task {
    id: usize,
    description: String,
    status: TaskStatus,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}
```

### 3️⃣ Business Logic (impl blocks)
```rust
impl Task { ... }      // Methods for a single task
impl TodoList { ... }  // Methods for managing all tasks
```

### 4️⃣ CLI Interface
```rust
#[derive(Parser)]
struct Cli { ... }

enum Commands {
    Add { ... },
    List { ... },
    Done { ... },
    // etc.
}
```

### 5️⃣ Main Function
```rust
fn main() {
    // 1. Parse CLI arguments
    // 2. Load tasks from JSON
    // 3. Execute command
    // 4. Save tasks to JSON
}
```

---

## 🔍 Key Rust Concepts Explained

### 🟢 Enum (TaskStatus)

```rust
enum TaskStatus {
    Todo,
    Done,
}
```

An **enum** can only be one of its variants. Perfect for modeling states!

### 🟢 Struct (Task)

```rust
struct Task {
    id: usize,
    description: String,
    status: TaskStatus,
}
```

A **struct** groups related data together. Like a class in other languages.

### 🟢 impl Block (Methods)

```rust
impl Task {
    fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Todo,
        }
    }
}
```

**impl** adds methods (functions) to a type.

### 🟢 Result & Error Handling

```rust
fn load() -> io::Result<Self> {
    let contents = fs::read_to_string(path)?;
    //                                      ^
    //                  This ? means: "if error, return it immediately"
    Ok(todo_list)
}
```

**Result<T, E>** forces you to handle errors explicitly. The `?` operator is syntactic sugar.

### 🟢 Iterators

```rust
// Find a task by ID
self.tasks.iter_mut().find(|t| t.id == id)

// Filter tasks by status
self.tasks.iter().filter(|t| t.status == TaskStatus::Todo)
```

**Iterators** are lazy and composable. Super powerful!

### 🟢 Pattern Matching

```rust
match cli.command {
    Commands::Add { description } => { ... }
    Commands::List { todo, done } => { ... }
    Commands::Done { id } => { ... }
}
```

**match** is like switch/case but exhaustive (you must handle all cases).

---

## 📚 Dependencies Explained

### 🔹 Clap (CLI parsing)

```toml
clap = { version = "4.5", features = ["derive"] }
```

**Why?** Automatically parses command-line arguments.

**Example:**
```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

Generates code to handle `todo add "task"`, `todo list`, etc.

---

### 🔹 Serde (Serialization)

```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Why?** Converts Rust structs ↔ JSON.

**Example:**
```rust
#[derive(Serialize, Deserialize)]
struct Task { ... }

// Rust → JSON
let json = serde_json::to_string(&task)?;

// JSON → Rust
let task: Task = serde_json::from_str(&json)?;
```

---

### 🔹 Colored (Terminal colors)

```toml
colored = "2.1"
```

**Why?** Makes output pretty!

**Example:**
```rust
println!("{}", "Success!".green().bold());
println!("{}", "Error!".red());
```

---

## 💡 Ideas for Improvement

Want to level up this project? Try adding:

- [ ] **Due dates** (`todo add "Task" --due 2024-12-31`)
- [ ] **Priorities** (high, medium, low)
- [ ] **Tags/Categories** (`#work`, `#personal`)
- [ ] **Edit task descriptions** (`todo edit 1 "New description"`)
- [ ] **Search** (`todo search "rust"`)
- [ ] **Export to Markdown** (`todo export > tasks.md`)
- [ ] **Undo last action** (using a history stack)
- [ ] **Statistics** (total tasks, completion rate)
- [ ] **Multiple lists** (`todo list work`, `todo list personal`)
- [ ] **Sync with GitHub Issues or Todoist API**

---

## 🐛 Troubleshooting

### Problem: "command not found: todo"

**Solution:** Either:
1. Use `cargo run -- add "task"` instead
2. Or install globally: `cargo install --path .`
3. Or use the full path: `./target/release/todo add "task"`

---

### Problem: "failed to parse JSON"

**Solution:** Delete `tasks.json` and start fresh:
```bash
rm tasks.json
todo add "First task"
```

---

### Problem: Tasks not saving

**Solution:** Check file permissions:
```bash
ls -la tasks.json
chmod 644 tasks.json
```

---

## 📝 License

MIT License - feel free to use this for learning!

---

## 🤝 Contributing

Found a bug or want to add a feature? Pull requests are welcome!

---

## 🎓 Learning Resources

Want to learn more Rust?

- 📖 [The Rust Book](https://doc.rust-lang.org/book/)
- 🎥 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 🦀 [Rustlings](https://github.com/rust-lang/rustlings) (interactive exercises)
- 💬 [Rust Discord Community](https://discord.gg/rust-lang)

---

**Happy coding! 🚀**

Built with ❤️ and 🦀 by [whispem](https://github.com/whispem)
