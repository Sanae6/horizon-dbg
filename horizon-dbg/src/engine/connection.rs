use std::{io, net::SocketAddr, ops::DerefMut, sync::Arc};

use horizon_dbg_protocol::{
  Keygen,
  frame::Header,
  postcard::{self, to_stdvec},
  postcard_schema::Schema,
  serde::Serialize,
  subscription::{
    DELETE_SUBSCRIPTION, DeleteSubscription, UPDATE_SUBSCRIPTION, UpdateSubscription,
  },
};
use postcard_rpc::Key2;
use tokio::{
  io::{AsyncWriteExt, BufWriter},
  net::{tcp::OwnedWriteHalf, TcpStream},
  sync::{oneshot, Mutex},
};

pub struct Connection {
  transport: Arc<Mutex<Transport>>,
}

pub enum Transport {
  Tcp(BufWriter<OwnedWriteHalf>),
}

impl Connection {
  pub fn connect_tcp(socket_addr: SocketAddr) -> oneshot::Receiver<io::Result<Connection>> {
    let(sender, receiver) = oneshot::channel();
    tokio::spawn(async move {
      let _ = sender.send(TcpStream::connect(socket_addr).await.map(Connection::new_tcp));
    });
    receiver
  }

  fn new_tcp(stream: TcpStream) -> Self {
    let (reader, writer) = stream.into_split();
    
    tokio::spawn(async move {

    });

    Self {
      transport: Arc::new(Mutex::new(Transport::Tcp(BufWriter::new(writer)))),
    }
  }

  fn write_message<T: Schema + Serialize + Send + 'static>(
    &mut self,
    key: &'static Keygen,
    value: T,
  ) {
    let length =
      postcard::experimental::serialized_size(&value).expect("failed to calculate size of packet");
    let length = length
      .try_into()
      .expect("packet was too large, try splitting it into ");
    let header = Header {
      length,
      key: u16::from_le_bytes(Key2::from_key8(key.key).to_bytes()),
    };

    let transport = self.transport.clone();
    tokio::spawn(async move {
      let mut transport = transport.lock().await;
      let transport = match &mut *transport {
        Transport::Tcp(buf_writer) => buf_writer,
      };

      use horizon_dbg_protocol::zerocopy::IntoBytes;
      transport.write_all(header.as_bytes()).await.unwrap();
      let awesome = to_stdvec(&value).expect("failed to serialize packet");
      transport.write_all(&awesome).await.unwrap();
    });
  }

  pub fn update_subscription_address(&mut self, index: u64, address: u64, size: u32) {
    self.write_message(
      &UPDATE_SUBSCRIPTION,
      UpdateSubscription {
        id: index,
        start_address: address,
        size: size,
      },
    );
  }

  pub fn delete_subscription(&mut self, index: u64) {
    self.write_message(&DELETE_SUBSCRIPTION, DeleteSubscription { id: index });
  }
}
