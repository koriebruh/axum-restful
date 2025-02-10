use std::env;
use dotenv::dotenv;
use redis::aio::MultiplexedConnection;
use redis::{RedisResult};

pub(crate) async fn redis_client() -> RedisResult<MultiplexedConnection> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL")
        .expect("ENV for redis not found");

    let client = redis::Client::open(redis_url).unwrap();
    let conn = client.get_multiplexed_async_connection().await?;

    Ok(conn)
}
