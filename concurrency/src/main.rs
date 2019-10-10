fn main() {
    println!("Hello, world!");
    rwlock();
}

fn rwlock() {
    let rwlock = std::sync::RwLock::new(1);
    let a = rwlock.read().unwrap();
    println!("a = {}", a);
    let _ = rwlock.read().unwrap();
    println!("a = {}", a);
    let _ = rwlock.read().unwrap();
    println!("a = {}", a);
    let _ = rwlock.read().unwrap();
    println!("a = {}", a);
    let _ = rwlock.read().unwrap();
    println!("a = {}", a);
}
