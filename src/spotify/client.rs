use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, time::Duration};

use super::{
    album::SpotifyAlbum,
    artist::SpotifyArtist,
    playlist::SpotifyPlaylist,
    song::SpotifySong,
    utils::{
        SPOTIFY_ALBUM_ENDPOINT, SPOTIFY_API_BASE, SPOTIFY_ARTIST_ENDPOINT, SPOTIFY_AUTH_URL,
        SPOTIFY_PLAYLIST_ENDPOINT, SPOTIFY_TRACKS_ENDPOINT,
    },
};

pub struct SpotifyConfig {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
}

impl SpotifyConfig {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            spotify_client_id: env::var("SPOTIFY_CLIENT_ID")
                .map_err(|_| "SPOTIFY_CLIENT_ID environment variable not set")?,
            spotify_client_secret: env::var("SPOTIFY_CLIENT_SECRET")
                .map_err(|_| "SPOTIFY_CLIENT_SECRET environment variable not set")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpotifyKind {
    Album,
    Artist,
    Playlist,
    Track,
    Unknown,
}

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

#[derive(Debug)]
pub struct Spotify {
    client_id: String,
    client_secret: String,
    token_cache: Cache<String, SpotifyAuthResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct SpotifyAuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i8,
}

impl Spotify {
    /// Creates a new [`Spotify`].
    #[must_use]
    pub fn new(config: SpotifyConfig) -> Self {
        Spotify {
            client_id: config.spotify_client_id,
            client_secret: config.spotify_client_secret,
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
    fn create_reqwest_client() -> reqwest::Client {
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
        let client = Self::create_reqwest_client();
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
    pub async fn get_spotify_song(&self, song_id: String) -> Result<SpotifySong, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_TRACKS_ENDPOINT}/{song_id}");
        let token = self.get_token().await?;
        let response = Self::create_reqwest_client()
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

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn get_spotify_album(&self, album_id: String) -> Result<SpotifyAlbum, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_ALBUM_ENDPOINT}/{album_id}");
        let token = self.get_token().await?;
        let response = Self::create_reqwest_client()
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
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn get_spotify_artist(
        &self,
        artist_id: String,
    ) -> Result<SpotifyArtist, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_ARTIST_ENDPOINT}/{artist_id}");
        let token = self.get_token().await?;
        let response = Self::create_reqwest_client()
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
                "Artist with ID {artist_id} not found"
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
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn get_spotify_playlist(
        &self,
        playlist_id: String,
    ) -> Result<SpotifyPlaylist, SpotifyError> {
        let url = format!("{SPOTIFY_API_BASE}/{SPOTIFY_PLAYLIST_ENDPOINT}/{playlist_id}");
        let token = self.get_token().await?;
        let response = Self::create_reqwest_client()
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

    /// Identifies the type of Spotify URL
    ///
    /// # Arguments
    ///
    /// * `spotify_url` - A Spotify URL to identify
    ///
    /// # Returns
    ///
    /// Returns a `SpotifyKind` enum indicating the type of content
    ///
    /// # Examples
    ///
    /// ```
    /// let kind = spotify.identify_spotify_url("https://open.spotify.com/track/123");
    /// assert_eq!(kind, SpotifyKind::Track);
    /// ```
    #[must_use]
    pub fn identify_spotify_url(&self, spotify_url: &str) -> SpotifyKind {
        // First verify it's a valid Spotify URL
        if !spotify_url.starts_with("https://open.spotify.com/") {
            return SpotifyKind::Unknown;
        }

        match spotify_url {
            url if url.contains("/album/") => SpotifyKind::Album,
            url if url.contains("/track/") => SpotifyKind::Track,
            url if url.contains("/artist/") => SpotifyKind::Artist,
            url if url.contains("/playlist/") => SpotifyKind::Playlist,
            _ => SpotifyKind::Unknown,
        }
    }

    /// Extracts the Spotify ID from a URL
    ///
    /// # Arguments
    ///
    /// * `url` - A Spotify URL (e.g., "<https://open.spotify.com/track/1234567890>")
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` with the ID if found, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// let spotify = Spotify::new(config);
    /// let id = spotify.extract_spotify_id("https://open.spotify.com/track/1234567890");
    /// assert_eq!(id, Some("1234567890".to_string()));
    /// ```
    pub fn extract_spotify_id(&self, url: &str) -> Option<String> {
        // First verify it's a valid Spotify URL
        if !url.starts_with("https://open.spotify.com/") {
            return None;
        }

        // Split URL into parts and get the ID
        let id = url
            .split('/')
            .last()
            .and_then(|id| id.split('?').next())
            .filter(|&id| !id.is_empty()) // Filter out empty strings
            .map(String::from);

        // Additional validation: ensure we have a non-empty ID
        if let Some(ref id) = id {
            if id.trim().is_empty() {
                return None;
            }
        }

        id
    }
}
