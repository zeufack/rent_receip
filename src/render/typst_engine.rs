use anyhow::{Context, Result, anyhow};
use typst::foundations::{Dict, IntoValue};
use typst_as_lib::TypstEngine;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;
use typst_layout::PagedDocument;
use typst_pdf::{PdfOptions, pdf};

use super::Renderer;
use crate::receipt::Receipt;

const TEMPLATE: &str = include_str!("../../assets/receipt.typ");
const MAIN_FILE: &str = "receipt.typ";

/// Renders receipts with the embedded Typst template.
///
/// The template (`assets/receipt.typ`) and its fonts are compiled into the
/// binary, so no external Typst or LaTeX installation is required.
pub struct TypstRenderer {
    engine: TypstEngine,
}

impl TypstRenderer {
    /// Builds the Typst engine, scanning the configured fonts once.
    pub fn new() -> Self {
        let engine = TypstEngine::builder()
            .with_static_source_file_resolver([(MAIN_FILE, TEMPLATE)])
            .search_fonts_with(TypstKitFontOptions::default())
            .build();
        Self { engine }
    }
}

impl Default for TypstRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for TypstRenderer {
    fn render(&self, receipt: &Receipt) -> Result<Vec<u8>> {
        let mut inputs = Dict::new();
        for (key, value) in receipt.fields() {
            inputs.insert(key.into(), value.into_value());
        }

        let warned = self.engine.compile_with_input(MAIN_FILE, inputs);
        for warning in warned.warnings {
            eprintln!("warning: {}", warning.message);
            for hint in warning.hints {
                eprintln!("  hint: {}", hint.v);
            }
        }

        let document: PagedDocument = warned
            .output
            .map_err(|err| anyhow!("{err}"))
            .context("failed to compile the receipt template")?;

        let bytes = pdf(&document, &PdfOptions::default())
            .map_err(|err| anyhow!("{err:?}"))
            .context("failed to export the receipt to PDF")?;

        Ok(bytes)
    }
}
