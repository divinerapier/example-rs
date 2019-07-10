use etcd::kv::{self, GetOptions};
use futures::future::Future;
use tokio::runtime::Runtime;

struct Error {}

impl From<std::vec::Vec<etcd::Error>> for Error {
    fn from(ee: Vec<etcd::Error>) -> Self {
        Self {}
    }
}

pub struct Client {
    c: etcd::Client<hyper::client::HttpConnector>,
    run_destructor: bool,
    runtime: Runtime,
}

pub struct KV {
    key: String,
    value: String,
}

impl Client {
    pub fn list_dir(&mut self, key: &str, ttl: Option<u64>) -> Result<Vec<KV>, Error> {
        let client = self.c.clone();
        let fu = kv::get(
            &client,
            key,
            GetOptions {
                recursive: false,
                sort: false,
                strong_consistency: true,
            },
        )
        .and_then(|response| {
            let response: etcd::Response<kv::KeyValueInfo> = response;
            let mut kv_pairs = vec![];
            let nodes = response.data.node.nodes;
            for node in nodes.as_ref().unwrap() {
                let node: &etcd::kv::Node = node;
                if let Some(is_dir) = node.dir {
                    if is_dir {
                        continue;
                    }
                }
                if let Some(value) = &node.value {
                    kv_pairs.push(KV {
                        key: node.key.as_ref().unwrap().clone(),
                        value: value.clone(),
                    });
                }
            }
            Ok(kv_pairs)
        });
        let res = self.runtime.block_on(fu)?;
        Ok(res)
    }
}
