fn main() {
    println!("Hello, world!");
    let mut node = Node {};
    foo(&node);
    bar(&mut node);
}

struct Node {}

fn foo(node: &[u8]) {}

fn bar(node: &mut [u8]) {}

impl std::ops::Deref for Node {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        println!("deref",);
        unsafe {
            std::slice::from_raw_parts(
                self as *const Node as *const u8,
                std::mem::size_of::<Node>(),
            ) as &[u8]
        }
    }
}

impl std::ops::DerefMut for Node {
    fn deref_mut(&mut self) -> &mut [u8] {
        println!("deref mut",);
        unsafe {
            std::slice::from_raw_parts_mut(
                self as *mut Node as *mut u8,
                std::mem::size_of::<Node>(),
            ) as &mut [u8]
        }
    }
}
