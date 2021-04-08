use anyhow::Result;
use grep::{
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher},
};

pub trait Matcher {
    fn matches(&mut self, line: &str) -> bool;
}

pub struct PatternMatcher {
    regex_matcher: RegexMatcher,
}

impl PatternMatcher {
    pub fn new(pattern: &str) -> Result<Self> {
        Ok(Self {
            regex_matcher: RegexMatcher::new(pattern)?,
        })
    }
}

impl Matcher for PatternMatcher {
    fn matches(&mut self, line: &str) -> bool {
        let mut found = false;
        Searcher::new()
            .search_slice(
                &self.regex_matcher,
                line.as_ref(),
                UTF8(|_lnum, _line| {
                    found = true;
                    Ok(true)
                }),
            )
            .unwrap();
        found
    }
}

pub struct PassthroughMatcher();

impl Matcher for PassthroughMatcher {
    fn matches(&mut self, _line: &str) -> bool {
        true
    }
}