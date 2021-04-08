use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

mod document;
mod tree;

use document::{Content, Node, Watcher};
use tree::Tree;

#[tokio::main]
async fn main() -> Result<()> {
    let content = Arc::new(RwLock::new(Content::new("example2".into())?));

    let (watcher, root_notify) = Watcher::new(content.clone());

    let root = Node::root(content.clone(), "root".into(), root_notify.clone());
    let (mut tree, root_id) = Tree::new(root);

    let child_1 = Node::new(content.clone(), "child_1".into());
    let child_1_id = tree.add_node(root_id, child_1, "line");

    let child_2 = Node::new(content.clone(), "child_2".into());
    let _child_2_id = tree.add_node(root_id, child_2, "child2");

    let child_3 = Node::new(content.clone(), "child_3".into());
    let _child_3_id = tree.add_node(child_1_id.unwrap(), child_3, "word");

    let watcher_task = tokio::spawn(async move {
        watcher.watch().await.unwrap();
    });

    tree.remove_node(child_1_id.unwrap());

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
