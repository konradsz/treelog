use anyhow::Result;
use matcher::{PassthroughMatcher, PatternMatcher};
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;

mod content;
mod document;
mod matcher;
mod tree;
mod watcher;

use content::TextContent;
use document::Document;
use tree::{Node, Tree};
use watcher::Watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let content = Arc::new(RwLock::new(TextContent::new()));

    let (watcher, indices, root_rx) = Watcher::new(content.clone());

    let mut root = Document::new(
        content.clone(),
        Arc::new(PassthroughMatcher {}),
        "root".into(),
    );
    // let mut root = Document::new(content.clone(), PatternMatcher::new("")?, "root".into());
    root.set_parent_indices(indices);

    let (mut tree, root_id) = Tree::new(root, root_rx.clone());

    let child_1 = Document::new(
        content.clone(),
        Arc::new(PatternMatcher::new("line")?),
        "child_1".into(),
    );
    let child_1_id = tree.add_node(root_id, child_1);

    let child_2 = Document::new(
        content.clone(),
        Arc::new(PatternMatcher::new("child2")?),
        "child_2".into(),
    );
    let _child_2_id = tree.add_node(root_id, child_2);

    let child_3 = Document::new(
        content.clone(),
        Arc::new(PatternMatcher::new("word")?),
        "child_3".into(),
    );
    let _child_3_id = tree.add_node(child_1_id.unwrap(), child_3);

    let watcher_task = tokio::spawn(async move {
        watcher.watch("example2").await.unwrap();
    });

    // tree.remove_node(child_1_id.unwrap());

    /*let c = content.clone();
    let sleep_task = tokio::spawn(async move {
        loop {
            let read_lock = c.read().await;
            println!("read {} lines", read_lock.len());
            drop(read_lock);

            sleep(Duration::from_millis(1_000)).await;
        }
    });*/
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let child_4 = Document::new(
            content.clone(),
            Arc::new(PatternMatcher::new("line1").unwrap()),
            "child_4".into(),
        );
        let _child_4_id = tree.add_node(_child_3_id.unwrap(), child_4);
        tokio::time::sleep(Duration::from_millis(1000)).await;
    });

    watcher_task.await?;

    Ok(())
}
