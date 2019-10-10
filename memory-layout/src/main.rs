fn main() {
    let a = Some(vec![1_u32, 1_u32, 1_u32]);
    let b: Option<Vec<u32>> = None;
    println!("sizeof(Some(Vec<u32>)) = {}", std::mem::size_of_val(&a));
    println!("sizeof(None) = {}", std::mem::size_of_val(&b));

    println!("Hello, world!");
}
