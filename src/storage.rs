// storage.rs
use crate::model::Vault;
use serde_json;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn save_vault(vault: &Vault, path: &str) -> Result<()> {
    let data = serde_json::to_string_pretty(vault).unwrap();
    fs::write(path, data)?;
    Ok(())
}

pub fn load_vault(path: &str) -> Vault {
    if Path::new(path).exists() {
        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    } else {
        Vault::new()
    }
}
