#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::raw::RedisModuleTypeMethods;
use redis_module::{Context, NextArg, RedisResult, RedisString, RedisValue};
use serde::{Deserialize, Serialize};
use std::os::raw::c_void;

// === RDB Persistence ===

unsafe extern "C" fn free(value: *mut c_void) {
    Box::from_raw(value.cast::<SummaryStatistics>());
}

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
        free: Some(free),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl SummaryStatistics {
    fn accept(&mut self, value: f64) -> i64 {
        self.count += 1;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.sum += value;
        self.count
    }

    fn average(&self) -> f64 {
        if self.count > 0 {
            self.sum / (self.count as f64)
        } else {
            0.0_f64
        }
    }
}

// === Module Commands ===

fn ns_create(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_iter().skip(1);
    let key_arg = args.into_iter().next_string()?;
    let key = RedisString::create(ctx.ctx, &key_arg.to_string());
    let redis_key = ctx.open_key_writable(&key);
    let ss = SummaryStatistics::default();

    match redis_key.set_value(&REDIS_TYPE, ss) {
        Ok(_) => Ok(RedisValue::Integer(true as i64)),
        Err(_) => Ok(RedisValue::Integer(false as i64)),
    }
}

fn ns_info(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_iter().skip(1);
    let key_arg = args.into_iter().next_string()?;
    let key = RedisString::create(ctx.ctx, &key_arg.to_string());
    let redis_key = ctx.open_key(&key);

    match redis_key.get_value::<SummaryStatistics>(&REDIS_TYPE) {
        Ok(Some(ss)) => {
            let mut res: Vec<RedisValue> = Vec::with_capacity(4 * 2);
            res.push(RedisValue::SimpleStringStatic("Count"));
            res.push(RedisValue::Integer(ss.count));
            res.push(RedisValue::SimpleStringStatic("Min"));
            if ss.min == f64::MAX {
                res.push(RedisValue::SimpleStringStatic("N/A"))
            } else {
                res.push(RedisValue::Float(ss.min))
            }
            res.push(RedisValue::SimpleStringStatic("Max"));
            if ss.max == f64::MIN {
                res.push(RedisValue::SimpleStringStatic("N/A"))
            } else {
                res.push(RedisValue::Float(ss.max))
            }
            res.push(RedisValue::SimpleStringStatic("Sum"));
            res.push(RedisValue::Float(ss.sum));
            Ok(RedisValue::Array(res))
        }
        Ok(None) | Err(_) => Ok(RedisValue::Null),
    }
}

fn ns_accept(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_arg = args.next_arg()?;
    let value_arg = args.next_arg()?;
    let key = RedisString::create(ctx.ctx, &key_arg.to_string());
    let redis_key = ctx.open_key_writable(&key);

    if let Some(ss) = redis_key.get_value::<SummaryStatistics>(&REDIS_TYPE)? {
        let value = value_arg.to_string().parse::<f64>().unwrap();
        ss.accept(value);
        Ok(RedisValue::Integer(true as i64))
    } else {
        Ok(RedisValue::Integer(false as i64))
    }
}

fn ns_average(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_iter().skip(1);
    let key_arg = args.into_iter().next_string()?;
    let key = RedisString::create(ctx.ctx, &key_arg.to_string());
    let redis_key = ctx.open_key_writable(&key);

    if let Some(ss) = redis_key.get_value::<SummaryStatistics>(&REDIS_TYPE)? {
        let value = ss.average();
        Ok(RedisValue::Float(value))
    } else {
        Ok(RedisValue::Null)
    }
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
    ["ns.create", ns_create, "write", 1, 1, 1],
    ["ns.accept", ns_accept, "write", 1, 1, 1],
    ["ns.average", ns_average, "readonly", 1, 1, 1],
  ]
}
