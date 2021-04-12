use crate::matcher::Matcher;
use std::sync::Arc;
use tokio::sync::{watch::Receiver, RwLock};

pub trait Node {
    fn set_id(&mut self, id: usize);

    fn get_receiver(&self) -> Receiver<usize>;
    fn set_parent_rx(&mut self, rx: Receiver<usize>);

    fn get_indices(&self) -> Arc<RwLock<Vec<usize>>>;
    fn set_parent_indices(&mut self, indices: Arc<RwLock<Vec<usize>>>);

    fn observe<M: 'static + Matcher + Send>(&mut self, matcher: M);
    fn cancel(&self);
}
