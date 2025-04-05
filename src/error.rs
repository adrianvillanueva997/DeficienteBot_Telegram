use std::fmt;
use teloxide::RequestError;

#[derive(Debug)]
pub enum BotError {
    TeloxideError(RequestError),
    ProcessingError(String),
    ParseError(String),
}

impl std::error::Error for BotError {}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::TeloxideError(e) => write!(f, "Telegram API error: {}", e),
            BotError::ProcessingError(e) => write!(f, "Processing error: {}", e),
            BotError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum SpotifyError {
    ApiError(String),
    NetworkError(String),
    TelegramError(teloxide::RequestError),
    ParseError(String),
}

impl std::fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiError(msg) => write!(f, "Spotify API error: {}", msg),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::TelegramError(e) => write!(f, "Telegram error: {}", e),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for SpotifyError {}
