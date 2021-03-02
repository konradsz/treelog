use crate::document::Content;
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

    pub fn new(content: Arc<Content>) -> Self {
        Self {
            content,
            indices: Vec::new(),
        }
    }
}
