# RedisNumbersStats

* RedisNumbersStats is a [Redis](https://redis.io/) module that implements a Redis version of
the Java Util *SummaryStatistics classes, such as [DoubleSummaryStatistics](https://docs.oracle.com/javase/8/docs/api/java/util/DoubleSummaryStatistics.html)
* This module is useful when dealing with a stream of numbers. It maintains a state data structure for collecting statistics such as count, min, max, sum, and average.
* RedisNumbersStats is built using [redismodule-rs](https://crates.io/crates/redis-module), an idiomatic Rust API
for the [Redis Modules API](https://redis.io/docs/reference/modules/).

## Build

Make sure you have Rust installed:
https://www.rust-lang.org/tools/install

Then, build as usual:

```bash
cargo build
```

Make sure you have Redis installed.

## Run

### Linux

```
redis-server --loadmodule ./target/debug/libredis_ns.dylib
```

### Mac OS

```
redis-server --loadmodule ./target/debug/libredis_ns.dylib
```

### An Example

```bash
127.0.0.1:6379> MODULE LIST
1) 1) "name"
   2) "rns"
   3) "ver"
   4) (integer) 1
   5) "path"
   6) "./target/debug/libredis_ns.dylib"
   7) "args"
   8) (empty array)
127.0.0.1:6379> KEYS *
(empty array)
127.0.0.1:6379> NS.CREATE team1:ages
(integer) 1
127.0.0.1:6379> NS.INFO team1:ages
1) Count
2) (integer) 0
3) Min
4) N/A
5) Max
6) N/A
7) Sum
8) "0"
127.0.0.1:6379> NS.ACCEPT team1:ages 22
(integer) 1
127.0.0.1:6379> NS.ACCEPT team1:ages 19
(integer) 1
127.0.0.1:6379> NS.ACCEPT team1:ages 25
(integer) 1
127.0.0.1:6379> NS.ACCEPT team1:ages 23
(integer) 1
127.0.0.1:6379> NS.ACCEPT team1:ages 28
(integer) 1
127.0.0.1:6379> NS.INFO team1:ages
1) Count
2) (integer) 5
3) Min
4) "19"
5) Max
6) "28"
7) Sum
8) "117"
127.0.0.1:6379> NS.AVERAGE team1:ages
"23.399999999999999"
127.0.0.1:6379> NS.CREATE team2:ages
(integer) 1
127.0.0.1:6379> NS.ACCEPT team2:ages 42
(integer) 1
127.0.0.1:6379> NS.ACCEPT team2:ages 32
(integer) 1
127.0.0.1:6379> NS.ACCEPT team2:ages 18
(integer) 1
127.0.0.1:6379> NS.INFO team2:ages
1) Count
2) (integer) 3
3) Min
4) "18"
5) Max
6) "42"
7) Sum
8) "92"
127.0.0.1:6379> NS.AVERAGE team2:ages
"30.666666666666668"
127.0.0.1:6379> NS.MERGE team3:ages team1:ages team2:ages
(integer) 2
127.0.0.1:6379> NS.INFO team3:ages
1) Count
2) (integer) 8
3) Min
4) "18"
5) Max
6) "42"
7) Sum
8) "209"
127.0.0.1:6379> NS.AVERAGE team3:ages
"26.125"
127.0.0.1:6379>
```

## License

MIT