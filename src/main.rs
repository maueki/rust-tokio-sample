extern crate tokio;
#[macro_use]
extern crate futures;
extern crate tokio_timer;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::{self, Interval};
use futures::future;

use std::time::{Duration, Instant};

fn process(mut socket: TcpStream) {
    println!("create new process");
    let f = Interval::new(Instant::now(), Duration::from_millis(2000))
        .for_each(move |instant| {
            println!("fire; instant={:?}", instant);
            loop {
                match socket.write(b"hoge\r\n") {
                    Ok(_) => break,
                    Err(err) => {
                        match err.kind() {
                            // Why WouldBlock is returned at first time?
                            std::io::ErrorKind::WouldBlock => continue,
                            _ => {
                                println!("");
                                return Err(timer::Error::shutdown())
                            }
                        }
                    },
                }
            }
            Ok(())
        })
        .map_err(|e| {
            println!("process error: {:?}", e);
        });

    tokio::spawn(f);
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
