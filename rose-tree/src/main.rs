use std::ops::Index;
use std::ops::IndexMut;

use petgraph::graph::DefaultIx;
use petgraph::graph::NodeIndex;

use rose_tree::RoseTree;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    id: u64,
    name: String,
}

#[derive(Debug)]
struct Manager<Ix = DefaultIx> {
    tree: RoseTree<Node>,
    mapper: HashMap<u64, NodeIndex<Ix>>,
}

impl Manager {
    fn new() -> Manager {
        let (mut tree, root) = rose_tree::RoseTree::<Node, u32>::new(Node::new(0, ""));
        let mut mapper = std::collections::HashMap::new();
        let root = tree.add_child(root, Node::new(1, "/"));
        mapper.insert(1u64, root);
        Manager { tree, mapper }
    }
    fn readdir(&mut self, ino: u64) {
        if ino == 1 {
            let parent_index = *self.mapper.get(&ino).unwrap();
            let nodes = vec![Node::new(2, "etc"), Node::new(3, "home")];
            for node in &nodes {
                match self.mapper.get(&node.id) {
                    Some(n) => {
                        let n: &mut Node = self.tree.index_mut(*n);
                        n.id = node.id;
                        n.name = node.name.clone();
                    }
                    None => {
                        let child_index = self.tree.add_child(parent_index, node.clone());
                        self.mapper.insert(node.id, child_index);
                    }
                }
            }
        } else if ino == 2 {
        } else if ino == 3 {
            let parent_index = *self.mapper.get(&ino).unwrap();
            let nodes = vec![Node::new(4, "Alice"), Node::new(5, "Bob")];
            for node in &nodes {
                match self.mapper.get(&node.id) {
                    Some(n) => {
                        let n: &mut Node = self.tree.index_mut(*n);
                        n.id = node.id;
                        n.name = node.name.clone();
                    }
                    None => {
                        let child_index = self.tree.add_child(parent_index, node.clone());
                        self.mapper.insert(node.id, child_index);
                    }
                }
            }
        } else if ino == 4 {
            let parent_index = *self.mapper.get(&ino).unwrap();
            let nodes = vec![Node::new(6, "Documents"), Node::new(7, "Steam")];
            for node in &nodes {
                match self.mapper.get(&node.id) {
                    Some(n) => {
                        let n: &mut Node = self.tree.index_mut(*n);
                        n.id = node.id;
                        n.name = node.name.clone();
                    }
                    None => {
                        let child_index = self.tree.add_child(parent_index, node.clone());
                        self.mapper.insert(node.id, child_index);
                    }
                }
            }
        } else {
        }
    }
}

impl Node {
    fn new<S: Into<String>>(id: u64, name: S) -> Node {
        Node {
            id,
            name: name.into(),
        }
    }
}

fn main() {
    let mut mgr: Manager<DefaultIx> = Manager::new();
    for i in 1..6 {
        mgr.readdir(i);
        println!("mgr: {:#?}", mgr);
    }
}
