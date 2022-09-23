#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisResult, RedisString};

// === Module Commands ===

fn ns_info(_: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_iter().skip(1);
    let name = args.into_iter().next_string()?;
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
