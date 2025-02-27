use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug)]
#[allow(dead_code)]
pub enum SpotifyError {
    AuthenticationError(String),
    RequestError(reqwest::Error),
}

impl std::error::Error for SpotifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpotifyError::RequestError(e) => Some(e),
            SpotifyError::AuthenticationError(_) => None,
        }
    }
}

impl std::fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpotifyError::AuthenticationError(msg) => write!(f, "Authentication error: {msg}"),
            SpotifyError::RequestError(e) => write!(f, "Request error: {e}"),
        }
    }
}

#[derive()]
pub struct Spotify {
    client_id: String,
    client_secret: String,
    token_cache: Cache<String, SpotifyAuthResponse>,
}

#[derive(Deserialize, Serialize, Clone)]
struct SpotifyAuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i8,
}

const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/api/token";

impl Spotify {
    /// Creates a new [`Spotify`].
    pub fn new(client_id: String, client_secret: String) -> Self {
        Spotify {
            client_id,
            client_secret,
            token_cache: Cache::builder()
                .max_capacity(1)
                .time_to_live(Duration::from_secs(3600)) // 1 hour TTL
                .build(),
        }
    }

    fn get_cached_token(&self) -> Option<SpotifyAuthResponse> {
        if self.token_cache.contains_key("spotify_token") {
            self.token_cache.get("spotify_token")
        } else {
            None
        }
    }

    /// Returns the get token of this [`Spotify`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    async fn get_token(&self) -> Result<SpotifyAuthResponse, SpotifyError> {
        if let Some(token) = self.get_cached_token() {
            return Ok(token);
        }
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];
        let response = client
            .post(SPOTIFY_AUTH_URL)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&params)
            .send()
            .await
            .map_err(SpotifyError::RequestError)?;
        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<SpotifyAuthResponse>()
                .await
                .map_err(SpotifyError::RequestError),
            status => {
                let error_test = response.text().await.map_err(SpotifyError::RequestError)?;
                Err(SpotifyError::AuthenticationError(format!(
                    "failed with status {status}: {error_test}",
                )))
            }
        }
    }
}
