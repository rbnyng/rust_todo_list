# Rust Todo List App

A simple todo list application built with Rust, using `eframe` for the GUI and `rfd` for file persistence management.

<img src="img/img.png?raw=true" alt="Title" style="width: 50%; height: auto;"/>

## Features

- **Add, Edit, and Delete Tasks**: Manage tasks with interactive buttons.
- **Persistence**: Save and load tasks from a JSON file.
- **Dark/Light Mode**: Adjust the theme.
- **Resizable Text**: Customize the UI text size for better readability.

## Usage

- **Adding a Task**: Click the `➕` button and enter the task. Press the green checkmark to add the task to the list.
- **Editing a Task**: Click the `Edit` button next to a task to modify its description. Press the `✔` button to save changes.
- **Deleting a Task**: Click the `❌` button to remove a task from the list.
- **Saving/Loading**: Use the `Save` and `Load` buttons to save the tasks to a file or load them from an existing file.

## Installation

To build this project from source, you need to have Rust and Cargo installed on your machine. If you haven't installed Rust, follow the [official installation guide](https://www.rust-lang.org/tools/install).

1. Clone the repository:

```bash
git clone https://yourrepositoryurl.com
cd rust-todo-list-app
```

2. Build and run the project:

```bash
cargo build --release
```

## Dependencies

This project uses several crates:

- `egui` for the GUI.
- `eframe` as the framework to run `egui`.
- `serde` and `serde_json` for serialization and deserialization of the todo items.
- `rfd` for opening file dialogues to save and load todo lists.
- `windows`

Specific versions of the crates used are:

```toml
[dependencies]
egui = "0.26.0"
eframe = { version = "0.26.0", default-features = false, features = ["default_fonts", "glow", "persistence"] }
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.114"
rfd = "0.14.0"
windows = "0.54.0"
```
