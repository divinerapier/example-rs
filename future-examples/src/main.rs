use futures::future::Future;
use futures_util::future::FutureExt;
use futures_util::try_future::TryFutureExt;
use hyper::client::{connect::HttpConnector, Client};
use hyper::{Body, Request, Response};
use std::fmt::Debug;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

struct SimpleFuture {
    // inner: Pin<Box<dyn Future<Output = Result<usize, usize>> + Send>>,
    inner: Result<usize, usize>,
}

impl SimpleFuture {
    fn new(v: Result<usize, usize>) -> SimpleFuture {
        SimpleFuture { inner: v }
    }
}

impl Future for SimpleFuture {
    type Output = Result<usize, usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        // Pin::new(&mut self.inner).poll(cx)
        Poll::Ready(self.inner.clone())
    }
}

fn main() {
    let f = SimpleFuture::new(Ok(3));
    let f = f.and_then(|v| SimpleFuture::new(Err(v + 1)));

    // println!("{:?}", futures::executor::block_on(f));

    // and_then 要求函数返回一个 impl Future, 成功 返回 ::ok(), 错误返回 ::err()

    // let f = futures::future::ok::<usize, usize>(1);
    // let f = f
    //     .and_then(|v| futures::future::ok::<usize, usize>(v + 1))
    //     .and_then(|v| futures::future::err::<usize, usize>(v + 1));

    // future 执行成功
    // let f = futures::future::ok::<usize, usize>(1)
    //     .and_then(|v| {
    //         println!("and_then: {}", v);
    //         futures::future::err::<usize, usize>(v + 3)
    //     })
    //     .or_else(|e| {
    //         println!("or_else: {}", e);
    //         futures::future::err::<usize, String>((e + 1).to_string())
    //     })
    //     .map_err(|e| {
    //         println!("map_err: {}", e);
    //         format!("error: {}", e)
    //     });
    // let f = futures::future::ok::<usize, usize>(1)
    //     .and_then(|v| {
    //         println!("and_then: {}", v);
    //         futures::future::err::<usize, usize>(v + 3)
    //     })
    //     .or_else(|e| {
    //         println!("or_else: {}", e);
    //         futures::future::err::<usize, String>((e + 1).to_string())
    //     })
    //     .map_err(|e| {
    //         println!("map_err: {}", e);
    //         format!("error: {}", e)
    //     });

    let r = tokio::runtime::Runtime::new().unwrap();
    // println!("{:?}", futures::executor::block_on(f));
    println!("{:?}", r.block_on(f));
    {
        let u = "http://172.21.20.250:8888/server";
        let mut u: url::Url = url::Url::parse(&u).expect(&format!("parse url: {:?}", u));
        // if let Some(query_pairs) = query_pairs {
        //     u.query_pairs_mut().extend_pairs(query_pairs.into_iter());
        // }
        let u = u.as_str().replace("+", "%20");
        let u: hyper::Uri = u.as_str().parse().unwrap();
        let request = Request::head(u.clone())
            .body(Body::empty())
            .expect(&format!("head {:?}", u));

        // println!("{:?}", futures::executor::block_on(get_attibute(request)));

        r.block_on(get_attibute(request));
    }

    {
        let u = "http://172.21.20.250:8888/server";
        let mut u: url::Url = url::Url::parse(&u).expect(&format!("parse url: {:?}", u));
        // if let Some(query_pairs) = query_pairs {
        //     u.query_pairs_mut().extend_pairs(query_pairs.into_iter());
        // }
        let u = u.as_str().replace("+", "%20");
        let u: hyper::Uri = u.as_str().parse().unwrap();
        let request = Request::get(u.clone())
            .body(Body::empty())
            .expect(&format!("head {:?}", u));

        // println!("{:?}", futures::executor::block_on(get_attibute(request)));

         r.block_on(get(request));
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileType {
    /// Named pipe (S_IFIFO)
    NamedPipe,
    /// Character device (S_IFCHR)
    CharDevice,
    /// Block device (S_IFBLK)
    BlockDevice,
    /// Directory (S_IFDIR)
    Directory,
    /// Regular file (S_IFREG)
    RegularFile,
    /// Symbolic link (S_IFLNK)
    Symlink,
    /// Unix domain socket (S_IFSOCK)
    Socket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileAttr {
    /// Inode number
    pub ino: u64,
    /// Size in bytes
    pub size: u64,
    /// Size in blocks
    pub blocks: u64,
    /// Time of last access
    pub atime: SystemTime,
    /// Time of last modification
    pub mtime: SystemTime,
    /// Time of last change
    pub ctime: SystemTime,
    /// Time of creation (macOS only)
    pub crtime: SystemTime,
    /// Kind of file (directory, file, pipe, etc)
    pub kind: FileType,
    /// Permissions
    pub perm: u16,
    /// Number of hard links
    pub nlink: u32,
    /// User id
    pub uid: u32,
    /// Group id
    pub gid: u32,
    /// Rdev
    pub rdev: u32,
    /// Flags (macOS only, see chflags(2))
    pub flags: u32,
}

fn get_attibute(
    request: Request<Body>,
) -> impl std::future::Future<Output = Result<FileAttr, String>> + 'static {
    let client = Client::new();
    println!("{}:{}", std::file!(), std::line!());
    client
        .request(request)
        .map(|res| match res {
            Ok(res) => {
                println!("{}:{}", std::file!(), std::line!());
                let response: Response<Body> = res;
                if !response.status().is_success() {
                    return Err(format!("status code: {}", response.status()));
                }
                // let header = response.headers();
                let (header, body) = response.into_parts();
                let header = header.headers;
                println!("{}:{} header: {:?}", std::file!(), std::line!(), header);
                let size = if header.contains_key("Content-Length") {
                    let value: &hyper::header::HeaderValue = &header["Content-Length"];
                    value.to_str().unwrap_or("0").parse::<u64>().unwrap_or(0)
                } else {
                    0u64
                };
                println!("{}:{}", std::file!(), std::line!());
                let last_modified = if header.contains_key("Last-Modified") {
                    let value: &hyper::header::HeaderValue = &header["Last-Modified"];
                    value.to_str().unwrap_or("0").parse::<usize>().unwrap_or(0)
                } else {
                    0usize
                };
                println!("{}:{}", std::file!(), std::line!());
                let is_dir = if header.contains_key("X-Filer-Isdir") {
                    let value: &hyper::header::HeaderValue = &header["X-Filer-Isdir"];
                    value
                        .to_str()
                        .unwrap_or("true")
                        .parse::<bool>()
                        .unwrap_or(true)
                } else {
                    true
                };
                println!("{}:{}", std::file!(), std::line!());
                Ok(FileAttr {
                    ino: 0,
                    size,
                    blocks: 1,
                    atime: std::time::SystemTime::now(),
                    mtime: UNIX_EPOCH
                        .clone()
                        .add(Duration::from_secs(last_modified as u64)),
                    ctime: UNIX_EPOCH,
                    crtime: UNIX_EPOCH,
                    kind: if is_dir {
                        FileType::Directory
                    } else {
                        FileType::RegularFile
                    },
                    perm: if is_dir { 0o755 } else { 0o644 } as u16,
                    nlink: 1,
                    uid: 0,
                    gid: 0,
                    rdev: 0,
                    flags: 0,
                })
            }
            Err(err) => {
                log::error!("{}:{}", std::file!(), std::line!());
                Err(format!("{:?}", err))
            }
        })
        .map_err(|err| {
            log::error!("{}:{}", std::file!(), std::line!());
            format!("{:?}", err)
        })
}

fn get(
    request: Request<Body>,
) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + 'static {
    let client = Client::new();
    let mut request = request;
    request.headers_mut().insert("Accept", "application/json".parse().unwrap());
    println!("{}:{}", std::file!(), std::line!());
    async move {
        let response: Response<Body> = match client.request(request).await {
            Ok(response) => response,
            Err(err) => {
                println!("{}:{} error: {:?}", std::file!(), std::line!(), err);
                panic!(err)
            }
        };
        if !response.status().is_success() {
            return Err(format!("get"));
        }
        let mut body: Body = response.into_body();
        let mut data = vec![];
        println!("{}:{}",std::file!(), std::line!() );
        while let Some(next) = body.next().await {
            match next {
                Ok(next) => {
                    let chunk: &[u8] = &next;
                    // println!("chunk: {:?}", chunk);
                    data.extend_from_slice(chunk);
                }
                Err(err) => {
                    println!("{}:{} error: {:?}", std::file!(), std::line!(), err);
                    panic!(err);
                }
            }
        }
        Ok(data)
    }
    // client
    //     .request(request)
    //     .map(|res| match res {
    //         Ok(res) => {
    //             println!("{}:{}", std::file!(), std::line!());
    //             let response: Response<Body> = res;
    //             if response.status().is_success() {
    //                 return Err(format!("status code: {}", response.status()));
    //             }
    //             println!("{}:{}", std::file!(), std::line!());
    //             // let header = response.headers();
    //             let (_header, body) = response.into_parts();
    //             let body: Body = body;
    //             body.concat()

    //             // let header = header.headers;
    //             // let size = if header.contains_key("Content-Length") {
    //             //     let value: &hyper::header::HeaderValue = &header["Content-Length"];
    //             //     value.to_str().unwrap_or("0").parse::<u64>().unwrap_or(0)
    //             // } else {
    //             //     0u64
    //             // };
    //             // println!("{}:{}", std::file!(), std::line!());
    //             // let last_modified = if header.contains_key("Last-Modified") {
    //             //     let value: &hyper::header::HeaderValue = &header["Last-Modified"];
    //             //     value.to_str().unwrap_or("0").parse::<usize>().unwrap_or(0)
    //             // } else {
    //             //     0usize
    //             // };
    //             // println!("{}:{}", std::file!(), std::line!());
    //             // let is_dir = if header.contains_key("X-Filer-Isdir") {
    //             //     let value: &hyper::header::HeaderValue = &header["X-Filer-Isdir"];
    //             //     value
    //             //         .to_str()
    //             //         .unwrap_or("true")
    //             //         .parse::<bool>()
    //             //         .unwrap_or(true)
    //             // } else {
    //             //     true
    //             // };
    //             // println!("{}:{}", std::file!(), std::line!());
    //             // Ok(FileAttr {
    //             //     ino: 0,
    //             //     size,
    //             //     blocks: 1,
    //             //     atime: std::time::SystemTime::now(),
    //             //     mtime: UNIX_EPOCH
    //             //         .clone()
    //             //         .add(Duration::from_secs(last_modified as u64)),
    //             //     ctime: UNIX_EPOCH,
    //             //     crtime: UNIX_EPOCH,
    //             //     kind: if is_dir {
    //             //         FileType::Directory
    //             //     } else {
    //             //         FileType::RegularFile
    //             //     },
    //             //     perm: if is_dir { 0o755 } else { 0o644 } as u16,
    //             //     nlink: 1,
    //             //     uid: 0,
    //             //     gid: 0,
    //             //     rdev: 0,
    //             //     flags: 0,
    //             // })
    //         }
    //         Err(err) => {
    //             log::error!("{}:{}", std::file!(), std::line!());
    //             Err(format!("{:?}", err))
    //         }
    //     })
    //     .map_err(|err| {
    //         log::error!("{}:{}", std::file!(), std::line!());
    //         format!("{:?}", err)
    //     })
}
