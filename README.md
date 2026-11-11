# rent-receip

Generate rent receipts (*quittances de loyer*) as PDF files from the command line.

The receipt is rendered from an embedded Typst template, so there is no LaTeX or
Typst installation to manage: the template and its fonts are compiled into the
binary. The layout follows the French ANIL model for an unfurnished or furnished
primary residence.

## Features

- Interactive first-run setup with persisted defaults
- One command per receipt, with sensible defaults for rent and charges
- Amounts written out in French words
- A4 PDF output, matching the official quittance layout
- Receipt numbers, issue date, payment date and rental period are configurable
- No external rendering tools required

## Requirements

- Rust 1.92 or newer (edition 2024)
- A font cache is not required; the embedded fonts are used automatically

## Install

From a checkout:

```bash
cargo build --release
./target/release/rent-receip --help
```

Or install it into `~/.cargo/bin`:

```bash
cargo install --path .
rent-receip --help
```

> The Cargo package is named `rent_receip` (underscores) while the installed
> binary is `rent-receip` (hyphens).

## Quick start

Run the setup once. It stores your defaults in the platform config directory:

```bash
rent-receip init
```

Generate a receipt for a given month:

```bash
rent-receip new --period 2026-11 --lessee "Marie Kamga"
```

This writes `quittance-novembre-2026-marie-kamga.pdf` using the configured rent
and charges.

A more complete example:

```bash
rent-receip new \
  --period 2026-11 \
  --lessee "Amina Traoré" --civility "Madame" \
  --lessee-address "12 rue des Lilas, 75011 Paris" \
  --rent 620 --charges 80 --energy 0 \
  --period-start 01/11/2026 --period-end 30/11/2026 \
  --date 02/11/2026 --payment-date 02/11/2026 \
  --out quittance.pdf
```

Show the current configuration and its path:

```bash
rent-receip show
```

## Options

Run `rent-receip new --help` for the full list. Useful flags:

| Flag | Meaning |
| --- | --- |
| `--period` | Billing period, e.g. `2026-11` or `Novembre 2026` |
| `--lessee`, `--civility`, `--lessee-address` | Tenant details |
| `--rent`, `--charges`, `--energy` | Amounts in major currency units |
| `--property` | Override the rented property address |
| `--period-start`, `--period-end` | Rental period bounds (`dd/mm/yyyy`) |
| `--date`, `--payment-date` | Issue and payment dates (`dd/mm/yyyy`) |
| `--number` | Explicit receipt number (disables auto-increment for this run) |
| `--payment-method` | Optional payment method shown on the receipt |
| `--out`, `-o` | Output PDF path |

`--amount` is accepted as an alias for `--rent`.

## Configuration

`rent-receip init` writes a TOML file (the exact path is printed by
`rent-receip show`, typically `~/.config/rent-receip/config.toml` on Linux):

```toml
lessor_name = "Jean Dupont"
lessor_address = "12 rue des Palmiers"
property_address = "Appartement 3, immeuble Les Palmiers, Akwa"
city = "Douala"
default_rent = 350000
default_charges = 25000
currency = "FCFA"
receipt_prefix = "Q"
next_receipt_number = 1
```

You can edit it directly, or re-run `rent-receip init --force` to overwrite it.
Per-run flags override the stored defaults without changing the file.

## How it works

- `src/config.rs` loads and saves the user defaults.
- `src/receipt.rs` holds the data model, French amount-to-words conversion and
  period helpers.
- `src/render/` defines a `Renderer` trait with a Typst implementation.
- `assets/receipt.typ` is the receipt layout, embedded with `include_str!`.

To customise the document, edit `assets/receipt.typ` and rebuild. Because the
template is embedded, the binary stays self-contained.

## Development

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

## Uninstall

```bash
cargo uninstall rent_receip
rm -rf ~/.config/rent-receip   # optional: remove saved settings
```

## License

MIT. See [LICENSE](LICENSE).
