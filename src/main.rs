use anyhow::Result;
use std::sync::Arc;

mod document;
mod matcher;

use document::{Content, Subdocument};

fn main() -> Result<()> {
    let content = Arc::new(Content::new("example".into())?);

    let _subdocument_1 = Subdocument::full_document(content.clone());
    let _subdocument_2 = Subdocument::new(content.clone(), "word");

    Ok(())
}
