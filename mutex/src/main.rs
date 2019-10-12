use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, RwLock};

mod counter;
mod idtree;
mod stdmap;

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    foo();
    // bar();
}

fn foo() {
    let mut handles = vec![];
    let t = stdmap::T::default();

    {
        let t = t.clone();
        let h = std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            t.info();
        });
        handles.push(h);
    }

    for i in 0..10 {
        let t = t.clone();
        let h = std::thread::spawn(move || loop {
            t.foo();
        });
        handles.push(h);
    }
    for i in 0..10 {
        let t = t.clone();
        let h = std::thread::spawn(move || loop {
            t.bar();
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn bar() {
    let mut handles = vec![];
    let t = idtree::T::default();

    {
        let t = t.clone();
        let h = std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            t.info();
        });
        handles.push(h);
    }

    for i in 0..10 {
        let t = t.clone();
        let h = std::thread::spawn(move || loop {
            t.foo();
        });
        handles.push(h);
    }
    for i in 0..10 {
        let t = t.clone();
        let h = std::thread::spawn(move || loop {
            t.bar();
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}
