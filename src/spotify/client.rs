use rspotify::{ClientCredsSpotify, Credentials};




#[derive(Debug, Clone, PartialEq)]
pub enum SpotifyKind {
    Album,
    Artist,
    Playlist,
    Track,
    Unknown,
}



#[derive(Debug)]
pub struct Spotify {
    pub client : ClientCredsSpotify
}

impl Spotify {
    /// Loads Spotify credentials from environment variables
    fn load_credentials() -> Credentials {
        Credentials::from_env().expect("Failed to load Spotify credentials from environment")
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn new() -> Result<Self, rspotify::ClientError> {
        let credentials = Self::load_credentials();
        let spotify = Self::create_token(credentials).await?;
        Ok(Spotify {
            client: spotify
        })
    }

    async fn create_token(credentials:Credentials) -> Result<ClientCredsSpotify, rspotify::ClientError> {
        let spotify = ClientCredsSpotify::new(credentials);
        spotify.request_token().await?;
        Ok(spotify)
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
