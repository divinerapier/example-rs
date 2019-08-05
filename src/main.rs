struct A {
    list: Vec<B>,
    map: std::collections::HashMap<String, usize>,
}

struct B {
    key: String,
    value: i32,
}

impl A {
    pub fn new() -> A {
        A {
            list: vec![],
            map: std::collections::HashMap::new(),
        }
    }
    pub fn add(mut self, b: B) -> Self {
        let key = b.key.clone();
        self.list.push(b);
        self.map.insert(key, self.list.len() - 1);
        self
    }
}

fn main() {}
