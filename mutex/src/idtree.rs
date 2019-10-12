use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct T {
    pub inner: Arc<RwLock<InnerT>>,
    pub counter: crate::counter::Counter,
}

pub struct InnerT {
    pub map: id_tree::Tree<u64>,
    pub indexes: Vec<id_tree::NodeId>,
}

impl Default for T {
    fn default() -> Self {
        let t = T {
            inner: Arc::new(RwLock::new(InnerT {
                map: id_tree::Tree::new(),
                indexes: Vec::new(),
            })),
            counter: crate::counter::Counter::new(1),
        };
        let tc = t.clone();
        let mut inner = tc.inner.write().unwrap();
        let index = inner
            .map
            .insert(id_tree::Node::new(0), id_tree::InsertBehavior::AsRoot)
            .unwrap();
        inner.indexes.push(index);
        t
    }
}

impl T {
    pub fn foo(&self) {
        self.counter.start("foo".to_owned());
        let inner = self.inner.read().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let now_sec = now.as_secs() as f64;
        let now = (now_sec / 10e9) as u64;
        let v = now % 10000000;
        let index = &inner.indexes[v as usize % inner.indexes.len()];
        match inner.map.get(&index) {
            Ok(d) => log::trace!("v: {}, d: {:?}", v, d),
            Err(_) => log::trace!("v: {}, none", v),
        }
    }

    pub fn bar(&self) {
        let _start = self.counter.start("bar".to_owned());
        let mut inner = self.inner.write().unwrap();

        // let map: &mut id_tree::Tree<u64> = &mut inner.map;
        let v: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
            % 10000000 as u64;
        let root_index = inner.indexes[0].clone();
        if v < inner.indexes.len() as u64 {
            let index = inner.indexes[v as usize % inner.indexes.len()].clone();
            match inner.map.get_mut(&index) {
                Ok(ref mut entry) => {
                    let data: &mut u64 = entry.data_mut();
                    *data += 1;
                    return;
                }
                Err(e) => {}
            }
        }
        let i = inner
            .map
            .insert(
                id_tree::Node::new(v),
                id_tree::InsertBehavior::UnderNode(&root_index),
            )
            .unwrap();
        inner.indexes.push(i);
        // log::info!("---------------------len: {}", inner.indexes.len());
    }

    pub fn info(&self) {
        let _start = self.counter.start("info".to_owned());
        let inner = self.inner.read().unwrap();
        log::info!("entry count: {}", inner.indexes.len());
    }
}
