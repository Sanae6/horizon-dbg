use postcard_rpc::topic;
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Schema)]
pub struct UpdateWatchRegion {
  id: u32,
  start_page: u32,
}

#[derive(Serialize, Deserialize, Schema)]
pub struct WatchRegionPageChangedMsg {
  id: u32,
  _todo: ()
}

topic!(
  WatchRegionPageChanged,
  WatchRegionPageChangedMsg,
  "watch_region_page_changed"
);
