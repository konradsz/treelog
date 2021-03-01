use anyhow::Result;
use document::Subdocument;
use std::sync::Arc;

mod document;

fn main() -> Result<()> {
    let content = Arc::new(document::Content::new("example".into())?);
    let subdocument_1 = document::Subdocument::new(content.clone());
    let subdocument_2 = document::Subdocument::new(content.clone());

    Ok(())
}
