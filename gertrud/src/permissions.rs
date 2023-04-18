use bitflags::bitflags;
use redis::FromRedisValue;

bitflags! {
    pub struct Permissions: u64 {
        const MANAGE_KEYS = 1 << 0;
        const READ_SEND = 1 << 1;
        const WRITE_SEND = 1 << 2;

        const ADMIN = Self::MANAGE_KEYS.bits() | Self::READ_SEND.bits() | Self::WRITE_SEND.bits();
    }
}

impl FromRedisValue for Permissions {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let value = u64::from_redis_value(v)?;
        Ok(Permissions::from_bits_truncate(value))
    }
}
