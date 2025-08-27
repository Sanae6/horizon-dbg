use alloc::vec::Vec;
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

use crate::{keygen, result::HorizonResult};

#[derive(Serialize, Deserialize, Schema)]
#[repr(C)]
pub struct UpdateSubscription {
  pub id: u64,
  pub start_address: u64,
  pub size: u32,
}

keygen!(const UPDATE_SUBSCRIPTION: UpdateSubscription = "update_subscription");

#[derive(Serialize, Deserialize, Schema)]
pub struct DeleteSubscription {
  pub id: u64,
}
keygen!(const DELETE_SUBSCRIPTION: DeleteSubscription = "delete_subscription");

pub type UpdateSubscriptionResult = Result<(), HorizonResult>;

#[derive(Serialize, Deserialize, Schema)]
pub struct Changes {
  pub address: u64,
  pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Schema)]
pub struct SubscriptionUpdated {
  pub address: u64,
  pub bytes: Vec<Changes>,
}

keygen!(const SUBSCRIPTION_UPDATED: SubscriptionUpdated = "subscription_updated");
