# rust-vault

A simple and secure CLI password manager written in Rust. rust-vault allows you to store, retrieve, and manage your passwords locally using JSON-based persistence. It’s a beginner-friendly Rust project ideal for learning CLI development, structs, enums, and file I/O.

## Table of Contents

Features

Installation

Usage

Project Structure

Learning Goals

Future Improvements

License

## Features

Add a new password entry with service, username, and password.

List all saved entries.

Retrieve the password for a specific service.

Delete an entry from the vault.

Persistent storage in a local JSON file.

Beginner-friendly CLI application written in Rust.

## Installation

Clone the repository:

git clone git@github.com:yourusername/rust-vault.git
cd rust-vault


Build the project using Cargo:

cargo build --release

Usage

Run commands via Cargo. Examples:

## Add a new entry
cargo run -- add <service> <username> <password>

## List all entries
cargo run -- list

## Get password for a service
cargo run -- get <service>

## Delete an entry
cargo run -- delete <service>


Example workflow:

cargo run -- add gmail ahmad123 mypass123
cargo run -- list
cargo run -- get gmail
cargo run -- delete gmail

## Learning Goals

Practice Rust syntax: structs, enums, Result, Option.

Work with file I/O and JSON serialization (serde).

Build a functional CLI application.

Understand modular project structure in Rust.

Future Improvements

Add encryption for secure password storage.

Interactive CLI input (prompt for password instead of passing in command).

Search or filter entries by service name.

Export/import vault for backup purposes.

Improved error handling and user experience.

License

MIT License © Ahmad

If you want, I can also make an even shorter, recruiter-friendly version that highlights skills and project features in one page for your GitHub portfolio. Do you want me to do that?
