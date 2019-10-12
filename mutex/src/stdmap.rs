use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct T {
    pub lasttime: Arc<AtomicU64>,
    pub map: Arc<RwLock<HashMap<u64, u64>>>,
    pub counter: crate::counter::Counter,
}

impl Default for T {
    fn default() -> Self {
        T {
            lasttime: Arc::new(AtomicU64::new(0)),
            map: Arc::new(RwLock::new(HashMap::new())),
            counter: crate::counter::Counter::new(1),
        }
    }
}

impl T {
    pub fn foo(&self) {
        self.counter.start("foo".to_owned());
        let map = self.map.read().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let now_sec = now.as_secs() as f64;
        let now = (now_sec / 10e9) as u64;
        let v = now % 10000000;
        match map.get(&v) {
            Some(d) => log::trace!("v: {}, d: {}", v, d),
            None => log::trace!("v: {}, none", v),
        }
    }

    pub fn bar(&self) {
        let _start = self.counter.start("bar".to_owned());
        let mut map = self.map.write().unwrap();
        let v: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
            % 10000000 as u64;
        let entry = map.entry(v).or_insert(0);
        *entry += 1;
    }

    pub fn info(&self) {
        let _start = self.counter.start("info".to_owned());
        let map = self.map.read().unwrap();
        log::info!("entry count: {}", map.len());
    }
}
