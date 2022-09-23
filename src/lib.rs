#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisResult, RedisString};

// === Module Commands ===

fn ns_info(_: &Context, _: Vec<RedisString>) -> RedisResult {
    return Ok("Hello".into());
}
