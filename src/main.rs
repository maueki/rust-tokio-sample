extern crate tokio;
#[macro_use]
extern crate futures;
extern crate tokio_timer;

use tokio::io::{self, Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::{self, Interval};
use futures::future;

use std::time::{Duration, Instant};

fn send_data(mut socket: &TcpStream, msg: &str) -> Result<(), Error> {
    loop {
        let msg = (msg.to_string() + &"\r\n".to_string());
        match socket.write(msg.as_bytes()) {
            Ok(_) => break,
            Err(err) => {
                match err.kind() {
                    // Why WouldBlock is returned at first time?
                    std::io::ErrorKind::WouldBlock => continue,
                    _ => {
                        return Err(err)
                    }
                }
            },
        }
    }

    Ok(())
}

fn process(mut socket: TcpStream) {
    println!("create new process");
    let mut msgs = vec!["hoge1", "hoge2"];

    let f = Interval::new(Instant::now(), Duration::from_millis(2000))
        .for_each(move |instant| {
            if let Some(msg) = msgs.pop() {
                println!("fire; instant={:?}", instant);
                match send_data(&socket, msg) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(timer::Error::shutdown()),
                }
            } else {
                Err(timer::Error::shutdown())
            }
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
