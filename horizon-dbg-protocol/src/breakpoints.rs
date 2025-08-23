use postcard_rpc::{endpoint, topic};
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

use crate::result::HorizonResult;

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
pub enum SetBreakpointRes {
  Success,
  KernelError(HorizonResult),
}

endpoint!(
  SetBreakpoint,
  SetBreakpointReq,
  SetBreakpointRes,
  "set_breakpoint"
);

#[derive(Serialize, Deserialize, Schema)]
pub struct BreakpointHitMsg {
  logical_id: u32,
}

topic!(BreakpointHit, BreakpointHitMsg, "breakpoint_hit");
