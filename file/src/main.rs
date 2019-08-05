fn main() {
    let dir = "/path/to/something";
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
