use nodit::{Interval, NoditMap};

use crate::engine::types::RegionType;

pub struct Memory {
  interval_tree: NoditMap<Interval<u32>, u32, u32>, 
}

struct Region {
  region_type: RegionType,
  start: u64,
  end: u64,
  
}
