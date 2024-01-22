use redis::Client;

pub async fn redis_connection() -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::open(std::env::var("REDIS_URL").expect("Redis URL not set"))?;
    match client.get_connection_manager().await {
        Ok(_) => {
            log::info!("✅ Connection to Redis is stablished!");
            Ok(client)
        }
        Err(err) => {
            log::error!("Redis connection error: {}", err);
            Err(err.into())
        }
    }
}
