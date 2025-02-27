use deficiente_telegram_bot::spotify::client::{Spotify, SpotifyConfig};

#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::spotify::client::{Spotify, SpotifyConfig, SpotifyKind};
    #[test]
    fn test_identify_spotify_url() {
        let config = SpotifyConfig {
            spotify_client_id: "test".to_string(),
            spotify_client_secret: "test".to_string(),
        };
        let spotify = Spotify::new(config);

        // Test cases with their expected results
        let test_cases = [
            (
                "https://open.spotify.com/track/1234567890",
                SpotifyKind::Track,
            ),
            (
                "https://open.spotify.com/album/1234567890",
                SpotifyKind::Album,
            ),
            (
                "https://open.spotify.com/artist/1234567890",
                SpotifyKind::Artist,
            ),
            (
                "https://open.spotify.com/playlist/1234567890",
                SpotifyKind::Playlist,
            ),
            // Edge cases
            ("invalid_url", SpotifyKind::Unknown),
            ("", SpotifyKind::Unknown),
            (
                "https://not-spotify.com/track/1234567890",
                SpotifyKind::Unknown,
            ),
            // URLs with query parameters
            (
                "https://open.spotify.com/track/1234567890?si=abcdef",
                SpotifyKind::Track,
            ),
            // Multiple occurrences of keywords
            (
                "https://open.spotify.com/track/1234567890/artist/456",
                SpotifyKind::Track,
            ),
        ];

        for (input, expected) in test_cases {
            let result = spotify.identify_spotify_url(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
#[test]
fn test_extract_spotify_id() {
    let config = SpotifyConfig {
        spotify_client_id: "test".to_string(),
        spotify_client_secret: "test".to_string(),
    };
    let spotify = Spotify::new(config);

    // Test cases
    let test_cases = [
        (
            "https://open.spotify.com/track/1234567890",
            Some("1234567890"),
        ),
        (
            "https://open.spotify.com/album/1234567890?si=abcdef",
            Some("1234567890"),
        ),
        (
            "https://open.spotify.com/playlist/1234567890",
            Some("1234567890"),
        ),
        (
            "https://open.spotify.com/artist/1234567890",
            Some("1234567890"),
        ),
        ("invalid_url", None),
        ("", None),
        ("https://not-spotify.com/track/1234567890", None),
        ("https://open.spotify.com/", None),
    ];

    for (input, expected) in test_cases {
        let result = spotify.extract_spotify_id(input);
        assert_eq!(result.as_deref(), expected, "Failed for input: {input}",);
    }
}
