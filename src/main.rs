use anyhow::Result;
use std::sync::Arc;
use tokio::{
    sync::{Mutex, RwLock},
    time::{sleep, Duration},
};

mod document;
mod tree;

use document::{Content, Node, Watcher};
use tree::Tree;

#[tokio::main]
async fn main() -> Result<()> {
    let content = Arc::new(RwLock::new(Content::new("example".into())?));

    let (watcher, rx) = Watcher::new(content.clone());
    let watcher_task = tokio::spawn(async move {
        watcher.watch().await.unwrap();
    });

    let (mut root, root_rx) = Node::root(rx, content.clone());
    let root = Arc::new(Mutex::new(root));

    let (mut tree, root_id) = Tree::new(root.clone());

    /*let mut root_observer = NodeObserver::new(root.clone(), rx);
    tokio::spawn(async move {
        root_observer.observe().await.unwrap();
    });*/

    tokio::spawn(async move {
        root.lock().await.observe_root().await.unwrap();
    });

    let (mut child_1, child_1_rx) = Node::new(root_rx, content.clone(), "pattern");
    let child_1 = Arc::new(Mutex::new(child_1));
    let child_1_id = tree.add_node(root_id, child_1.clone());
    /*let mut child_1_observer = NodeObserver::new(root.clone(), child_1_rx);
    tokio::spawn(async move {
        child_1_observer.observe().await.unwrap();
    });*/
    tokio::spawn(async move {
        child_1.lock().await.observe_node().await.unwrap();
    });

    //let _subdocument_1 = Subdocument::full_document(content.clone());
    //let _subdocument_2 = Subdocument::new(content.clone(), "word");

    let c = content.clone();
    let sleep_task = tokio::spawn(async move {
        loop {
            let read_lock = c.read().await;
            println!("read {} lines", read_lock.len());
            drop(read_lock);

            sleep(Duration::from_millis(1_000)).await;
        }
    });

    //watcher_task.await?;
    //root_task.await?;
    sleep_task.await?;

    Ok(())
}
