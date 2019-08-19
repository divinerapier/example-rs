fn main() {
    let client = redis::Client::open("redis://172.16.10.28:6378").unwrap();
    let mut conn = client.get_connection().unwrap();
    let _: () = redis::cmd("AUTH").arg("test").query(&mut conn).unwrap();
    scan(&mut conn);
}

fn scan(conn: &mut redis::Connection) {
    let iter: std::collections::HashMap<usize, String> = redis::cmd("SCAN")
        .cursor_arg(0)
        .arg("COUNT")
        .arg(10)
        .clone()
        .query(conn)
        .unwrap();

    for (i, e) in iter.iter().enumerate() {
        println!("index: {}, value: {:?}", i, e)
    }
}
