use crate::document::Content;
use std::sync::Arc;

pub struct Subdocument {
    content: Arc<Content>,
    indices: Vec<u32>,
}

impl Subdocument {
    pub fn new(content: Arc<Content>) -> Self {
        Self {
            content,
            indices: Vec::new(),
        }
    }
}
