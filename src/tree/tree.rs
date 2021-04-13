use slab::Slab;
use std::collections::HashMap;
use std::convert::From;

use super::{Node, NodeId};
use crate::matcher::{PassthroughMatcher, PatternMatcher};

pub struct Tree<T>
where
    T: Node,
{
    data: Slab<T>,
    structure: HashMap<NodeId, Vec<NodeId>>,
}

impl<T> Tree<T>
where
    T: Node,
{
    pub fn new(root_value: T) -> (Self, NodeId) {
        let mut data = Slab::with_capacity(64);
        let root_id = NodeId::from(data.insert(root_value));

        let mut structure = HashMap::new();
        structure.insert(root_id, Vec::new());

        // Safety: it is safe to use get_unchecked since this element was added line above
        unsafe {
            data.get_unchecked_mut(root_id.into())
                .set_id(root_id.into());
            data.get_unchecked_mut(root_id.into())
                .observe(PassthroughMatcher());
        }

        (Tree { data, structure }, root_id)
    }

    pub fn add_node(&mut self, parent_id: NodeId, value: T, pattern: &str) -> Option<NodeId> {
        if let Some(parent) = self.structure.get_mut(&parent_id) {
            let node_id = NodeId::from(self.data.insert(value));

            // Safety: it is safe to use get_unchecked since this element was added line above
            unsafe {
                self.data.get_unchecked_mut(node_id.into()).set_id(node_id);

                let parent_rx = self.data.get_unchecked_mut(parent_id.into()).get_receiver();
                self.data
                    .get_unchecked_mut(node_id.into())
                    .set_parent_rx(parent_rx);

                let parent_indices = self.data.get_unchecked_mut(parent_id.into()).get_indices();
                self.data
                    .get_unchecked_mut(node_id.into())
                    .set_parent_indices(parent_indices);

                self.data
                    .get_unchecked_mut(node_id.into())
                    .observe(PatternMatcher::new(pattern).unwrap());
            }

            parent.push(node_id);
            self.structure.insert(node_id, Vec::new());
            Some(node_id)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn remove_node(&mut self, node_id: NodeId) {
        let mut nodes_to_remove = Vec::new();
        self.collect_ids(node_id, &mut nodes_to_remove);

        for node_id in nodes_to_remove {
            let node = self.data.get(node_id.into()).unwrap();
            node.cancel();

            self.structure.remove(&node_id);
            self.data.remove(node_id.into());
        }
    }

    fn collect_ids(&self, id: NodeId, all: &mut Vec<NodeId>) {
        all.push(id);
        if let Some(children) = self.structure.get(&id) {
            for child in children {
                self.collect_ids(*child, all);
            }
        }
    }

    #[allow(dead_code)]
    fn get_node(&self, id: usize) -> &T {
        self.data.get(id).unwrap()
    }
}
