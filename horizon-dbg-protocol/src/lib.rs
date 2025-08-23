use postcard_rpc::endpoint;
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Schema)]
pub struct Req {}

endpoint! {
  Among,
  Req,
  Req,
  ""
}
