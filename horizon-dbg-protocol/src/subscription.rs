use alloc::vec::Vec;
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

use crate::result::HorizonResult;

#[derive(Serialize, Deserialize, Schema)]
pub struct UpdateSubscriptionMsg {
  id: u32,
  start_address: u64,
  size: u32,
}

pub type UpdateSubscriptionResult = Result<(), HorizonResult>;

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
