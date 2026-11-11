use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "rent-receip",
    version,
    about = "Generate rent receipts (quittances de loyer) as PDF",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Ask for the defaults once and store them in the config file.
    Init {
        /// Overwrite an existing config file.
        #[arg(long)]
        force: bool,
    },
    /// Generate a rent receipt PDF.
    New(Box<NewArgs>),
    /// Print the current configuration.
    Show,
}

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Billing period, e.g. `2026-11` or `Novembre 2026`.
    #[arg(long, short)]
    pub period: String,
    /// Tenant full name.
    #[arg(long)]
    pub lessee: String,
    /// Tenant postal address.
    #[arg(long)]
    pub lessee_address: Option<String>,
    /// Tenant civility, e.g. `Monsieur` or `Madame`.
    #[arg(long)]
    pub civility: Option<String>,
    /// Rent (loyer) in major currency units. Defaults to the configured amount.
    #[arg(long, alias = "amount")]
    pub rent: Option<u64>,
    /// Provision for charges in major currency units. Defaults to the configured value.
    #[arg(long)]
    pub charges: Option<u64>,
    /// Contribution to energy savings in major currency units.
    #[arg(long)]
    pub energy: Option<u64>,
    /// Property address. Defaults to the configured one.
    #[arg(long)]
    pub property: Option<String>,
    /// First day of the rental period (`dd/mm/yyyy`). Derived from `--period` when possible.
    #[arg(long)]
    pub period_start: Option<String>,
    /// Last day of the rental period (`dd/mm/yyyy`). Derived from `--period` when possible.
    #[arg(long)]
    pub period_end: Option<String>,
    /// Date of issue (`dd/mm/yyyy`), used in “Fait à …, le …”. Defaults to today.
    #[arg(long)]
    pub date: Option<String>,
    /// Payment date (`dd/mm/yyyy`). Defaults to today.
    #[arg(long)]
    pub payment_date: Option<String>,
    /// Receipt number. Defaults to the next auto-incremented value.
    #[arg(long)]
    pub number: Option<String>,
    /// Payment method, e.g. `Espèces` or `Virement`.
    #[arg(long)]
    pub payment_method: Option<String>,
    /// Output PDF path.
    #[arg(long, short)]
    pub out: Option<PathBuf>,
}
