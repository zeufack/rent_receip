use anyhow::Result;

use crate::receipt::Receipt;

/// Turns a receipt into a PDF byte stream.
///
/// Keeping this behind a trait means the rendering engine can be swapped
/// (e.g. Typst today, a direct PDF writer or PDF form overlay later) without
/// touching the CLI or the data model.
pub trait Renderer {
    fn render(&self, receipt: &Receipt) -> Result<Vec<u8>>;
}

pub mod typst_engine;

pub use typst_engine::TypstRenderer;
