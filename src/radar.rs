
use tokio::timer::Interval;
use tokio_uds::{UnixStream};
use futures::Future;
use futures::prelude::*;
use futures::{Poll, Async};

use std;
use std::time::{Duration, Instant};
use std::io::ErrorKind;
use std::io::Write;

use data::CanData;

pub struct Radar {
    st: UnixStream,
    can_data: CanData,
    interval: Interval,
}

impl Radar {
    pub fn new(st: UnixStream) -> Self {
        let can_data = CanData::new("rev_testdata5a.csv".to_string()).unwrap();
        let interval = Interval::new(Instant::now(), Duration::from_millis(1000));

        Radar{st: st, can_data: can_data, interval: interval}
    }
}

impl Future for Radar {
    type Item = ();
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<(), std::io::Error> {
        loop {
            match self.interval.poll() {
                Ok(Async::Ready(Some(_))) => {
                    match send_data(&self.st, &"hoge") {
                        Ok(_) => {
                            continue
                        },
                        _ => {
                            println!("send_data failed??");
                            return Ok(Async::Ready(()))
                        }
                    }
                },
                Ok(Async::Ready(None)) => {
                    println!("interval terminated");
                    return Ok(Async::NotReady)
                },
                Ok(Async::NotReady) => {
                    return Ok(Async::NotReady)
                },
                Err(e) => {
                    println!("interval.poll() failed: {:?}", e);
                    return Ok(Async::Ready(()))
                }
            }
        }
    }

}

fn send_data(mut socket: &UnixStream, msg: &str) -> Result<(), std::io::Error> {
    let msg = msg.to_string() + &"\r\n".to_string();
    loop {
        match socket.write(msg.as_bytes()) {
            Ok(_) => {
                break
            },
            Err(err) => {
                match err.kind() {
                    // Why WouldBlock is returned at first time?
                    ErrorKind::WouldBlock => {
                        continue
                    },
                    _ => {
                        println!("send_data failed: {:?}", err);
                        return Err(err)
                    }
                }
            },
        }
    }

    Ok(())
}
