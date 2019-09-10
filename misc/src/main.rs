fn main() {
    print!("{:2} ", " ");
    for i in 1..16 {
        print!("{:2} ", i);
    }
    println!("");
    for i in 1..16 {
        print!("{:2} ", i);
        for j in 1..16 {
            print!("{:2} ", i ^ j);
        }
        println!("");
    }
}
