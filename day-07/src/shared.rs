pub struct Node {
    pub value: usize,
    pub parent_idx: Option<usize>,
    pub children_idxs: Vec<usize>,
}

pub struct Tree {
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn append_node(&mut self, val: usize, parent_idx: Option<usize>) {
        self.nodes.push(Node {
            value: val,
            parent_idx,
            children_idxs: vec![],
        });

        if parent_idx.is_none() {
            return;
        }

        // if we have a parent index (i.e., not root node):
        // add the idx to the node's children_idxs
        // recursively sum values for all ancestor dirs
        let idx = self.nodes.len() - 1;
        self.nodes[parent_idx.unwrap()].children_idxs.push(idx);
        self.recurse_update_values(val, parent_idx);
    }

    pub fn get_last_index(&mut self) -> Option<usize> {
        if self.nodes.is_empty() {
            return Some(0);
        }

        Some(self.nodes.len() - 1)
    }

    pub fn recurse_update_values(&mut self, value: usize, idx: Option<usize>) -> usize {
        let node = &mut self.nodes[idx.unwrap()];
        node.value += value;
        let next_idx = node.parent_idx;

        if next_idx.is_none() {
            return node.value;
        } else {
            println!("next_idx: {v}", v = next_idx.unwrap());
        }

        self.recurse_update_values(value, next_idx)
    }
}
