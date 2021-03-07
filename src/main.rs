use anyhow::Result;
use std::sync::Arc;
use tokio::{
    sync::RwLock,
    time::{sleep, Duration},
};

mod document;

use document::{Content, Node, Watcher};

#[tokio::main]
async fn main() -> Result<()> {
    let content = Arc::new(RwLock::new(Content::new("example".into())?));

    let watcher = Watcher::new(content.clone());
    let mut node = Node::new(watcher.new_receiver(), content.clone(), "pattern");
    let watcher_task = tokio::spawn(async move {
        watcher.watch().await.unwrap();
    });

    let root_task = tokio::spawn(async move {
        node.watch().await;
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

    watcher_task.await?;
    root_task.await?;
    sleep_task.await?;

    Ok(())
}
