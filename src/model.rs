use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub service: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<Entry>,
}
impl Vault {
    // Create a new empty vault
    pub fn new() -> Self {
        Vault {
            entries: Vec::new(),
        }
    }

    // Add a new entry
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    // Find an entry by service name
    pub fn find_entry(&self, service: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.service == service)
    }

    // Remove an entry by service name
    pub fn remove_entry(&mut self, service: &str) -> bool {
        let index = self.entries.iter().position(|e| e.service == service);
        if let Some(i) = index {
            self.entries.remove(i);
            true
        } else {
            false
        }
    }
}
