use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

use crate::keygen;

#[derive(Serialize, Deserialize, Schema)]
#[repr(C)]
pub enum BreakpointKind {
  Soft,
  Hard,
  Watch,
}
#[derive(Serialize, Deserialize, Schema)]
#[repr(C)]
pub struct SetBreakpoint {
  pub logical_id: u32,
  pub address: u64,
  pub kind: BreakpointKind,
}

keygen!(const SET_BREAKPOINT: SetBreakpoint = "set_breakpoint");

#[derive(Serialize, Deserialize, Schema)]
#[repr(C)]
pub struct BreakpointHitMsg {
  pub logical_id: u32,
}

keygen!(const BREAKPOINT_HIT: BreakpointHitMsg = "breakpoint_hit");
