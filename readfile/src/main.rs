use std::io::Read;

fn main() {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("./Cargo.toml");
    let mut file = match file {
        Ok(f) => f,
        Err(e) => {
            println!("open file error. {:?}", e);
            return;
        }
    };

    let mut buffer = Vec::with_capacity(4096);
    buffer.resize(4096, 0);
    let size = file.read(&mut buffer).unwrap();
    println!("read size: {}", size);
    println!("content: {}", String::from_utf8_lossy(&buffer));
}
