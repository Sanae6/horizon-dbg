use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Schema)]
pub enum BreakpointKind {
  Soft,
  Hard,
  Watch,
}
#[derive(Serialize, Deserialize, Schema)]
pub struct SetBreakpointReq {
  logical_id: u32,
  address: u64,
  kind: BreakpointKind,
}

#[derive(Serialize, Deserialize, Schema)]
pub struct BreakpointHitMsg {
  logical_id: u32,
}
