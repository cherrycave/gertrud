use redis::FromRedisValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KeyType {
    Proxy,
    Logging,
    Server,
}

impl FromRedisValue for KeyType {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let s: String = FromRedisValue::from_redis_value(v)?;
        match s.as_str() {
            "proxy" => Ok(KeyType::Proxy),
            "logging" => Ok(KeyType::Logging),
            "server" => Ok(KeyType::Server),
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Invalid key type",
            ))),
        }
    }
}
