use nodit::{Interval, NoditMap};

use crate::engine::types::RegionType;

pub struct MemoryMap {
  interval_tree: NoditMap<Interval<u32>, u32, u32>,
}

struct Region {
  region_type: RegionType,
  start: u64,
  end: u64,
}

pub struct MemoryWatchRegion {
  start_page: u64,
  data: WatchRegionData,
}

pub enum WatchRegionData {
  Waiting { pages: u32 },
  Populated { data: Vec<Option<[u8; 0x1000]>> },
  Error // todo
}
