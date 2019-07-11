use etcd::{
    kv::{self, GetOptions, KeyValueInfo},
    Response,
};
use futures::future::Future;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub enum Error {
    EtcdError(EtcdApiError),
    StdIOError(String),
    Unknown,
}

impl Error {
    pub fn etcd(&self) -> bool {
        use Error::*;
        match self {
            EtcdError(_) => true,
            _ => false,
        }
    }

    pub fn exists_key(&self) -> bool {
        use Error::*;
        match self {
            EtcdError(etcd_error) => match etcd_error {
                EtcdApiError::KeyExists => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn directory_not_empty(&self) -> bool {
        use Error::*;
        match self {
            EtcdError(etcd_error) => match etcd_error {
                EtcdApiError::DirectoryNotEmpty => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum EtcdApiError {
    KeyExists,
    DirectoryNotEmpty,
    KeyNotFound,
    NotDirectory,
    Unknown,
}

impl From<std::vec::Vec<etcd::Error>> for Error {
    fn from(ee: Vec<etcd::Error>) -> Self {
        let len = ee.len();
        for e in ee {
            return Error::from(e);
        }
        println!("unknown error. {}", len);
        Error::Unknown
    }
}

impl From<etcd::Error> for Error {
    fn from(e: etcd::Error) -> Error {
        match e {
            etcd::Error::Api(ref error) => {
                if error.message.eq("Key already exists") {
                    return Error::EtcdError(EtcdApiError::KeyExists);
                } else if error.message.eq("Directory not empty") {
                    return Error::EtcdError(EtcdApiError::DirectoryNotEmpty);
                } else if error.message.eq("Key not found") {
                    return Error::EtcdError(EtcdApiError::KeyNotFound);
                } else if error.message.eq("Not a directory") {
                    return Error::EtcdError(EtcdApiError::NotDirectory);
                } else {
                    println!("unknown etcd error: {}", error);
                    return Error::EtcdError(EtcdApiError::Unknown);
                }
            }
            _ => return Error::EtcdError(EtcdApiError::Unknown),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIOError(format!("{:?}", e))
    }
}

pub struct Client {
    c: etcd::Client<hyper::client::HttpConnector>,
    runtime: Runtime,
}

#[derive(Debug)]
pub struct KV {
    key: String,
    value: String,
    dir: bool,
}

impl Client {
    pub fn new(endpoints: &[&str], basic_auth: Option<etcd::BasicAuth>) -> Result<Client, Error> {
        Ok(Client {
            c: etcd::Client::new(endpoints, basic_auth)?,
            runtime: Runtime::new()?,
        })
    }
}

impl Client {
    pub fn list_dir(&mut self, key: &str) -> Result<Vec<KV>, Error> {
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
            if let Some(nodes) = response.data.node.nodes {
                for node in nodes {
                    let node: etcd::kv::Node = node;
                    let mut dir = false;
                    if let Some(is_dir) = node.dir {
                        dir = is_dir;
                    }
                    if let Some(value) = node.value {
                        kv_pairs.push(KV {
                            key: node.key.unwrap(),
                            value: value,
                            dir,
                        });
                    }
                }
            }
            Ok(kv_pairs)
        });
        let res = self.runtime.block_on(fu)?;
        Ok(res)
    }

    pub fn create_dir(
        &mut self,
        key: &str,
        ttl: Option<u64>,
    ) -> Result<Response<KeyValueInfo>, Error> {
        let client = self.c.clone();
        let fu = kv::create_dir(&client, key, ttl);
        Ok(self.runtime.block_on(fu)?)
    }

    pub fn create(
        &mut self,
        key: &str,
        value: &str,
        ttl: Option<u64>,
    ) -> Result<Response<KeyValueInfo>, Error> {
        let client = self.c.clone();
        let fu = kv::create(&client, key, value, ttl);
        Ok(self.runtime.block_on(fu)?)
    }

    pub fn set(
        &mut self,
        key: &str,
        value: &str,
        ttl: Option<u64>,
    ) -> Result<Response<KeyValueInfo>, Error> {
        let client = self.c.clone();
        let fu = kv::set(&client, key, value, ttl);
        Ok(self.runtime.block_on(fu)?)
    }

    pub fn delete(&mut self, key: &str, force: bool) -> Result<Response<KeyValueInfo>, Error> {
        let client = self.c.clone();
        let fu = kv::delete_dir(&client, key);
        if force {
            let dirs = self.list_dir(key)?;
            for dir in dirs {
                println!("dir: {:?}", dir);
                self.delete(&dir.key, true)?;
            }
        }
        Ok(self.runtime.block_on(fu)?)
    }
}
