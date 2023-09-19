// use redis::{Commands, Connection, RedisResult};
//
// pub async fn get_str(con: &mut Connection, key: &String,field:&String) -> RedisResult<String> {
//     let value:RedisResult<String> = con.hget(key,field);
//     match value {
//         redis::Value::Nil => Ok(String::new()),
//         _ => FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into()),
//     }
//
// }