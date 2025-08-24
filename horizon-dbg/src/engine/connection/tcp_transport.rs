use std::{io, sync::Arc};

use postcard_rpc::host_client::{WireRx, WireTx};
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt, ReadBuf},
  net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub struct TcpWriter {
  writer: OwnedWriteHalf,
}

impl TcpWriter {
  pub fn new(half: OwnedWriteHalf) -> Self {
    Self { writer: half }
  }
}

impl WireTx for TcpWriter {
  type Error = io::Error;

  async fn send(&mut self, data: Vec<u8>) -> Result<(), Self::Error> {
    self.writer.write_all(&data).await
  }
}

pub struct TcpReader {
  reader: OwnedReadHalf,
}

impl TcpReader {
  pub fn new(half: OwnedReadHalf) -> Self {
    Self { reader: half }
  }
}

impl WireRx for TcpReader {
  type Error = io::Error;

  async fn receive(&mut self) -> Result<Vec<u8>, Self::Error> {
    let mut vec = vec![0; 512];

    let mut buf = ReadBuf::new(&mut vec);
    self.reader.read_buf(&mut buf).await?;
    let len = buf.filled().len();
    vec.truncate(len);
    Ok(vec)
  }
}
