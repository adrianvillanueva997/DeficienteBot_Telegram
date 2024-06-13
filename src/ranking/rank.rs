use redis::AsyncCommands;
use tracing::instrument;

use super::leaderboards::LEADERBOARD;

#[derive(Debug)]
pub struct Rank {
    pub redis_client: redis::Client,
}

impl Rank {
    #[must_use]
    pub fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }

    #[instrument]
    async fn set_rank(&self, command_id: &str, score: i32) {
        let mut connection = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let _: () = connection
            .zadd(LEADERBOARD, command_id, score)
            .await
            .unwrap();
    }
    #[instrument]
    async fn get_score(&self, command_id: &str) -> Option<i32> {
        let mut connection = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let score: Option<i32> = connection.zscore(LEADERBOARD, command_id).await.unwrap();
        score
    }
    #[instrument]
    pub async fn get_ranking(&self) -> Result<Vec<(String, i32)>, redis::RedisError> {
        let mut connection = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let mut cmd = redis::cmd("ZRANGE");
        cmd.arg(LEADERBOARD).arg(0).arg(-1).arg("WITHSCORES");
        let mut ranking: Vec<(String, i32)> = cmd.query_async(&mut connection).await?;
        ranking.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(ranking)
    }

    #[instrument]
    pub async fn update_rank(&self, command_id: &str) {
        let score = self.get_score(command_id).await.unwrap_or(0); // If the command doesn't exist, it will be created with a score of 0.
        self.set_rank(command_id, score + 1).await;
    }
}
