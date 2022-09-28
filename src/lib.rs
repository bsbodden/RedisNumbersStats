#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::{Context, NextArg, RedisResult, RedisString};
use redis_module::raw::RedisModuleTypeMethods;

// === Data Type Declaration ===

pub const REDIS_TYPE_NAME: &str = "Redis-NSS";
pub const REDIS_TYPE_VERSION: i32 = 1;

pub static REDIS_TYPE: RedisType = RedisType::new(
    REDIS_TYPE_NAME,
    REDIS_TYPE_VERSION,
    RedisModuleTypeMethods {
        version: redis_module::TYPE_METHOD_VERSION,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: None,
        mem_usage: None,
        digest: None,
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,
        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
    },
);

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
            max: f64::MIN,
            sum: 0.0_f64,
        }
    }
}

// === Module Commands ===

fn ns_create(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_iter().skip(1);
    let key_arg = args.into_iter().next_string()?;
}

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
  data_types: [
    REDIS_TYPE
  ],
  commands: [
    ["ns.info", ns_info, "readonly", 0, 0, 0],
  ]
}
