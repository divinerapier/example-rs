use futures::future::Future;
use futures::stream::Stream;
use std::io::Write;

fn fetch_url(url: hyper::Uri) -> impl Future<Item = (), Error = ()> {
    let client = hyper::client::Client::new();

    client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().for_each(|chunk| {
                std::io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        // If all good, just tell the user...
        .map(|_| {
            println!("\n\nDone.");
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

fn fetch_url_body(url: hyper::Uri) -> impl Future<Item = hyper::body::Chunk, Error = ()> {
    let client = hyper::client::Client::new();

    client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            // res.into_body().for_each(|chunk| {
            //     std::io::stdout()
            //         .write_all(&chunk)
            //         .map_err(|e| panic!("example expects stdout is open, error={}", e))
            // })
            res.into_body().concat2()
        })
        // If all good, just tell the user...
        .map(|body| {
            let body: hyper::body::Chunk = body;
            // println!("\n\nbody: {:?}", body);
            body
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
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
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let body = runtime.block_on(task).unwrap();
    println!("body: {:?}", body);
}
