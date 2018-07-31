extern crate tokio;
#[macro_use]
extern crate futures;
extern crate tokio_timer;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::Interval;
use futures::future;

use std::time::{Duration, Instant};

fn process(mut socket: TcpStream) {
    let f = Interval::new(Instant::now(), Duration::from_millis(5000))
        .for_each(|instant| {
            println!("fire; instant={:?}", instant);
//            socket.poll_write(b"hoge\r\n").into_future();
            Ok(())
        })
        .map_err(|e| {
            println!("process error: {:?}", e);
        });

    tokio::run(f);
}

fn main() {
    let addr = "127.0.0.1:60000".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(move |socket| {
        process(socket);
        Ok(())
    })
        .map_err(|e| {
            println!("accept error = {:?}", e);
        });

    tokio::run(server);
}
