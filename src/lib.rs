//! Generate rent receipts (*quittances de loyer*) as PDF files.
//!
//! The crate is split into three layers:
//! - [`config`] loads and saves the user defaults,
//! - [`receipt`] holds the data model and French formatting helpers,
//! - [`render`] turns a [`receipt::Receipt`] into PDF bytes through a
//!   pluggable [`render::Renderer`].
//!
//! The Typst template and its fonts are embedded in the binary, so rendering
//! does not need any external tooling.

#![forbid(unsafe_code)]

pub mod config;
pub mod receipt;
pub mod render;
