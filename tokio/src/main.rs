use futures::future::Future;
use futures::stream::Stream;

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let path = "/";
    let paths = tokio_fs::read_dir(path)
        .flatten_stream()
        .map(|entry| {
            let entry: tokio_fs::DirEntry = entry;
            let entry: std::fs::DirEntry = entry.into_std();
            entry.path().to_str().unwrap().to_owned()
        })
        .collect();

    let paths = rt.block_on(paths).unwrap();
    println!("path: {:?}", paths);
}
