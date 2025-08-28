use thunderdome::{Arena, Index};

use crate::engine::{connection::Connection, memory::Subscription};

pub mod connection;
pub mod memory;
mod types;

pub struct DebuggerInterface {
  connection: Connection,
  subscriptions: Arena<Subscription>,
}

impl DebuggerInterface {
  pub fn new(connection: Connection) -> DebuggerInterface {
    DebuggerInterface {
      connection,
      subscriptions: Default::default(),
    }
  }

  pub fn add_subscription(&mut self, address: u64, size: u32) -> Index {
    // todo: start at cursor
    let index = self.subscriptions.insert(Subscription { address, size });
    self
      .connection
      .update_subscription_address(index.to_bits(), address, size);
    index
  }

  pub fn update_subscription_address(&mut self, index: Index, address: u64) {
    let sub = self.subscriptions.get_mut(index).expect("invalid index");
    sub.address = address;
    self
      .connection
      .update_subscription_address(index.to_bits(), address, sub.size);
  }

  pub fn delete_subscription(&mut self, index: Index) {
    self
      .subscriptions
      .remove(index)
      .expect("tried to remove twice");

    self.connection.delete_subscription(index.to_bits());
  }
}
