#![allow(dead_code)]

use futures::stream::Stream;
use futures::Future;
use rusoto_core::request::{HttpClient, TlsError};
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{CreateBucketRequest, GetObjectRequest, PutObjectRequest, S3Client, S3};
use srand;
use std::env;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

/// Create client using given static access/secret keys
pub fn new_s3client_with_credentials(
    region: Region,
    access_key: String,
    secret_key: String,
) -> Result<S3Client, TlsError> {
    Ok(S3Client::new_with(
        HttpClient::new()?,
        StaticProvider::new_minimal(access_key, secret_key),
        region,
    ))
}

pub fn create_test_bucket() -> (S3Client, String) {
    let endpoint = env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string());
    println!("S3_ENDPOINT: {}", endpoint);
    let client = new_s3client_with_credentials(
        Region::Custom {
            name: "eu-west-1".to_owned(),
            endpoint,
        },
        "admin".to_owned(),
        "password".to_owned(),
    )
    .unwrap();

    let bucket_name = String::from("5577006791947779410");
    // let bucket_name = srand::ThreadLocal::uint64().to_string();

    // println!("bucket: {}", bucket_name);
    // client
    //     .create_bucket(CreateBucketRequest {
    //         bucket: bucket_name.clone(),
    //         ..Default::default()
    //     })
    //     .sync()
    //     .unwrap();

    (client, bucket_name)
}

pub fn put_object(client: &S3Client, bucket: &str, key: &str, data: Vec<u8>) {
    client
        .put_object(PutObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            body: Some(data.into()),
            ..Default::default()
        })
        .sync()
        .unwrap();
}

// fn walkdir(root: &Path, tx: Sender<PathBuf>) {
//     for entry in walkdir::WalkDir::new(root) {
//         let entry: walkdir::DirEntry = entry.unwrap();
//         let path: &Path = entry.path();
//         if !path.is_dir() {
//             tx.send(path.to_path_buf());
//         } else {
//             walkdir(path, tx.clone());
//         }
//     }
// }

fn recursive_upload(client: &S3Client, bucket: &str, root: &str) {
    // let root: &Path = Path::new(root);
    // let parent: &Path = root.parent().unwrap();
    // let root_cloned = root.to_path_buf();
    // let (tx, rx): (Sender<PathBuf>, Receiver<PathBuf>) = channel();
    // let h = std::thread::spawn(move || {
    //     walkdir(&root_cloned, tx);
    // });
    // for path in rx {
    //     let data = std::fs::read(&path).unwrap();
    //     let path: PathBuf = path;
    //     // println!("upload {} to {}",path.);
    //     put_object(
    //         &client,
    //         bucket,
    //         path.strip_prefix(parent)
    //             .as_ref()
    //             .unwrap()
    //             .to_str()
    //             .unwrap(),
    //         data,
    //     );
    // }
    // h.join();
    let root: &Path = Path::new(root);
    let parent: &Path = root.parent().unwrap();
    for entry in walkdir::WalkDir::new(root) {
        let entry: walkdir::DirEntry = entry.unwrap();
        let path: &Path = entry.path();
        if path.is_dir() {
            continue;
        }
        let key = path.strip_prefix(parent).unwrap();
        println!("upload {:?} to {:?}", path, key);
        let data = std::fs::read(path).unwrap();
        put_object(client, bucket, key.to_str().unwrap(), data);
    }
}

pub fn get_body(client: &S3Client, bucket: &str, key: &str) -> Vec<u8> {
    let object = client
        .get_object(GetObjectRequest {
            bucket: bucket.to_owned(),
            key: key.to_owned(),
            ..Default::default()
        })
        .sync()
        .unwrap();
    object.body.unwrap().concat2().wait().unwrap().to_vec()
}

pub fn init_logger() {
    let _ = env_logger::Builder::from_default_env()
        .filter(Some("s4"), log::LevelFilter::Debug)
        .try_init();
}

pub struct ReaderWithError {
    pub abort_after: usize,
}

impl Read for ReaderWithError {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        if buf.len() > self.abort_after {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "explicit, unconditional error",
            ));
        }
        for i in buf.iter_mut() {
            *i = 0;
        }
        self.abort_after -= buf.len();
        Ok(buf.len())
    }
}

fn main() {
    let (client, bucket_name) = create_test_bucket();
    recursive_upload(&client, &bucket_name, env::var("UPLOAD_DIR"));
}
