use etcd::kv::{self, Action, GetOptions};
use etcd::Client;
use futures::Future;
use tokio::runtime::Runtime;

fn main() {
    // Create a client to access a single cluster member. Addresses of multiple cluster
    // members can be provided and the client will try each one in sequence until it
    // receives a successful response.
    let client = Client::new(&["http://localhost:2379"], None).unwrap();

    let client2 = client.clone();

    let res = kv::delete_dir(&client2, "/home").map_err(|e| println!("remove dir error: {:?}", e));

    println!("remove dir {} successful", "/home");

    let res = kv::create_dir(&client2, "/home", None).and_then(move |_| {
        // let response: etcd::Response<kv::KeyValueInfo> = response;
        // Ok(response.data)

        let f1 = kv::create(&client2, "/home/user1", "name1", None);
        let f2 = kv::create(&client2, "/home/user2", "name2", None);
        let f3 = kv::create(&client2, "/home/user3", "name3", None);

        f1.join3(f2, f3).wait()
    });
    assert!(Runtime::new().unwrap().block_on(res).is_ok());

    let client2 = client.clone();

    let res = kv::get(
        &client2,
        "/home",
        GetOptions {
            recursive: false,
            sort: false,
            strong_consistency: true,
        },
    )
    .and_then(move |response| {
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
            let key = &node.key;
            let value = node.value.as_ref().unwrap();
            kv_pairs.push((key.clone(), value.clone()));
        }

        futures::future::ok(kv_pairs)
    })
    .map_err(|e| println!("error: {:?}", e));

    let res = Runtime::new().unwrap().block_on(res);
    assert!(res.is_ok());
    println!("{:?}", res.unwrap());
}
