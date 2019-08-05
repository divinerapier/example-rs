fn main() {
    // f0()
    bench_seek()
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
     ;
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
