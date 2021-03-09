use slab::Slab;
use std::collections::HashMap;

pub struct Tree<T> {
    data: Slab<T>,
    structure: HashMap<usize, Vec<usize>>,
}

impl<T> Tree<T> {
    pub fn new(root_value: T) -> (Self, usize) {
        let mut data = Slab::with_capacity(64);
        let root_id = data.insert(root_value);

        let mut structure = HashMap::new();
        structure.insert(root_id, Vec::new());

        (Tree { data, structure }, root_id)
    }

    pub fn add_node(&mut self, parent_id: usize, value: T) -> Option<usize> {
        if let Some(parent) = self.structure.get_mut(&parent_id) {
            let new_id = self.data.insert(value);
            parent.push(new_id);
            self.structure.insert(new_id, Vec::new());
            Some(new_id)
        } else {
            None
        }
    }

    pub fn remove_node(&mut self, node_id: usize) {
        let mut nodes_to_remove = Vec::new();
        self.collect_ids(node_id, &mut nodes_to_remove);

        for node in nodes_to_remove {
            self.structure.remove(&node);
            self.data.remove(node);
        }
    }

    fn collect_ids(&self, id: usize, all: &mut Vec<usize>) {
        all.push(id);
        if let Some(children) = self.structure.get(&id) {
            for c in children {
                self.collect_ids(*c, all);
            }
        }
    }

    pub fn get_node(&self, id: usize) {}
}
