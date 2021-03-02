use anyhow::Result;
use std::sync::Arc;

mod document;

use document::{Content, Subdocument};

fn main() -> Result<()> {
    let content = Arc::new(Content::new("example".into())?);
    let subdocument_1 = Subdocument::full_document(content.clone());
    let subdocument_2 = Subdocument::new(content.clone());

    Ok(())
}
