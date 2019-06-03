#![deny(rust_2018_idioms, warnings)]

use futures::{Future, Stream};
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
                        drop(socket);
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
