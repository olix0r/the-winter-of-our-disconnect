#![deny(rust_2018_idioms, warnings)]

use futures::{try_ready, Async, Future, Poll, Stream};
use log::{error, info};
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "the-winter-of-our-disconnect")]
struct Opt {
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,
}

fn main() {
    pretty_env_logger::init();

    let Opt { port } = Opt::from_args();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&addr).expect("must bind");
    info!("listening on {}", addr);

    tokio::run(
        listener
            .incoming()
            .for_each(|socket| {
                match socket.peer_addr() {
                    Ok(peer) => {
                        info!("accepted from: {}", peer);
                        let _ = socket.set_nodelay(true);
                        tokio::spawn(Serve(Some(socket), peer));
                    }
                    Err(e) => {
                        error!("ignoring connection from unknown peer: {}", e);
                    }
                }

                Ok(())
            })
            .map_err(|e| panic!("listener failed: {}", e)),
    );
}

struct Serve(Option<tokio::net::TcpStream>, SocketAddr);

impl Future for Serve {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use tokio::io::AsyncRead;

        let mut buf = [0u8; 1];
        let sz = try_ready!(self
            .0
            .as_mut()
            .unwrap()
            .poll_read(&mut buf)
            .map_err(|e| error!("failed to read from socket: {}", e)));

        let sock = self.0.take().unwrap();
        drop(sock);
        info!("dropped {} after {}B", self.1, sz);

        Ok(Async::Ready(()))
    }
}
