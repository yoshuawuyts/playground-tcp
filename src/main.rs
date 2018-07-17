#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

// #[macro_use]
// extern crate futures_await;

use playground_tcp::cli::Cli;
use structopt::StructOpt;
use tokio::{io, net, prelude::*};

fn main() -> Result<(), Box<std::error::Error>> {
  let args = Cli::from_args();

  let listener = args.port.bind()?;
  let handle = tokio::reactor::Handle::current();
  let listener = net::TcpListener::from_std(listener, &handle)?;

  let server = listener
    .incoming()
    .map_err(|err| eprintln!("err {:?}", err))
    .for_each(move |socket| {
      let (reader, writer) = socket.split();
      let fut = io::copy(reader, writer).then(move |res| {
        match res {
          Ok((amt, _, _)) => println!("wrote {} bytes", amt),
          Err(e) => eprintln!("error: {}", e),
        }

        Ok(())
      });
      tokio::spawn(fut)
    });

  tokio::run(server);
  Ok(())
}
