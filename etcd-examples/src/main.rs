use etcd::kv::{self, Action, GetOptions};
use etcd::Client;
use futures::Future;
use tokio::runtime::Runtime;

mod client;

fn main() {
    // foo();
    // bar();
    // bar2();
    bar3();
}

fn foo() {
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

fn bar() {
    let mut conn = client::Client::new(&["http://localhost:2379"], None).unwrap();
    let result = conn.create("/app", "value", None);
    if let Err(e) = result {
        if !e.exists_key() {
            println!("create /app error: {:?}", e);
            return;
        }
    }

    let result = conn.create("/app", "value", None);
    if let Err(e) = result {
        if !e.exists_key() {
            println!("create /app error: {:?}", e);
            return;
        }
    }

    println!("delete");

    let result = conn.delete("/app", false);
    if let Err(e) = result {
        if !e.exists_key() {
            println!("delete /app error: {:?}", e);
            return;
        }
    }

    let result = conn.create_dir("/app", None);
    if let Err(e) = result {
        if !e.exists_key() {
            println!("create dir /app error: {:?}", e);
            return;
        }
    }

    let result = conn.create_dir("/app", None);
    if let Err(e) = result {
        if !e.exists_key() {
            println!("create dir /app error: {:?}", e);
            return;
        }
    }
}

fn bar2() {
    let mut conn = client::Client::new(&["http://localhost:2379"], None).unwrap();

    println!("delete /app. {:?}", conn.delete("/app", false));

    println!("force delete /app. {:?}", conn.delete("/app", true));

    let result = conn.create("/app", "value", None);
    if let Err(e) = result {
        if !e.exists_key() {
            println!("create /app error: {:?}", e);
            return;
        }
    }

    let result = conn.create("/app/1", "1", None);
    if let Err(e) = result {
        if !e.exists_key() {
            println!("create /app error: {:?}", e);
            return;
        }
    }

    println!(
        "set /app = 'hello world'. {:?}",
        conn.set("/app", "hello world", None)
    );

    println!("list /app");
    let result = conn.list_dir("/app");
    println!("list dir. {:?}", result);

    let result = conn.list_dir("/app/1");
    println!("list dir. {:?}", result);
}

fn bar3() {
    let mut conn = client::Client::new(&["http://localhost:2379"], None).unwrap();
    println!("is_dir(\"/app\") = {}", conn.is_dir("/app"));
    println!("is_dir(\"/app/1\") = {}", conn.is_dir("/app/1"));
    println!("is_dir(\"/app/2\") = {}", conn.is_dir("/app/2"));
    println!("is_dir(\"/app/3\") = {}", conn.is_dir("/app/3"));
    println!("is_dir(\"/app/4\") = {}", conn.is_dir("/app/4"));
}
