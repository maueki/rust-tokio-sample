extern crate tokio;
#[macro_use]
extern crate futures;
extern crate tokio_timer;
extern crate tokio_uds;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod packet;
mod data;
mod radar;

use tokio::io::Error;
use tokio_uds::{UnixListener, UnixStream};
use tokio::prelude::*;
use tokio::timer::{self, Interval};

use std::time::{Duration, Instant};

use radar::Radar;

fn process(socket: UnixStream) {
    println!("create new process");

    let radar = Radar::new(socket);

    tokio::spawn(radar.map(|_| {
        println!("in map");
        }).map_err(|_| {
        println!("in panic");
        panic!("")}
    ));
}

fn main() {
    let addr = "/tmp/can_dummy";
    let listener = UnixListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(move |socket| {
        process(socket);
        Ok(())
    })
        .map_err(|e| {
            println!("accept error = {:?}", e);
        });

    tokio::run(server);
}
