mod models;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::PathBuf;

pub fn get_db_path() -> Result<PathBuf> {
    if let Ok(db_path) = std::env::var("CCSWITCH_DB_PATH") {
        return Ok(PathBuf::from(db_path));
    }
    let home = dirs::home_dir().context("Cannot find home directory")?;
    Ok(home.join(".cc-switch/cc-switch.db"))
}

pub fn connect() -> Result<Connection> {
    let path = get_db_path();
    let path = match path {
        Ok(p) => p,
        Err(_) => {
            anyhow::bail!("Database not found. Please install and run CC Switch first.");
        }
    };

    if !path.exists() {
        anyhow::bail!(
            "Database not found at {:?}. Please install and run CC Switch first.",
            path
        );
    }

    Connection::open(&path).context("Failed to open database")
}

pub use models::*;
