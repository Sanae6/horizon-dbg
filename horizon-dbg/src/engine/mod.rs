use egui::ahash::HashMap;
use thunderdome::{Arena, Index};
use xtra::Address;

use crate::engine::{connection::Connection, memory::Subscription};

mod types;
pub mod memory;
pub mod connection;

struct DebuggerInterface {
  connection: Connection,
  subscriptions: Arena<Subscription>,
}

impl DebuggerInterface {
  pub fn new() -> DebuggerInterface {
    DebuggerInterface { connection: todo!(), subscriptions: Default::default() }
  }

  pub fn add_subscription(&mut self, address: u64, size: u32) -> Index {
    // todo: start at cursor
    self.subscriptions.insert(Subscription { address, size })
  }

  pub fn update_subscription_address(&mut self, index: Index, address: u64) {
    self.subscriptions.get_mut(index).expect("invalid index").address = address;

    
  }
}
