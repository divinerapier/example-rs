type Result = std::result::Result<i32, i32>;

fn main() {
    foo();
}

fn foo() {
    let a: Result = Ok(3);
    let mut i = 0;
    let _ = a
        .map(|v| -> i32 {
            println!("ok {}", v);
            // Ok(v + 1)
            9
        })
        .map_err(|e| -> i32 {
            println!("err {}", e);
            i = i + 1;
            i
        })
        .map(|v| -> i32 {
            println!("ok {} {}", v, v);
            10
        })
        .map_err(|e| -> i32 {
            println!("err {} {}", e, e);
            100
        });
}
