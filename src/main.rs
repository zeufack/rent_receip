mod cli;

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use rent_receip::config::{self, Config};
use rent_receip::receipt::{Receipt, normalize_period, period_bounds};
use rent_receip::render::{Renderer, TypstRenderer};

use cli::{Cli, Command, NewArgs};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Init { force } => config::init(force),
        Command::New(args) => new_receipt(*args),
        Command::Show => show_config(),
    }
}

fn new_receipt(args: NewArgs) -> Result<()> {
    let mut config = Config::load()?;

    let number = match &args.number {
        Some(number) => number.clone(),
        None => config.next_number(),
    };

    let rent = args.rent.unwrap_or(config.default_rent);
    let charges = args.charges.unwrap_or(config.default_charges);
    let energy = args.energy.unwrap_or(0);
    let total = rent
        .checked_add(charges)
        .and_then(|value| value.checked_add(energy))
        .context("amount is too large")?;

    let bounds = period_bounds(&args.period);
    let period_start = args
        .period_start
        .or_else(|| bounds.as_ref().map(|(start, _)| start.clone()))
        .unwrap_or_default();
    let period_end = args
        .period_end
        .or_else(|| bounds.as_ref().map(|(_, end)| end.clone()))
        .unwrap_or_default();

    let today = Local::now().format("%d/%m/%Y").to_string();
    let date = args.date.unwrap_or_else(|| today.clone());

    let receipt = Receipt {
        number,
        date,
        period: normalize_period(&args.period),
        period_start,
        period_end,
        lessor_name: config.lessor_name.clone(),
        lessor_address: config.lessor_address.clone(),
        lessee_name: args.lessee,
        lessee_address: args.lessee_address.unwrap_or_default(),
        lessee_civility: args.civility.unwrap_or_default(),
        property_address: args
            .property
            .unwrap_or_else(|| config.property_address.clone()),
        city: config.city.clone(),
        rent,
        charges,
        energy,
        total,
        currency: config.currency.clone(),
        payment_method: args.payment_method.unwrap_or_default(),
        payment_date: args.payment_date.unwrap_or(today),
    };

    let bytes = TypstRenderer::new().render(&receipt)?;

    let out = args
        .out
        .unwrap_or_else(|| PathBuf::from(receipt.default_file_name()));
    if let Some(parent) = out.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("cannot create {}", parent.display()))?;
    }
    fs::write(&out, bytes).with_context(|| format!("cannot write {}", out.display()))?;

    if args.number.is_none() {
        config.save()?;
    }

    println!("Wrote {}", out.display());
    Ok(())
}

fn show_config() -> Result<()> {
    let config = Config::load()?;
    println!("{}", Config::path()?.display());
    println!();
    print!("{}", toml::to_string_pretty(&config)?);
    Ok(())
}
