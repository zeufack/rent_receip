use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Persistent defaults written by `init` and read by `new`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub lessor_name: String,
    #[serde(default)]
    pub lessor_address: String,
    #[serde(default)]
    pub property_address: String,
    #[serde(default)]
    pub city: String,
    /// Default monthly rent (historically stored as `default_amount`).
    #[serde(alias = "default_amount")]
    pub default_rent: u64,
    #[serde(default)]
    pub default_charges: u64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default = "default_prefix")]
    pub receipt_prefix: String,
    #[serde(default = "default_next_number")]
    pub next_receipt_number: u64,
}

fn default_currency() -> String {
    "FCFA".to_string()
}

fn default_prefix() -> String {
    "Q".to_string()
}

fn default_next_number() -> u64 {
    1
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("io", "rent-receip", "rent-receip")
            .context("cannot determine the user config directory")?;
        Ok(dirs.config_dir().join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        let text = fs::read_to_string(&path).with_context(|| {
            format!(
                "cannot read config at {} — run `rent-receip init` first",
                path.display()
            )
        })?;
        toml::from_str(&text).with_context(|| format!("invalid config file at {}", path.display()))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("cannot create {}", parent.display()))?;
        }
        let text = toml::to_string_pretty(self).context("cannot serialize config")?;
        fs::write(&path, text).with_context(|| format!("cannot write {}", path.display()))?;
        Ok(())
    }

    pub fn next_number(&mut self) -> String {
        let number = format!("{}-{:04}", self.receipt_prefix, self.next_receipt_number);
        self.next_receipt_number += 1;
        number
    }
}

pub fn init(force: bool) -> Result<()> {
    let path = Config::path()?;
    if path.exists() && !force {
        bail!(
            "config already exists at {} — pass --force to overwrite it",
            path.display()
        );
    }

    println!("Configuring rent-receip (press Enter to accept a default).\n");

    let lessor_name = prompt_required("Lessor full name")?;
    let lessor_address = prompt("Lessor address", Some(""))?;
    let property_address = prompt("Rented property address", Some(""))?;
    let city = prompt("City (used on the receipt)", Some(""))?;

    let default_rent = loop {
        let raw = prompt("Default monthly rent (in major currency units)", None)?;
        match raw.replace([' ', '_'], "").parse::<u64>() {
            Ok(value) => break value,
            Err(_) => println!("Please enter a whole number, e.g. 350000."),
        }
    };

    let default_charges = loop {
        let raw = prompt("Default monthly charges (0 if none)", Some("0"))?;
        match raw.replace([' ', '_'], "").parse::<u64>() {
            Ok(value) => break value,
            Err(_) => println!("Please enter a whole number, e.g. 15000."),
        }
    };

    let currency = prompt("Currency", Some("FCFA"))?;
    let receipt_prefix = prompt("Receipt number prefix", Some("Q"))?;

    let config = Config {
        lessor_name,
        lessor_address,
        property_address,
        city,
        default_rent,
        default_charges,
        currency,
        receipt_prefix,
        next_receipt_number: 1,
    };
    config.save()?;

    println!("\nSaved config to {}", path.display());
    Ok(())
}

fn prompt(label: &str, default: Option<&str>) -> Result<String> {
    match default {
        Some(value) => print!("{label} [{value}]: "),
        None => print!("{label}: "),
    }
    io::stdout().flush().context("cannot flush stdout")?;

    let mut line = String::new();
    let read = io::stdin()
        .read_line(&mut line)
        .context("cannot read from stdin")?;
    if read == 0 {
        bail!("unexpected end of input; `init` needs answers on stdin");
    }

    let value = line.trim();
    if value.is_empty() {
        Ok(default.unwrap_or("").to_string())
    } else {
        Ok(value.to_string())
    }
}

fn prompt_required(label: &str) -> Result<String> {
    loop {
        let value = prompt(label, None)?;
        if !value.is_empty() {
            return Ok(value);
        }
        println!("This field is required.");
    }
}
