#[macro_use]
extern crate futures;
extern crate tokio;

use futures::{Async, Future, Poll, Stream};
use std::time::Duration;
use tokio::timer::Interval;

pub struct Fibonacci {
    interval: Interval,
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new(duration: Duration) -> Fibonacci {
        Fibonacci {
            interval: Interval::new_interval(duration),
            curr: 1,
            next: 1,
        }
    }
}

impl Stream for Fibonacci {
    type Item = u64;

    // The stream will never yield an error
    type Error = ();

    fn poll(&mut self) -> Poll<Option<u64>, ()> {
        // Wait until the next interval
        try_ready!(self
            .interval
            .poll()
            // The interval can fail if the Tokio runtime is unavailable.
            // In this example, the error is ignored.
            .map_err(|_| ()));

        let curr = self.curr;
        let next = curr + self.next;

        self.curr = self.next;
        self.next = next;

        Ok(Async::Ready(Some(curr)))
    }
}

pub struct Display10<T> {
    stream: T,
    curr: usize,
}

impl<T> Display10<T> {
    fn new(stream: T) -> Display10<T> {
        Display10 { stream, curr: 0 }
    }
}

impl<T> Future for Display10<T>
where
    T: Stream,
    T::Item: std::fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        while self.curr < 10 {
            let value = match try_ready!(self.stream.poll()) {
                Some(value) => value,
                // There were less than 10 values to display, terminate the
                // future.
                None => break,
            };

            println!("value #{} = {}", self.curr, value);
            self.curr += 1;
        }

        Ok(Async::Ready(()))
    }
}

fn main() {
    let fib = Fibonacci::new(Duration::from_secs(1));
    let display = Display10::new(fib);

    tokio::run(display);
}
