use crate::document::Content;
use anyhow::Result;
use grep::{
    matcher::Matcher,
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher},
};
use std::sync::Arc;

pub struct Subdocument {
    content: Arc<Content>,
    indices: Vec<usize>,
}

impl Subdocument {
    pub fn full_document(content: Arc<Content>) -> Self {
        Self {
            indices: (0..content._len()).map(|i| i).collect(),
            content,
        }
    }

    pub fn new(content: Arc<Content>, pattern: &str) -> Result<Self> {
        let regex_matcher = RegexMatcher::new(pattern)?;
        let mut searcher = Searcher::new();

        let mut indices = Vec::new();
        for index in 0..content._len() {
            searcher.search_slice(
                &regex_matcher,
                content.get_line(index).as_ref(),
                UTF8(|_, line| {
                    regex_matcher.find(line.as_bytes())?.unwrap();
                    indices.push(index);
                    Ok(true)
                }),
            )?;
        }

        Ok(Self { content, indices })
    }
}
