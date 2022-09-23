#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisResult, RedisString};

// === Module Commands ===

fn ns_info(_: &Context, args: Vec<RedisString>) -> RedisResult {
    return Ok("Hello".into());
}

// === Module Declaration ===

redis_module! {
  name: "rns",
  version: 1,
  data_types: [],
  commands: [
    ["ns.info", ns_info, "readonly", 0, 0, 0],
  ]
}
