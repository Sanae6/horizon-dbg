#![no_std]

use postcard_rpc::{endpoints, topics, TopicDirection};

use crate::{
  breakpoints::BreakpointHitMsg, subscription::{SubscriptionUpdateMsg, UpdateSubscriptionMsg, UpdateSubscriptionResult}
};

extern crate alloc;

pub mod breakpoints;
pub mod result;
pub mod subscription;

endpoints! {
  list = ENDPOINTS_LIST;

  | EndpointTy         | RequestTy             | ResponseTy               | Path                  |
  | ----------         | ---------             | ----------               | ----                  |
  | UpdateSubscription | UpdateSubscriptionMsg | UpdateSubscriptionResult | "update_subscription" |
}

topics! {
  list = SWITCH_TOPICS;
  direction = TopicDirection::ToClient;

  | TopicTy             | MessageTy             | Path                   |
  | -------             | ---------             | ----                   |
  | BreakpointHit       | BreakpointHitMsg      | "breakpoint_hit"       |
  | SubscriptionUpdated | SubscriptionUpdateMsg | "subscription_updated" |
}

topics! {
  list = APP_TOPICS;
  direction = TopicDirection::ToServer;
  
  | TopicTy             | MessageTy             | Path                   |
  | -------             | ---------             | ----                   |
}
