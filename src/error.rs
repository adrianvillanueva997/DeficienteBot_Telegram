use std::fmt;
use teloxide::RequestError;

#[derive(Debug)]
pub enum BotError {
    Teloxide(RequestError),
    Processing(String),
    Parse(String),
}

impl std::error::Error for BotError {}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::Teloxide(e) => write!(f, "Telegram API error: {e}"),
            BotError::Processing(e) => write!(f, "Processing error: {e}"),
            BotError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

#[derive(Debug)]
pub enum SpotifyError {
    Api(String),
    // Network(String),
    Telegram(teloxide::RequestError),
    // Parse(String),
}

impl std::fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api(msg) => write!(f, "Spotify API error: {msg}"),
            // Self::Network(msg) => write!(f, "Network error: {msg}"),
            Self::Telegram(e) => write!(f, "Telegram error: {e}"),
            // Self::Parse(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl std::error::Error for SpotifyError {}
