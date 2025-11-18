rust-vault

A simple and secure CLI password manager written in Rust. Store, retrieve, and manage your passwords locally using JSON-based persistence.

Features

Add a new password entry (service, username, password)

List all saved entries

Retrieve the password for a specific service

Delete an entry

Persistent storage in a local JSON file

Beginner-friendly Rust CLI project

Installation

Clone the repository:

git clone git@github.com:yourusername/rust-vault.git
cd rust-vault


Build the project with Cargo:

cargo build --release

Usage

Run commands using Cargo:

# Add a new entry
cargo run -- add <service> <username> <password>

# List all entries
cargo run -- list

# Get password for a service
cargo run -- get <service>

# Delete an entry
cargo run -- delete <service>


Example:

cargo run -- add gmail ahmad123 mypass123
cargo run -- list
cargo run -- get gmail
cargo run -- delete gmail

Project Structure

main.rs – CLI entry point

model.rs – Defines Entry and Vault structs

storage.rs – Handles reading/writing vault to JSON

commands.rs – Implements add, list, get, delete commands

Learning Goals

Practice Rust syntax: structs, enums, Result, Option

Work with file I/O and JSON serialization (serde)

Build a functional CLI

Understand modular project structure in Rust

License

MIT License © Ahmad

If you want, I can also add a small “Future Improvements” section to make it look more professional and show planning skills in the README. Do you want me to do that?
