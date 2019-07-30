use futures::{Async, Future, Poll, Stream};
use tokio;

use std::io::{Read, Write};
use std::path::Path;

struct FileStream {
    batch_size: usize,
    pub file: std::fs::File,
    pub buffer: Vec<u8>,
    pub length: usize,
}

impl FileStream {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<FileStream> {
        let batch_size: usize = 409600;
        let file = std::fs::File::open(path)?;
        let mut buffer = Vec::with_capacity(batch_size);
        buffer.resize(batch_size, 0);
        Ok(FileStream {
            batch_size,
            file,
            buffer,
            length: 0,
        })
    }
}

impl Stream for FileStream {
    type Item = Vec<u8>;
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.buffer.clear();
        self.buffer.resize(self.batch_size, 0);
        let size = self.file.read(&mut self.buffer)?;
        println!("read size: {}", size);
        if size == 0 {
            return Ok(Async::Ready(None));
        }

        Ok(Async::Ready(Some(self.buffer[0..size].to_vec())))
    }
}

struct FileWriter<T> {
    file: std::fs::File,
    stream: T,
    length: usize,
}

impl<T> FileWriter<T>
where
    T: Stream,
{
    pub fn new(file: std::fs::File, stream: T) -> FileWriter<T> {
        FileWriter {
            file: file,
            stream: stream,
            length: 0,
        }
    }
}

impl<T> Future for FileWriter<T>
where
    T: Stream<Item = Vec<u8>>,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        while let Some(value) = futures::try_ready!(self.stream.poll()) {
            let value: Vec<u8> = value;
            self.length += value.len();
            println!("read data: {}, already: {}", value.len(), self.length);
            self.file.write(&value[0..value.len()]).unwrap();
        }
        Ok(Async::Ready(()))
    }
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let input_path = current_dir.join("input.txt");
    let output_path = current_dir.join("output.txt");
    println!("input: {:?}", input_path);
    println!("output: {:?}", output_path);
    let file_stream = FileStream::new(input_path).unwrap();
    let output = std::fs::File::create(output_path).unwrap();
    let writer = FileWriter::new(output, file_stream);
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(writer).unwrap();
}
