use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    let (sender, receiver) = channel();
    let handler = std::thread::spawn(move || {
        consumer(receiver);
    });
    let handler1 = std::thread::spawn(move || {
        producer(sender);
    });
    handler.join();
    handler1.join();
}

fn consumer(receiver: Receiver<Vec<u8>>) {
    for data in receiver.iter() {
        println!("{:?}", data);
    }
    println!("consumer closed",);
}

fn producer(sender: Sender<Vec<u8>>) {
    for i in 0..10 {
        let data = vec![0 + i * 3, 1 + i * 3, 2 + i * 3];
        sender.send(data);
    }
    println!("producer closed",);
}
