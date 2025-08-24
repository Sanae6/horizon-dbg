use alloc::vec::Vec;
use postcard_rpc::topic;
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Schema)]
pub struct UpdateSubscription {
  id: u32,
  start_address: u64,
  size: u32,
}

#[derive(Serialize, Deserialize, Schema)]
struct Changes {
  address: u64,
  bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Schema)]
pub struct SubscriptionUpdateMsg {
  address: u64,
  bytes: Vec<Changes>,
}

topic!(
  SubscriptionUpdate,
  SubscriptionUpdateMsg,
  "subscription_update"
);
