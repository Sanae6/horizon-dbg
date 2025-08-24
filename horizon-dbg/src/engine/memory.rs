use nodit::{Interval, NoditMap};

use crate::engine::types::RegionType;

pub struct Memory {
  interval_tree: NoditMap<Interval<u32>, u32, Region>,
}

struct Region {
  pub region_type: RegionType,
  pub start: u64,
  pub end: u64,
}

pub struct Subscription {
  pub address: u64,
  pub size: u32,
}
