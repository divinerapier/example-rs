fn main() {
    let _a = T::new(1, "a");
    let _b = T::new(2, "b");
    drop(_a);
    drop(_b);
}

pub struct T {
    id: usize,
    name: String,
}

impl T {
    fn new<S: Into<String>>(id: usize, name: S) -> T {
        T {
            id,
            name: name.into(),
        }
    }
}

impl Drop for T {
    fn drop(&mut self) {
        println!("id: {}, name: {}", self.id, self.name);
    }
}
