mod tcp_transport;

use std::io;

use postcard_rpc::host_client::{HostClient, WireSpawn, WireTx};
use tokio::net::TcpStream;
use xtra::Address;

use crate::engine::connection::tcp_transport::{TcpReader, TcpWriter};

pub struct Connection {
  host_client: HostClient<()>,
}

impl Connection {
  pub fn new_tcp(stream: TcpStream) -> Self{
    let (reader, writer) = stream.into_split();
    let host_client = HostClient::new_with_wire(
      TcpWriter::new(writer),
      TcpReader::new(reader),
      Spawner,
      postcard_rpc::header::VarSeqKind::Seq2,
      "errored",
      1,
    );

    Self {
      host_client
    }
  }

  pub fn update_subscription_address(&self) {}
}

struct Spawner;

impl WireSpawn for Spawner {
  fn spawn(&mut self, fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
  }
}
