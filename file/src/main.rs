use std::io::Seek;
use std::io::Write;
 use std::io::Read;
fn main() {
    // f0()
    // bench_seek()
    // write_everywhere()
    // list_dir("/Users/fangsihao/Documents/code");
    // iter();
    // truncate();
    read_file();
}

fn f0() {
    let dir = "./main.rs";
    let entries = std::fs::read_dir(dir).unwrap();
    for entry in entries {
        let entry: std::fs::DirEntry = entry.unwrap();
        let path = entry.path();
        println!("entry: {:?}", entry);
        println!("path: {:?}", path);
        let extension = path.extension();
        println!("extension: {:?}", extension);
        let filename = path.file_name();
        println!("filename: {:?}", filename);

        println!("\n\n");
    }
}

fn bench_seek() {
    use rand::prelude::*;
    use std::io::Seek;

    let args = std::env::args().collect::<Vec<String>>();
    let count = if args.len() > 1 {
        args[1].parse::<i32>().unwrap()
    } else {
        100000
    };

    let mut file = std::fs::File::open("./test.txt").unwrap();
    let length = file.metadata().unwrap().len();
    println!("file length {}, seek count {}", length, count);
    let start = std::time::SystemTime::now();
    let mut rng = rand::thread_rng();
    for _i in 0..count {
        let offset: u64 = rng.gen();
        let offset = offset % (length as u64);
        file.seek(std::io::SeekFrom::Start(offset)).unwrap();
    }
    let elapsed = start.elapsed().unwrap();
    println!(
        "seek: {} times, elapsed: {}, speed: {:.2}us/seek",
        count,
        elapsed.as_micros(),
        elapsed.as_micros() as f64 / count as f64
    );
}

fn write_everywhere() {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .create_new(false)
        .truncate(true)
        .open("./write_everywhere.txt")
        .unwrap();
    for i in 0..9 {
        file.write(format!("{:09}\n", i).as_bytes()).unwrap();
    }
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.write(format!("{:09}\n", 10).as_bytes()).unwrap();
}

fn list_dir(dir: &str) {
    let file_list = std::fs::read_dir(dir).unwrap();
    for entry in file_list {
        let entry: std::fs::DirEntry = entry.unwrap();
        println!("file name: {:?}", entry.file_name());
    }
}

fn iter() {
    let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr: Vec<i32> = arr
        .into_iter()
        .map(|x| {
            let a = x * x;
            println!("square: {}", a);
            a
        })
        .map(|x| {
            let a = x - 2;
            println!("minus: {}", a);
            a
        })
        .collect();
}

fn truncate() {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .read(true)
        .open("./test-seek.txt")
        .unwrap();
    file.seek(std::io::SeekFrom::Start(64 * 1024)).unwrap();
    file.write_all("hello world!".as_bytes()).unwrap();
    file.set_len(64 * 1024).unwrap();
    file.sync_all();
    file.seek(std::io::SeekFrom::Start(64 * 1024)).unwrap();
    file.write_all("hello world!".as_bytes()).unwrap();
}

fn read_file() {
    let path = std::path::PathBuf::from("./Cargo.toml");
    let mut buffer = Vec::with_capacity(10);
       let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .read(true)
        .open(path)
        .unwrap();
    file.read_exact(&mut buffer);
    println!("{:?}", buffer);
}