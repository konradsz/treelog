use slab::Slab;
use std::collections::HashMap;

use crate::document::{Identifiable, OnNotify};
use crate::matcher::{PassthroughMatcher, PatternMatcher};

pub struct Tree<T>
where
    T: OnNotify + Identifiable,
{
    data: Slab<T>,
    structure: HashMap<usize, Vec<usize>>,
}

impl<T> Tree<T>
where
    T: OnNotify + Identifiable,
{
    pub fn new(root_value: T) -> (Self, usize) {
        let mut data = Slab::with_capacity(64);
        let root_id = data.insert(root_value);

        let mut structure = HashMap::new();
        structure.insert(root_id, Vec::new());

        // Safety: it is safe to use get_unchecked since this element was added line above
        unsafe {
            data.get_unchecked_mut(root_id).set_id(root_id);
            data.get_unchecked_mut(root_id)
                .observe_node(PassthroughMatcher());
        }

        (Tree { data, structure }, root_id)
    }

    pub fn add_node(&mut self, parent_id: usize, value: T, pattern: &str) -> Option<usize> {
        if let Some(parent) = self.structure.get_mut(&parent_id) {
            let node_id = self.data.insert(value);

            // Safety: it is safe to use get_unchecked since this element was added line above
            unsafe {
                self.data.get_unchecked_mut(node_id).set_id(node_id);

                let parent_rx = self.data.get_unchecked_mut(parent_id).get_receiver();
                self.data
                    .get_unchecked_mut(node_id)
                    .set_parent_rx(parent_rx);

                self.data
                    .get_unchecked_mut(node_id)
                    .observe_node(PatternMatcher::new(pattern).unwrap());
            }

            parent.push(node_id);
            self.structure.insert(node_id, Vec::new());
            Some(node_id)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn remove_node(&mut self, node_id: usize) {
        let mut nodes_to_remove = Vec::new();
        self.collect_ids(node_id, &mut nodes_to_remove);

        for node_id in nodes_to_remove {
            let node = self.data.get(node_id).unwrap();
            node.cancel();

            self.structure.remove(&node_id);
            self.data.remove(node_id);
        }
    }

    fn collect_ids(&self, id: usize, all: &mut Vec<usize>) {
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
