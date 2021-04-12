use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

mod content;
mod document;
mod matcher;
mod tree;
mod watcher;

use content::Content;
use document::Document;
use tree::{Node, Tree};
use watcher::Watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let content = Arc::new(RwLock::new(Content::new("example2".into())?));

    let (watcher, indices, root_notify) = Watcher::new(content.clone());

    let mut root = Document::root(content.clone(), "root".into(), root_notify.clone());
    root.set_parent_indices(indices);

    let (mut tree, root_id) = Tree::new(root);

    let child_1 = Document::new(content.clone(), "child_1".into());
    let child_1_id = tree.add_node(root_id, child_1, "line");

    let child_2 = Document::new(content.clone(), "child_2".into());
    let _child_2_id = tree.add_node(root_id, child_2, "child2");

    let child_3 = Document::new(content.clone(), "child_3".into());
    let _child_3_id = tree.add_node(child_1_id.unwrap(), child_3, "word");

    let watcher_task = tokio::spawn(async move {
        watcher.watch().await.unwrap();
    });

    //tree.remove_node(child_1_id.unwrap());

    /*let c = content.clone();
    let sleep_task = tokio::spawn(async move {
        loop {
            let read_lock = c.read().await;
            println!("read {} lines", read_lock.len());
            drop(read_lock);

            sleep(Duration::from_millis(1_000)).await;
        }
    });*/

    watcher_task.await?;

    Ok(())
}
