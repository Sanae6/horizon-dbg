use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct HorizonResult(u32);

impl HorizonResult {
  pub fn succeeded(self) -> bool {
    self.0 == 0
  }

  pub fn failed(self) -> bool {
    self.0 != 0
  }

  pub fn module(self) -> u32 {
    todo!()
  }

  pub fn description(self) -> u32 {
    todo!()
  }
}
