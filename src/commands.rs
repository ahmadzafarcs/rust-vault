// commands.rs
use crate::model::{Entry, Vault};
use crate::storage::{load_vault, save_vault};
use std::io::{self, Write};

const VAULT_FILE: &str = "vault.json";

// Add a new entry
pub fn add(service: &str, username: &str, password: &str) {
    let mut vault = load_vault(VAULT_FILE);
    let entry = Entry {
        service: service.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    };
    vault.add_entry(entry);
    save_vault(&vault, VAULT_FILE).expect("Failed to save vault");
    println!("Entry added for {}", service);
}

// List all entries
pub fn list() {
    let vault = load_vault(VAULT_FILE);
    if vault.entries.is_empty() {
        println!("Vault is empty");
    } else {
        for entry in vault.entries {
            println!("- {} ({})", entry.service, entry.username);
        }
    }
}

// Get a password for a service
pub fn get(service: &str) {
    let vault = load_vault(VAULT_FILE);
    match vault.find_entry(service) {
        Some(entry) => println!("Password for {}: {}", service, entry.password),
        None => println!("No entry found for {}", service),
    }
}

// Delete an entry
pub fn delete(service: &str) {
    let mut vault = load_vault(VAULT_FILE);
    if vault.remove_entry(service) {
        save_vault(&vault, VAULT_FILE).expect("Failed to save vault");
        println!("Deleted entry for {}", service);
    } else {
        println!("No entry found for {}", service);
    }
}
