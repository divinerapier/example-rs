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

    let bucket_name = srand::ThreadLocal::uint64().to_string();

    println!("bucket: {}", bucket_name);
    client
        .create_bucket(CreateBucketRequest {
            bucket: bucket_name.clone(),
            ..Default::default()
        })
        .sync()
        .unwrap();

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
    let (client, _) = create_test_bucket();
    println!("Hello, world!");
}
