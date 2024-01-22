use redis::AsyncCommands;

use super::leaderboards::LEADERBOARD;

#[derive()]
struct Rank {
    pub redis_client: redis::Client,
}

impl Rank {
    fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }

    async fn update_rank(&self, command_id: String, score: i32) {
        let mut connection = self.redis_client.get_async_connection().await.unwrap();
        let _: () = connection
            .zadd(LEADERBOARD, command_id, score)
            .await
            .unwrap();
    }
    async fn get_rank(&self, command_id: String) -> Option<i32> {
        let mut connection = self.redis_client.get_async_connection().await.unwrap();
        let score: Option<i32> = connection.zscore(LEADERBOARD, command_id).await.unwrap();
        score
    }
    async fn get_ranking(&self) -> Vec<(String, i32)> {
        let mut connection = self.redis_client.get_async_connection().await.unwrap();
        let ranking: Vec<(String, i32)> = connection
            .zrange_withscores(LEADERBOARD, 0, -1)
            .await
            .unwrap();
        ranking
    }
}
