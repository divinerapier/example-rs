use futures::future::Future;
use futures::stream::Stream;
use futures_util::future::FutureExt;
use futures_util::stream::StreamExt;
use futures_util::try_future::TryFutureExt;
use hyper::client::{connect::HttpConnector, Client};
use hyper::{Body, Request, Response};
use std::io::Write;

// fn fetch_url(url: hyper::Uri) -> impl Future<Output = Result<(), ()>> {
//     let client = hyper::client::Client::new();

//     client
//         // Fetch the url...
//         .get(url)
//         // And then, if we get a response back...
//         .and_then(|res| {
//             println!("Response: {}", res.status());
//             println!("Headers: {:#?}", res.headers());

//             // The body is a stream, and for_each returns a new Future
//             // when the stream is finished, and calls the closure on
//             // each chunk of the body...
//             res.into_body().for_each(|chunk| {
//                 std::io::stdout()
//                     .write_all(&chunk)
//                     .map_err(|e| panic!("example expects stdout is open, error={}", e))
//             })
//         })
//         // If all good, just tell the user...
//         .map(|_| {
//             println!("\n\nDone.");
//             futures::future::ok(())
//         })
//         // If there was an error, let the user know...
//         .map_err(|err| {
//             eprintln!("Error {}", err);
//             futures::future::err(())
//         })
// }

// fn fetch_url_body(url: hyper::Uri) -> impl Future<Output = Result<(), ()>> {
//     let client = hyper::client::Client::new();

//     client
//         // Fetch the url...
//         .get(url)
//         // And then, if we get a response back...
//         .and_then(|res| {
//             println!("Response: {}", res.status());
//             println!("Headers: {:#?}", res.headers());

//             // The body is a stream, and for_each returns a new Future
//             // when the stream is finished, and calls the closure on
//             // each chunk of the body...
//             // res.into_body().for_each(|chunk| {
//             //     std::io::stdout()
//             //         .write_all(&chunk)
//             //         .map_err(|e| panic!("example expects stdout is open, error={}", e))
//             // })
//             // res.into_body().concat2()
//             let body: hyper::Body = res.into_body();
//             let mut data = vec![];
//             while let Some(next) = body.next().await {
//                 let chunk: &[u8] = &next?;
//                 data.extend_from_slice(chunk);
//             }
//             // res.into_body().for_each(|chunk| {
//             //     // std::io::stdout()
//             //     //     .write_all(&chunk)
//             //     //     .map_err(|e| panic!("example expects stdout is open, error={}", e))
//             //     println!("chunk {:?}\n", chunk);
//             // })
//             // for chunk in res.into_body() {
//             //     chunk.and_then(|chunk| {
//             //         println!("chunk {:?}\n", chunk);
//             //     });
//             // }
//             ()
//         });
//         futures::future::ok(())

//         // // If all good, just tell the user...
//         // .map(|body| {
//         //     // let body: hyper::body::Chunk = body;
//         //     // println!("\n\nbody: {:?}", body);
//         //     futures::future::ok(())
//         // })
//         // // If there was an error, let the user know...
//         // .map_err(|err| {
//         //     eprintln!("Error {}", err);
//         //     futures::future::err(())
//         // })
// }

// fn fetch_url_body(
//     url: hyper::Uri,
// ) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + 'static {
//     let client = Client::new();
//     let request = Request::get(url).body(Body::empty()).unwrap();
//     // async move {
//     //     let response = client.request(request).await;
//     //     let response = response.unwrap();
//     //     if response.status().is_success() {
//     //         return Err(format!("get"));
//     //     }
//     //     let mut body: Body = response.into_body();
//     //     let mut data = vec![];
//     //     while let Some(next) = body.next().await {
//     //         let chunk: &[u8] = &next.unwrap();
//     //         data.extend_from_slice(chunk);
//     //     }
//     //     Ok(data)
//     // }

//     let task = client.request(request).and_then(|response| {
//         if response.status().is_success() {
//             return Err(format!("get"));
//         }
//         let mut body: Body = response.into_body();
//         let mut data = vec![];

//         let chunks = body.collect();
//         println!("chunks: {:#?}", chunks);
//     })

//     Ok(data)
// }

fn fetch_url_body(url: hyper::Uri) -> impl std::future::Future<Output = Result<(), ()>> + 'static {
    let client = Client::new();
    let request = match Request::get(url).body(Body::empty()) {
        Ok(request) => request,
        Err(e) => {
            println!("create request with error: {}", e);
            return futures::future::ok(());
        }
    };
    // async move {
    //     let response = client.request(request).await;
    //     let response = response.unwrap();
    //     if response.status().is_success() {
    //         return Err(format!("get"));
    //     }
    //     let mut body: Body = response.into_body();
    //     let mut data = vec![];
    //     while let Some(next) = body.next().await {
    //         let chunk: &[u8] = &next.unwrap();
    //         data.extend_from_slice(chunk);
    //     }
    //     Ok(data)
    // }

    let task = client
        .request(request)
        .map(|response| {})
        .map(|err| {})
        .and_then(|response| {
            if response.status().is_success() {
                println!("get status: {}", response.status());
                // return futures::future::err(format!("get"));
                // return futures::future::ok(());
                return Ok(());
            }
            let mut body: Body = response.into_body();
            let mut data = vec![];

            let chunks = body.collect();
            println!("chunks: {:#?}", chunks);
            Ok(())
        });

    task
}

fn main() {
    let client = hyper::client::Client::new();
    let client = std::sync::Arc::new(client);
    let c = client.clone();
    // hyper::rt::run(hyper::rt::lazy(move || {
    //     // let uri = "https://docs.rs/hyper/0.13.0-alpha.4/hyper/struct.Response.html#examples-1"
    //     //     .parse()
    //     //     .unwrap();
    //     let uri = "http://www.baidu.com".parse().unwrap();
    //     c.get(uri)
    //         .and_then(|res| {
    //             println!("response status: {}", res.status());
    //             res.into_body().for_each(|chunk| {
    //                 std::io::stdout()
    //                     .write_all(&chunk)
    //                     .map_err(|e| panic!("example expects stdout is open, error={}", e))
    //             })
    //         })
    //         .map_err(|err| {
    //             println!("err: {:?}", err);
    //         })
    // }));
    // println!("hello world!");

    let uri = "http://www.google.com/search?q=a+%20+b&oq=a+%2B+b&aqs=chrome..69i57j69i64l2.1583j0j4&sourceid=chrome&ie=UTF-8";
    println!("original: {:?}", uri);
    // let uri = "https://www.google.com/search?q=a+%20+b&oq=a+%2B+b&aqs=chrome..69i57j69i64l2.1583j0j4&sourceid=chrome&ie=UTF-8"
    //     .parse()
    //     .unwrap();
    let u: url::Url = url::Url::parse(uri).unwrap();
    println!("rust-url: {:?}", u);
    let u: hyper::Uri = u.as_str().parse().unwrap();
    // let u: hyper::Uri = uri.parse().unwrap();
    println!("hyper-uri: {:?}", u);
    // println!("uri: {:?}", uri);
    // hyper::rt::run(fetch_url_body(u));
    // println!("hello world!");
    let task = fetch_url_body(u);
    // let mut runtime = tokio::runtime::Runtime::new().unwrap();
    // let body = runtime.block_on(task).unwrap();
    let body = futures::executor::block_on(task).unwrap();
    println!("body: {:?}", body);
}
