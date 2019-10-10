use id_tree::InsertBehavior::*;
use id_tree::*;

fn main() {
    foo();
    bar();
}

fn foo() {
    //      0
    //     / \
    //    1   2
    //   / \
    //  3   4
    let mut tree: Tree<i32> = TreeBuilder::new().with_node_capacity(5).build();

    let root_id: NodeId = tree.insert(Node::new(0), AsRoot).unwrap();
    let child_id: NodeId = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(3), UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(4), UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(7), UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(6), UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(5), UnderNode(&child_id)).unwrap();

    println!("Pre-order:");
    for node in tree.traverse_pre_order(&root_id).unwrap() {
        print!("{}, ", node.data());
    }
    // results in the output "0, 1, 3, 4, 7, 6, 5, 2, "

    let children = tree.children(&child_id).unwrap();
    println!("");
    for child in children {
        println!("{:?}, ", child);
    }
    // test peekable
    println!("test peekable");
    let children = tree.children(&child_id).unwrap();
    let mut children = children.peekable();
    println!("peekable: {:?}", children.peek());
    for child in children.skip(1) {
        println!("range peekable. {:?}", child);
    }
}

fn bar() {
    use rose_tree::*;
    use std::ops::Index;
    println!("");
    let (mut tree, root_index) = RoseTree::<i32, u32>::new(0);
    let child_id = tree.add_child(root_index, 1);
    tree.add_child(root_index, 2);
    tree.add_child(child_id, 3);
    tree.add_child(child_id, 4);
    tree.add_child(child_id, 7);
    tree.add_child(child_id, 6);
    tree.add_child(child_id, 5);
    for id in tree.children(child_id) {
        println!("{:?}", tree.index(id));
    }
}
