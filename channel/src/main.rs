use bytes::buf::IntoBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    // bounded_channel();
    check_channel_is_closed();
}

fn close_channel() {
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

fn consumer(receiver: Receiver<bytes::Bytes>) {
    for data in receiver.iter() {
        let data = data.as_ref();
        println!("{:?}", data);
    }
    println!("consumer closed",);
}

fn producer(sender: Sender<bytes::Bytes>) {
    for i in 0..10 {
        let data = vec![0 + i * 3, 1 + i * 3, 2 + i * 3];
        let data = bytes::Bytes::from(data);
        sender.send(data);
    }
    println!("producer closed",);
}

fn bounded_channel() {
    let (sender, receiver) = std::sync::mpsc::sync_channel(10);
    let mut handlers = vec![];
    for i in 0..10 {
        let sender = sender.clone();
        let t = std::thread::spawn(move || {
            for j in 0.. {
                let data = format!("thread: {}. {}", i, j);
                sender.send(data.clone());
                println!("{}", data);
            }
        });
        handlers.push(t);
    }
    for h in handlers {
        h.join();
    }
}

fn check_channel_is_closed() {
    let (sender, receiver) = std::sync::mpsc::channel();
    let handler = std::thread::spawn(move || {
        sender.send(1);
        sender.send(2);
        sender.send(3);
    });
    handler.join();
    std::sync::mpsc::RecvError;
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
}
