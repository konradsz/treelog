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
    indices: Vec<u32>,
}

impl Subdocument {
    pub fn full_document(content: Arc<Content>) -> Self {
        Self {
            indices: (0..content.len()).map(|i| i as u32).collect(),
            content,
        }
    }

    pub fn new(content: Arc<Content>, pattern: &str) -> Result<Self> {
        let regex_matcher = RegexMatcher::new(pattern)?;
        let mut searcher = Searcher::new();

        let mut indices = Vec::new();
        for index in 0..content.len() as u32 {
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
