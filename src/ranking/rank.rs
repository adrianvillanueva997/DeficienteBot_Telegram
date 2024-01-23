use redis::AsyncCommands;
use tracing::instrument;

use super::leaderboards::LEADERBOARD;

#[derive(Debug)]
pub struct Rank {
    pub redis_client: redis::Client,
}

impl Rank {
    pub fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }

    #[instrument]
    async fn set_rank(&self, command_id: String, score: i32) {
        let mut connection = self.redis_client.get_async_connection().await.unwrap();
        let _: () = connection
            .zadd(LEADERBOARD, command_id, score)
            .await
            .unwrap();
    }
    #[instrument]
    async fn get_score(&self, command_id: String) -> Option<i32> {
        let mut connection = self.redis_client.get_async_connection().await.unwrap();
        let score: Option<i32> = connection.zscore(LEADERBOARD, command_id).await.unwrap();
        score
    }
    #[instrument]
    pub async fn get_ranking(&self) -> Vec<(String, i32)> {
        let mut connection = self.redis_client.get_async_connection().await.unwrap();
        let ranking: Vec<(String, i32)> = connection
            .zrange_withscores(LEADERBOARD, 0, -1)
            .await
            .unwrap();
        ranking
    }
    #[instrument]
    pub async fn update_rank(&self, command_id: String) {
        let score = self.get_score(command_id.clone()).await.unwrap_or(0); // If the command doesn't exist, it will be created with a score of 0.
        self.set_rank(command_id, score + 1).await;
    }
}
