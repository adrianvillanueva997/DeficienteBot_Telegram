use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{album::SpotifyAlbum, artist::SpotifyArtist, playlist::SpotifyPlaylist, song::SpotifySong, utils::{
    SPOTIFY_ALBUM_ENDPOINT, SPOTIFY_API_BASE, SPOTIFY_AUTH_URL, SPOTIFY_TRACKS_ENDPOINT,
}};

#[derive(Debug)]
#[allow(dead_code)]
pub enum SpotifyError {
    Authentication(String),
    Request(reqwest::Error),
    NotFound(String),
    RateLimit(String),
}

impl std::error::Error for SpotifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpotifyError::Request(e) => Some(e),
            SpotifyError::Authentication(_)
            | SpotifyError::NotFound(_)
            | SpotifyError::RateLimit(_) => None,
        }
    }
}

impl std::fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpotifyError::Authentication(msg) => write!(f, "Authentication error: {msg}"),
            SpotifyError::Request(e) => write!(f, "Request error: {e}"),
            SpotifyError::NotFound(msg) => write!(f, "Resource not found: {msg}"),
            SpotifyError::RateLimit(msg) => write!(f, "Rate limit exceeded: {msg}"),
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

    /// Returns the get cached token of this [`Spotify`].
    fn get_cached_token(&self) -> Option<SpotifyAuthResponse> {
        if self.token_cache.contains_key("spotify_token") {
            self.token_cache.get("spotify_token")
        } else {
            None
        }
    }

    /// Returns the create reqwest client of this [`Spotify`].
    ///
    /// # Panics
    ///
    /// Panics if .
    fn create_reqwest_client(&self) -> reqwest::Client {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create client")
    }

    /// Returns the get token of this [`Spotify`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    async fn get_token(&self) -> Result<SpotifyAuthResponse, SpotifyError> {
        if let Some(spotify_auth_response) = self.get_cached_token() {
            return Ok(spotify_auth_response);
        }
        let client = self.create_reqwest_client();
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
            .map_err(SpotifyError::Request)?;
        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<SpotifyAuthResponse>()
                .await
                .map_err(SpotifyError::Request),
            status => {
                let error_test = response.text().await.map_err(SpotifyError::Request)?;
                Err(SpotifyError::Authentication(format!(
                    "failed with status {status}: {error_test}",
                )))
            }
        }
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    async fn get_spotify_song(&self, song_id: String) -> Result<SpotifySong, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_TRACKS_ENDPOINT}/{song_id}");
        let token = self.get_token().await?;
        let response = self
            .create_reqwest_client()
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .await
            .map_err(SpotifyError::Request)?;

        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<SpotifySong>()
                .await
                .map_err(SpotifyError::Request),
            reqwest::StatusCode::NOT_FOUND => Err(SpotifyError::NotFound(format!(
                "Song with ID {song_id} not found"
            ))),
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(SpotifyError::RateLimit("Rate limit exceeded".to_string()))
            }
            status => {
                let error_text = response.text().await.map_err(SpotifyError::Request)?;
                Err(SpotifyError::Authentication(format!(
                    "Request failed with status {status}: {error_text}",
                )))
            }
        }
    }

    async fn get_spotify_album(&self, album_id: String) -> Result<SpotifyAlbum, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_ALBUM_ENDPOINT}/{album_id}");
        let token = self.get_token().await?;
        let response = self
            .create_reqwest_client()
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .await
            .map_err(SpotifyError::Request)?;
        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<SpotifyAlbum>()
                .await
                .map_err(SpotifyError::Request),
            reqwest::StatusCode::NOT_FOUND => Err(SpotifyError::NotFound(format!(
                "Song with ID {album_id} not found"
            ))),
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(SpotifyError::RateLimit("Rate limit exceeded".to_string()))
            }
            status => {
                let error_text = response.text().await.map_err(SpotifyError::Request)?;
                Err(SpotifyError::Authentication(format!(
                    "Request failed with status {status}: {error_text}",
                )))
            }
        }
    }
        async fn get_spotify_artist(&self, artist_id: String) -> Result<SpotifyArtist, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_ALBUM_ENDPOINT}/{artist_id}");
        let token = self.get_token().await?;
        let response = self
            .create_reqwest_client()
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .await
            .map_err(SpotifyError::Request)?;
        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<SpotifyArtist>()
                .await
                .map_err(SpotifyError::Request),
            reqwest::StatusCode::NOT_FOUND => Err(SpotifyError::NotFound(format!(
                "Song with ID {artist_id} not found"
            ))),
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(SpotifyError::RateLimit("Rate limit exceeded".to_string()))
            }
            status => {
                let error_text = response.text().await.map_err(SpotifyError::Request)?;
                Err(SpotifyError::Authentication(format!(
                    "Request failed with status {status}: {error_text}",
                )))
            }
        }
    }
        async fn get_spotify_playlist(&self, playlist_id: String) -> Result<SpotifyPlaylist, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_ALBUM_ENDPOINT}/{playlist_id}");
        let token = self.get_token().await?;
        let response = self
            .create_reqwest_client()
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .await
            .map_err(SpotifyError::Request)?;
        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<SpotifyPlaylist>()
                .await
                .map_err(SpotifyError::Request),
            reqwest::StatusCode::NOT_FOUND => Err(SpotifyError::NotFound(format!(
                "Song with ID {playlist_id} not found"
            ))),
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(SpotifyError::RateLimit("Rate limit exceeded".to_string()))
            }
            status => {
                let error_text = response.text().await.map_err(SpotifyError::Request)?;
                Err(SpotifyError::Authentication(format!(
                    "Request failed with status {status}: {error_text}",
                )))
            }
        }
    }
}
