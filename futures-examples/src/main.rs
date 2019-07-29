use futures::{Async, Future, Poll, Stream};

use std::io::{Read, Write};
use tokio;
struct FileStream {
    pub path: String,
    pub file: std::fs::File,
    pub buffer: Vec<u8>,
    pub length: usize,
}

impl FileStream {
    pub fn new(path: &str) -> std::io::Result<FileStream> {
        let file = std::fs::File::open(path)?;
        Ok(FileStream {
            path: path.to_owned(),
            file,
            buffer: Vec::with_capacity(4096),
            length: 0,
        })
    }
}

impl Stream for FileStream {
    type Item = Vec<u8>;
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.buffer.clear();
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
        let value = match futures::try_ready!(self.stream.poll()) {
            Some(value) => value,
            None => return Ok(Async::Ready(())),
        };
        let value: Vec<u8> = value;
        self.length += value.len();
        println!("read data: {}, already: {}", value.len(), self.length);
        self.file.write(&value[0..value.len()]);
        Ok(Async::Ready(()))
    }
}

fn main() {
    let mut file_stream = FileStream::new("/Users/fangsihao/Documents/code/rust/github.com/divinerapier/example-rs/futures-examples/src/test.txt").unwrap();
    let mut output = std::fs::File::create("/Users/fangsihao/Documents/code/rust/github.com/divinerapier/example-rs/futures-examples/src/output.txt").unwrap();
    // let fu = file_stream.and_then(move |data| {
    //     println!("write size: {}", data.length);
    //     output.write(&data.buffer[0..data.length])
    // });
    let writer = FileWriter::new(output, file_stream);
    // tokio::runtime::Runtime::block_on(writer);
    tokio::run(writer);
}
