#[macro_use]
extern crate redis_module;

use redis_module::{Context, NextArg, RedisResult, RedisString};

// === Data Types ===

#[derive(Debug, Clone)]
struct SummaryStatistics {
    count: i64,
    min: f64,
    max: f64,
    sum: f64,
}

impl Default for SummaryStatistics {
    fn default() -> SummaryStatistics {
        SummaryStatistics {
            count: 0_i64,
            min: f64::MAX,
        }
    }
}

// === Module Commands ===

fn ns_info(_: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_iter().skip(1);
    let name = args.into_iter().next_string()?;
    let greet = format!("Hello {}!", name);
    return Ok(greet.into());
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
