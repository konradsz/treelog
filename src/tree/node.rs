use std::{convert::From, sync::Arc};
use tokio::{
    sync::{watch::Receiver, RwLock},
    task::JoinHandle,
};

pub trait Node {
    fn get_receiver(&self) -> Receiver<usize>;

    fn get_indices(&self) -> Arc<RwLock<Vec<usize>>>;
    fn set_parent_indices(&mut self, indices: Arc<RwLock<Vec<usize>>>);

    fn observe(&mut self, new_parent_index_rx: Receiver<usize>) -> JoinHandle<()>;
    fn cancel(&self);
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<NodeId> for usize {
    fn from(id: NodeId) -> Self {
        id.0
    }
}
